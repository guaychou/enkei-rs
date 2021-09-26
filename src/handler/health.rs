use crate::db::DbInstance;
use crate::error::AppError;
use axum::{extract::Extension, Json};
use serde_json::{json, Value};
use tracing::instrument;

#[instrument(name = "health_handler", skip(db_instance))]
pub async fn health_checkz(
    Extension(db_instance): Extension<DbInstance>,
) -> Result<Json<Value>, AppError> {
    let data = db_instance.client.list_database_names(None, None).await?;
    Ok(json!({ "data": data }).into())
}
