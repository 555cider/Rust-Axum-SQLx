use axum::{http, response::IntoResponse, Json};
use serde::Serialize;
use serde_json::json;

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR,
}

#[derive(Debug, Serialize)]
pub enum CustomError {
    BadRequest,
    TaskNotFound,
    InternalServerError,
}

impl IntoResponse for CustomError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            Self::InternalServerError => (
                http::StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error",
            ),
            Self::BadRequest => (http::StatusCode::BAD_REQUEST, "Bad Request"),
            Self::TaskNotFound => (http::StatusCode::NOT_FOUND, "Task Not Found"),
        };
        (status, Json(json!({ "error": error_message }))).into_response()
    }
}
