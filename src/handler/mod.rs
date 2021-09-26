pub mod health;
pub mod servicemap;
use getset::Getters;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
pub struct ServiceModel {
    #[getset(get = "pub with_prefix")]
    label: String,
    #[getset(get = "pub with_prefix")]
    gcp_name: String,
    #[getset(get = "pub with_prefix")]
    namespace: String,
}
