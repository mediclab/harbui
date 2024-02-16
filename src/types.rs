use serde::{Deserialize, Serialize};

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
    pub image_manifests: Vec<ImageManifest>
}