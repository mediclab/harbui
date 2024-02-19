use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "mediaType")]
pub enum Manifest {
    #[serde(rename = "application/vnd.oci.image.index.v1+json")]
    OCIImageIndexV1(OCIImageIndexV1),
    #[serde(rename = "application/vnd.oci.image.manifest.v1+json")]
    OCIImageManifestV1(OCIImageManifestV1),
    #[serde(rename = "application/vnd.docker.distribution.manifest.v2+json")]
    DockerDistributionManifestV2(DockerDistributionManifestV2),
    #[serde(rename = "application/vnd.docker.distribution.manifest.list.v2+json")]
    DockerDistributionManifestListV2(DockerDistributionManifestListV2),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ManifestResponse {
    pub manifest: Manifest,
    pub digest: String,
    pub reference: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OCIImageIndexV1 {
    #[serde(rename = "schemaVersion")]
    pub schema_version: i8,
    pub manifests: Vec<OCIImageManifestV1Short>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DockerDistributionManifestListV2 {
    #[serde(rename = "schemaVersion")]
    pub schema_version: i8,
    pub manifests: Vec<OCIImageManifestV1Short>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OCIImageConfigV1 {
    pub digest: String,
    pub size: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DockerDistributionManifestV2 {
    #[serde(rename = "schemaVersion")]
    pub schema_version: i8,
    pub config: OCIImageConfigV1,
    pub layers: Vec<Layer>,
}

impl DockerDistributionManifestV2 {
    pub fn get_total_size(&self) -> u64 {
        self.layers.iter().map(|i| i.size).sum()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DockerContainerImageV1 {
    pub digest: String,
    pub size: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OCIImageManifestV1Short {
    pub digest: String,
    pub size: u64,
    pub annotations: Option<HashMap<String, String>>,
    pub platform: Platform,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OCIImageManifestV1 {
    #[serde(rename = "schemaVersion")]
    pub schema_version: i8,
    pub config: OCIImageConfigV1,
    pub layers: Vec<Layer>,
}

impl OCIImageManifestV1 {
    pub fn get_total_size(&self) -> u64 {
        self.layers.iter().map(|i| i.size).sum()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Layer {
    #[serde(rename = "mediaType")]
    pub media_type: String,
    pub size: u64,
    pub digest: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CatalogResponse {
    pub repositories: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TagsResponse {
    pub name: String,
    pub tags: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Platform {
    pub architecture: String,
    pub os: String,
    #[serde(rename = "os.version")]
    pub os_version: Option<String>,
    #[serde(rename = "os.features")]
    pub os_features: Option<Vec<String>>,
    pub variant: Option<String>,
    pub features: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MediaType {
    #[serde(rename = "application/vnd.oci.image.index.v1+json")]
    OCIImageIndexV1,
    #[serde(rename = "application/vnd.oci.image.manifest.v1+json")]
    OCIImageManifestV1,
    #[serde(rename = "application/vnd.oci.image.config.v1+json")]
    OCIImageConfigV1,
    #[serde(rename = "application/vnd.docker.distribution.manifest.v2+json")]
    DockerDistributionManifestV2,
    #[serde(rename = "application/vnd.docker.distribution.manifest.list.v2+json")]
    DockerDistributionManifestListV2,
    #[serde(rename = "application/vnd.docker.container.image.v1+json")]
    DockerContainerImageV1,
}

impl Display for MediaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            MediaType::OCIImageIndexV1 => String::from("application/vnd.oci.image.index.v1+json"),
            MediaType::OCIImageManifestV1 => String::from("application/vnd.oci.image.manifest.v1+json"),
            MediaType::OCIImageConfigV1 => String::from("application/vnd.oci.image.config.v1+json"),
            MediaType::DockerDistributionManifestV2 => {
                String::from("application/vnd.docker.distribution.manifest.v2+json")
            }
            MediaType::DockerDistributionManifestListV2 => {
                String::from("application/vnd.docker.distribution.manifest.list.v2+json")
            }
            MediaType::DockerContainerImageV1 => String::from("application/vnd.docker.container.image.v1+json"),
        };
        write!(f, "{}", str)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ImageConfigResponse {
    pub architecture: String,
    pub os: String,
    pub author: Option<String>,
    pub config: ImageConfig,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ImageConfig {
    #[serde(rename = "Env")]
    pub env: Option<Vec<String>>,
    #[serde(rename = "Cmd")]
    pub cmd: Option<Vec<String>>,
    #[serde(rename = "Labels")]
    pub labels: Option<HashMap<String, String>>,
}
