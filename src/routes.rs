use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use crate::state::AppState;

pub const USERS_TAG: &str = "users";
const USERS_TAG_DESCRIPTION: &str = "Routes relating to user lookup and management";

pub const ROUTE_TAG: &str = "misc-routes";
const ROUTE_TAG_DESCRIPTION: &str = "lorem ipsum dolor sit amet";
#[derive(OpenApi)]
#[openapi(
    tags(
    (name = USERS_TAG, description = USERS_TAG_DESCRIPTION ),
    (name = ROUTE_TAG, description = ROUTE_TAG_DESCRIPTION )
    )
)]
pub struct ApiDoc;

pub fn register_routes(state: AppState) -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(health))
        .with_state(state)
}

#[utoipa::path(
    method(get, head),
    path = "/api/health",
    tag = ROUTE_TAG,
    responses(
        (status = OK, description = "Success", body = str, content_type = "text/plain")
    )
)]
pub async fn health() -> &'static str {
    "ok"
}