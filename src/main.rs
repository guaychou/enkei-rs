use enkei::{
    cli, configuration,
    error::AppError,
    log, application,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let cli = cli::Options::new();
    log::log_init();
    let config = configuration::read_config(cli.get_config_path());
    let app = application::build(config.clone())
        .await?;
    let addr = SocketAddr::from(([0, 0, 0, 0], *config.server.get_port()));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(application::shutdown_signal()).await?;
    Ok(())
}
