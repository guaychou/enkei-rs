use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Enkei, Ml Automated platform autoscaler !")]
pub struct Options {
    /// Enkei config path
    #[structopt(long = "config.enkei", default_value = "config/enkei.yaml")]
    config: String,
}

impl Options {
    pub fn new() -> Self {
        Options::from_args()
    }

    pub fn get_config_path(&self) -> &str {
        self.config.as_str()
    }
}
