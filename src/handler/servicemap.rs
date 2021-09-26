use super::ServiceModel;
use crate::db::DbInstance;
use crate::error::AppError;
use axum::{extract::Extension, Json};
use futures::stream::TryStreamExt;
use serde_json::{json, Value};
use tracing::instrument;

// Inserting the service map
#[instrument(name = "service_map_handler", skip(db_instance))]
pub async fn add_service_map(
    Json(req): Json<ServiceModel>,
    Extension(db_instance): Extension<DbInstance>,
) -> Result<Json<Value>, AppError> {
    if db_instance.is_service_not_exist(&req).await? {
        let res = db_instance.add_service(&req).await?;
        Ok(json!(res).into())
    } else {
        Ok(json!({"message": "Data already exist"}).into())
    }
}

pub async fn get_all_service_map(
    Extension(db_instance): Extension<DbInstance>,
) -> Result<Json<Value>, AppError> {
    let res = db_instance.collection.find(None, None).await?;
    let data: Vec<ServiceModel> = res.try_collect().await?;
    Ok(json!(data).into())
}
