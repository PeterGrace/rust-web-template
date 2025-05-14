use std::error::Error;
use async_trait::async_trait;
use sqlx::PgPool;
use utoipa::ToSchema;
use crate::modules::users::User;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use axum::response::{IntoResponse, Response};
use axum::Json;
use axum::http::StatusCode;

pub mod users;

#[derive(PartialEq)]
pub enum AuthorizableType{
    User(User),
}

#[derive(PartialEq)]
pub enum RBAC{
    Read,
    Write,
    Delete,
    Admin
}

#[async_trait]
pub trait Authorizable {
    async fn check_authorization<'a>(
        pool: &'a Option<PgPool>,
        id: &'a AuthorizableType,
        user: &'a User,
        rbac: &'a RBAC
    ) -> Result<bool, Box<dyn Error + Send + Sync>>;
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub(crate) struct AppAPIResponse {
    /// A string message explaining the response
    message: String,
    /// optional json-formatted data related to the response
    data: Option<Value>
}
impl AppAPIResponse {
    fn message<S: Into<String>>(msg: S) -> Self {
        Self {
            message: msg.into(),
            data: None,
        }
    }
    fn data<S: Into<String>,D: Into<Value>>(msg: S, data: D) -> Self {
        Self {
            message: msg.into(),
            data: Some(data.into()),
        }
    }

}
impl IntoResponse for AppAPIResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}
