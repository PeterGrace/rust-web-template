//region general
pub const PREFIX_NAMESPACE: &str = "{{project-name}}";
pub const JWT_SECRET: &str = "secret";
//endregion

//region connectivity
pub const MAX_DB_CONNECTIONS: u32 = 5;
pub const DB_CONNECT_TIMEOUT: u64 = 5;
pub const SESSION_INACTIVITY_LIMIT_HOURS: i64 = 24;
//endregion

//region paths
pub const API_VER: &str = "/api/v1";
pub const SCALAR_PATH: &str = "/api/scalar";
pub const API_PATH: &str = "/api/openapi.json";
pub const HEALTH_PATH: &str = "/api/health";
//endregion


//region route tags and descriptions
pub const USERS_TAG: &str = "users";
pub const USERS_TAG_DESCRIPTION: &str = "Routes relating to user lookup and management";
pub const ROUTE_TAG: &str = "misc-routes";
pub const ROUTE_TAG_DESCRIPTION: &str = "lorem ipsum dolor sit amet";
//endregion
