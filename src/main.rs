pub mod metrics_data;
pub mod routes;
pub mod state;
pub mod auth;
pub mod consts;
mod modules;

#[macro_use]
extern crate tracing;
#[macro_use]
extern crate metrics;

use console_subscriber as tokio_console_subscriber;
use axum::routing::get;
use metrics::Recorder;
use metrics_exporter_prometheus::PrometheusBuilder;
use metrics_util::layers::{Prefix, Stack};
use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use std::sync::Arc;
use std::time::Duration;
use axum::http::Method;
use axum::middleware;
use cached::AsyncRedisCache;
use tokio::net::TcpListener;
use tokio::sync::RwLock;
use tokio::task;
use tokio::task::{JoinHandle, JoinSet};
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::{EnvFilter, Registry, prelude::*};
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use utoipa_scalar::{Scalar, Servable};
use metrics_data::*;
use routes::*;
use consts::*;
use tower_http::cors::{Any, CorsLayer};
use crate::state::AppState;
use dotenv::dotenv;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};
use crate::auth::token_extractor::JwksCache;
use tower_sessions::cookie::time::Duration as CookieDuration;
use crate::auth::token_middleware::auth_middleware;
use crate::modules::users::user_routes;
use tokio::sync::OnceCell;
use lazy_static::lazy_static;

lazy_static!{
    pub static ref API_DOC: OnceCell<utoipa::openapi::OpenApi> = OnceCell::new();
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenv::dotenv();

    //region console logging
    let console_layer = tokio_console_subscriber::spawn();
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("warn"))
        .unwrap();
    let format_layer = tracing_subscriber::fmt::layer()
        .event_format(
            tracing_subscriber::fmt::format()
                .with_file(true)
                .with_thread_ids(true)
                .with_thread_names(true)
                .with_line_number(true),
        )
        .with_span_events(FmtSpan::NONE);


    let subscriber = Registry::default()
        .with(console_layer)
        .with(filter_layer)
        .with(format_layer);
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set tracing subscriber");
    //endregion

    //region metrics
    let metrics_addr: SocketAddr= SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), 8081).into();
    let (recorder, exporter) = PrometheusBuilder::new()
        .with_http_listener(metrics_addr)
        .build()
        .expect("Couldn't build Prometheus exporter");

    let j = task::Builder::new()
        .name("metrics-recorder".into())
        .spawn(async move {
            if let Err(e) = exporter.await {
                error!("Error exporting metrics: {e:#?}");
                return false;
            };
            true
        })
        .expect("Couldn't spawn Prometheus exporter");
    let mut joinhandles: Vec<JoinHandle<bool>> = vec![];
    joinhandles.push(j);

    let layered_recorder = Stack::new(recorder)
        .push(metrics_util::layers::PrefixLayer::new(PREFIX_NAMESPACE))
        .install();

    register_metrics();
    counter!(METRIC_APP_INFO,
    "version" => env!("CARGO_PKG_VERSION"),
    "git_hash" =>  env!("GIT_HASH"),
    )
    .increment(1);
    info!("Initialized metrics, app starting");
    //endregion

//region database initialization
    //TODO: Implement database connectivity
    let pool = None;
    // let db_connection_str = std::env::var("DATABASE_URL").expect("Need DATABASE_URL");
    //
    // let pool = PgPoolOptions::new()
    //     .max_connections(MAX_DB_CONNECTIONS)
    //     .acquire_timeout(Duration::from_secs(DB_CONNECT_TIMEOUT))
    //     .connect(&db_connection_str)
    //     .await
    //     .expect("can't connect to database");
    //
    // sqlx::migrate!("./migrations")
    //     .run(&pool)
    //     .await
    //     .expect("Migrations failed");
//endregion

    // TODO: implement caching
    let user_cache = None;
    
    // let user_cache= Arc::new(RwLock::new(
    //     AsyncRedisCache::new("user_cache", 60)
    // .set_refresh(true)
    //     .build()
    //     .await
    //     .expect("Couldn't create cache"),
    // ));


    let state = AppState {
        pool,
        jwks_cache: JwksCache::new(),
        user_cache
    };
    //region axum route setup and serve()
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(CookieDuration::hours(
            SESSION_INACTIVITY_LIMIT_HOURS,
        )));

    let cors_layer = CorsLayer::new()
        .allow_methods([Method::OPTIONS, Method::GET, Method::PUT, Method::POST, Method::DELETE])
        .allow_origin(Any)
        .allow_headers(Any);

    let auth_layer = middleware::from_fn_with_state(
        state.clone(),
        auth_middleware,
    );

    let api = ApiDoc::openapi();

    let public_routes = OpenApiRouter::new()
        .merge(register_routes(state.clone()))
        .route(API_PATH, get(openapi));

    let protected_routes = OpenApiRouter::<AppState>::new()
        .nest(&format!("{API_VER}/{USERS_TAG}"),user_routes(state.clone()))
        .layer(auth_layer);

    let (router, api) = OpenApiRouter::with_openapi(api)
        .merge(public_routes)
        .merge(protected_routes)
        .layer(cors_layer)
        .layer(session_layer)
        .with_state(state)
        .split_for_parts();

    let app = router.merge(Scalar::with_url(SCALAR_PATH, api.clone()));
    if let Err(e) = API_DOC.set(api) {
        error!("Couldn't store api into document variable: {e}");
    }

    let listener = TcpListener::bind((Ipv6Addr::LOCALHOST, 8080))
        .await
        .expect("Failed to bind");
    let _ = axum::serve(listener, app).await;
    //endregion
    Ok(())
}
