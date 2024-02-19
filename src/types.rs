use envconfig::Envconfig;
use serde::{Deserialize, Serialize};

#[derive(Envconfig, Clone)]
pub struct Config {
    #[envconfig(from = "REGISTRY_HOST")]
    pub host: String,
    #[envconfig(from = "REGISTRY_UNSECURED", default = "false")]
    pub unsecured: bool,
    #[envconfig(from = "REGISTRY_HTTP_BASIC_USER")]
    pub http_basic_user: Option<String>,
    #[envconfig(from = "REGISTRY_HTTP_BASIC_PASSWORD")]
    pub http_basic_pass: Option<String>,
    #[envconfig(from = "HARBUI_DELETING_ALLOWED", default = "false")]
    pub deleting_allowed: bool,
    #[envconfig(from = "HARBUI_VERSION", default = "dev")]
    pub version: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ImageTags {
    pub image: String,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ImageManifest {
    pub digest: String,
    pub author: String,
    pub total_size: u64,
    pub os: String,
    pub architecture: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Manifests {
    pub tag: String,
    pub image_manifests: Vec<ImageManifest>,
}
