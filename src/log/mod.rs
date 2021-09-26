use tracing_subscriber::EnvFilter;
use {env, figlet_rs::FIGfont};

pub fn log_init() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "enkei=debug,tower_http=debug")
    }
    tracing_subscriber::fmt()
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(true)
        .compact()
        .init();
    print_banner();
}

fn print_banner() {
    let standard_font = FIGfont::standand().unwrap();
    let figure = standard_font.convert(env!("CARGO_PKG_NAME")).unwrap();
    println!("{}", figure);
    tracing::info!(
        "Starting {} version: {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    )
}
