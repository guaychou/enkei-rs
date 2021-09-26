use std::convert::Infallible;

use axum::{
    body::{Bytes, Full},
    http::Response,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use hyper::Error as HyperError;
use mongodb::error::Error as DbError;
use serde_json::{json, Value};
use thiserror::Error;
use tower::BoxError;
use validator::ValidationErrors;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Validation data error ")]
    Validation(#[from] ValidationErrors),
    #[error("Database error {0}")]
    DbError(#[from] DbError),
    #[error("Starting Server error")]
    AxumError(#[from] HyperError),
}

impl IntoResponse for AppError {
    type Body = Full<Bytes>;
    type BodyError = Infallible;

    fn into_response(self) -> Response<Self::Body> {
        let (status, error_message) = match self {
            AppError::Validation(e) => (StatusCode::BAD_REQUEST, json!(e)),
            AppError::DbError(e) => handle_db_error(&e),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!("Unhandled Internal server error"),
            ),
        };
        let body = Json(json!({
            "code" : status.as_u16(),
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

pub fn handle_error(error: BoxError) -> Result<impl IntoResponse, Infallible> {
    if error.is::<tower::timeout::error::Elapsed>() {
        return Ok((
            StatusCode::REQUEST_TIMEOUT,
            Json(json!({
                "code" : 408,
                "error" : "Uhh ohh, request time out",
            })),
        ));
    };
    if error.is::<tower::load_shed::error::Overloaded>() {
        return Ok((
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({
                "code" : 503,
                "error" : "Uhh ohh, service unavailable",
            })),
        ));
    }

    Ok((
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({
            "code" : 500,
            "error" : "Uhh ohh, unhandled internal error",
        })),
    ))
}

fn handle_db_error(e: &DbError) -> (StatusCode, Value) {
    // will work on this later hehe
    tracing::error!("Database error : {}", e);
    match e {
        _ => (StatusCode::INTERNAL_SERVER_ERROR, json!(e.to_string())),
    }
}
