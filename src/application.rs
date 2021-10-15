use crate::configuration::Config;
use crate::db::DbInstance;
use crate::error::{AppError,handle_error};
use crate::handler::{health, servicemap};
use axum::AddExtensionLayer;
use axum::{
    handler::{get, post},
    routing::{BoxRoute},
    Router,
};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

pub async fn build(config: Config) -> Result<Router<BoxRoute>, AppError> {

    let db = DbInstance::db_init(config.dbconfig).await?;
    let middleware = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .load_shed()
        .buffer(*config.server.get_buffer())
        .concurrency_limit(*config.server.get_concurrency_limit())
        .timeout(*config.server.get_timeout())
        .rate_limit(
            *config.server.get_rate_limit(),
            *config.server.get_limiter_timeout(),
        )
        .into_inner();
    
    let router = Router::new()
        .route("/health", get(health::health_checkz))
        .route(
            "/servicemap",
            post(servicemap::add_service_map).get(servicemap::get_all_service_map),
        )
        .layer(AddExtensionLayer::new(db.clone()))
        .layer(middleware)
        .handle_error(handle_error)
        .boxed();
    Ok(router)
}

pub async fn shutdown_signal() {
    use std::io;
    use tokio::signal::unix::SignalKind;
    async fn terminate() -> io::Result<()> {
        tokio::signal::unix::signal(SignalKind::terminate())?
            .recv()
            .await;
        Ok(())
    }

    tokio::select! {
        _ = terminate() => {},
        _ = tokio::signal::ctrl_c() => {},
    }
    tracing::info!("signal received, starting graceful shutdown")
}