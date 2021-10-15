use std::time::Duration;
use tracing::info;
use {getset::Getters, serde::Deserialize};

#[derive(Deserialize, Getters, Clone)]
#[serde(default = "default_database_config")]
pub struct DatabaseConfig {
    #[getset(get = "pub with_prefix")]
    mongouri: String,
    #[getset(get = "pub with_prefix")]
    db_name: String,
    #[getset(get = "pub with_prefix")]
    collection_name: String,
    #[getset(get = "pub with_prefix")]
    min_pool_size: u32,
    #[getset(get = "pub with_prefix")]
    max_pool_size: u32,
}

#[derive(Clone, Deserialize, Getters)]
#[serde(default = "default_server_config")]
pub struct ServerConfig {
    #[getset(get = "pub with_prefix")]
    port: u16,
    #[getset(get = "pub with_prefix")]
    buffer: usize,
    #[getset(get = "pub with_prefix")]
    concurrency_limit: usize,
    #[getset(get = "pub with_prefix")]
    rate_limit: u64,
    #[getset(get = "pub with_prefix")]
    #[serde(with = "humantime_serde")]
    limiter_timeout: Duration,
    #[getset(get = "pub with_prefix")]
    #[serde(with = "humantime_serde")]
    timeout: Duration,
}

#[derive(Clone, Deserialize, Getters)]
pub struct Config {
    pub dbconfig: DatabaseConfig,
    #[serde(default = "default_server_config")]
    pub server: ServerConfig,
}

fn default_server_config() -> ServerConfig {
    ServerConfig {
        port: 8080,
        buffer: 10,
        concurrency_limit: 5,
        rate_limit: 5,
        limiter_timeout: Duration::from_secs(10),
        timeout: Duration::from_secs(10),
    }
}

fn default_database_config() -> DatabaseConfig {
    DatabaseConfig {
        collection_name: String::from("test"),
        db_name: String::from("test"),
        max_pool_size: 100,
        min_pool_size: 10,
        mongouri: String::from("mongodb://root:example@localhost:27017"),
    }
}

pub fn read_config(config_path: &str) -> Config {
    let f = std::fs::File::open(config_path).expect("Couldn't found file");
    let config: Config = serde_yaml::from_reader(f).expect("Parse failed");
    info!("Config has been read from config in : {}", config_path);
    config
}
