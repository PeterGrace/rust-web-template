use sqlx::PgPool;
use cached::AsyncRedisCache;
use tokio::sync::RwLock;
use std::sync::Arc;
use crate::auth::token_extractor::JwksCache;
use crate::modules::users::User;

#[derive(Clone)]
pub(crate) struct AppState {
    pub(crate) pool: Option<PgPool>,
    pub(crate) jwks_cache: JwksCache,
    pub(crate) user_cache: Option<Arc<RwLock<AsyncRedisCache<String, User>>>>
}
