use crate::configuration::Config;
use crate::db::DbInstance;
use crate::error::AppError;
use crate::handler::{health, servicemap};
use axum::AddExtensionLayer;
use axum::{
    handler::{get, post},
    routing::BoxRoute,
    Router,
};

pub async fn build(config: Config) -> Result<Router<BoxRoute>, AppError> {
    let db = DbInstance::db_init(config.dbconfig).await?;
    let router = Router::new()
        .route("/health", get(health::health_checkz))
        .route(
            "/servicemap",
            post(servicemap::add_service_map).get(servicemap::get_all_service_map),
        )
        .layer(AddExtensionLayer::new(db.clone()))
        .boxed();
    Ok(router)
}
