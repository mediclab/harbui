use crate::registry_api::types::*;
use reqwest::header::ACCEPT;
use reqwest::RequestBuilder;
use serde::de::DeserializeOwned;

pub mod types;

#[derive(Clone, Debug)]
pub struct Config {
    pub base_uri: String,
    pub is_secured: bool,
    pub http_basic_user: Option<String>,
    pub http_basic_pass: Option<String>,
}

#[derive(Clone, Debug)]
pub struct RegistryClient {
    client: reqwest::Client,
    url: String,
    basic_auth: Option<BasicAuth>,
}

#[derive(Clone, Debug)]
struct BasicAuth {
    pub http_basic_user: String,
    pub http_basic_pass: Option<String>,
}

impl RegistryClient {
    pub fn new(config: &Config) -> Self {
        let client = reqwest::Client::builder().build().unwrap();
        let url = if config.is_secured {
            format!("https://{}", config.base_uri)
        } else {
            format!("http://{}", config.base_uri)
        };

        let basic_auth = if config.http_basic_user.is_some() {
            Some(BasicAuth {
                http_basic_user: config.http_basic_user.clone().unwrap(),
                http_basic_pass: config.http_basic_pass.clone(),
            })
        } else {
            None
        };

        Self {
            client,
            url,
            basic_auth,
        }
    }

    pub async fn get_catalog(&self) -> RegistryResponse<CatalogResponse> {
        let request = self.client.get(format!("{}/v2/_catalog", self.url));

        self.send::<CatalogResponse>(request).await
    }

    pub async fn get_tags(&self, image: &str) -> RegistryResponse<TagsResponse> {
        let request = self.client.get(format!("{}/v2/{}/tags/list", self.url, image));

        self.send::<TagsResponse>(request).await
    }

    pub async fn get_manifest(&self, name: &str, reference: &str) -> RegistryResponse<Manifest> {
        let request = self
            .client
            .get(format!("{}/v2/{}/manifests/{}", self.url, name, reference))
            .header(
                ACCEPT,
                [
                    MediaType::OCIImageIndexV1.to_string(),
                    MediaType::OCIImageManifestV1.to_string(),
                    MediaType::DockerDistributionManifestV2.to_string(),
                    MediaType::DockerDistributionManifestListV2.to_string(),
                ]
                .join(", "),
            );

        self.send::<Manifest>(request).await
    }

    pub async fn delete_manifest(&self, name: &str, reference: &str) -> RegistryResponse<()> {
        let request = self
            .client
            .delete(format!("{}/v2/{}/manifests/{}", self.url, name, reference))
            .header(
                ACCEPT,
                [
                    MediaType::OCIImageIndexV1.to_string(),
                    MediaType::OCIImageManifestV1.to_string(),
                    MediaType::DockerDistributionManifestV2.to_string(),
                    MediaType::DockerDistributionManifestListV2.to_string(),
                ]
                .join(", "),
            );

        self.send::<()>(request).await
    }

    pub async fn get_config(&self, name: &str, digest: &str) -> RegistryResponse<ImageConfigResponse> {
        let request = self.client.get(format!("{}/v2/{}/blobs/{}", self.url, name, digest));

        self.send::<ImageConfigResponse>(request).await
    }

    async fn send<T>(&self, request: RequestBuilder) -> RegistryResponse<T>
    where
        T: DeserializeOwned,
    {
        let mut req = request;
        if let Some(basic_auth) = self.basic_auth.clone() {
            req = req.basic_auth(basic_auth.http_basic_user, basic_auth.http_basic_pass);
        }

        match req.send().await {
            Ok(res) => {
                let digest = res
                    .headers()
                    .get("docker-content-digest")
                    .map(|h| String::from(h.to_str().unwrap_or_default()));

                if res.status().is_success() {
                    let status = res.status().as_u16();
                    match res.json::<T>().await {
                        Ok(content) => Ok(RegistryAnswer::new(status, content, digest)),
                        Err(e) => {
                            error!("Can't parse response: {:?}", e);
                            Err(RegistryErrors::custom("Parse error"))
                        }
                    }
                } else if res.status().is_client_error() {
                    match res.json::<RegistryErrors>().await {
                        Ok(content) => Err(content),
                        Err(e) => {
                            error!("Can't parse error response: {:?}", e);
                            Err(RegistryErrors::custom("Parse error"))
                        }
                    }
                } else if res.status().is_server_error() {
                    error!(
                        "Server error: {:?}. Server answer: {:?}",
                        res.status(),
                        res.text().await
                    );
                    Err(RegistryErrors::custom("Server error"))
                } else {
                    error!(
                        "Unknown error: {:?}. Server answer: {:?}",
                        res.status(),
                        res.text().await
                    );
                    Err(RegistryErrors::custom("Unknown error"))
                }
            }
            Err(e) => {
                error!("Request error occurred {:?}", e);
                Err(RegistryErrors::custom("Unknown error"))
            }
        }
    }
}
