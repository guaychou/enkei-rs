use enkei::{
    cli, configuration,
    error::{handle_error, AppError},
    log, startup,
};
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let cli = cli::Options::new();
    log::log_init();
    let config = configuration::read_config(cli.get_config_path());
    let app = startup::build(config.clone())
        .await?
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .load_shed()
                .buffer(*config.server.get_buffer())
                .concurrency_limit(*config.server.get_concurrency_limit())
                .timeout(*config.server.get_timeout())
                .rate_limit(
                    *config.server.get_rate_limit(),
                    *config.server.get_limiter_timeout(),
                )
                .into_inner(),
        )
        .handle_error(handle_error)
        .check_infallible();
    let addr = SocketAddr::from(([0, 0, 0, 0], *config.server.get_port()));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
