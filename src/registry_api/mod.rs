use crate::registry_api::types::*;
use anyhow::{bail, Result};
use reqwest::header::ACCEPT;
use reqwest::{RequestBuilder, StatusCode};
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

    pub async fn get_catalog(&self) -> Result<CatalogResponse> {
        let request = self.client.get(format!("{}/v2/_catalog", self.url));
        let ans = self.send::<CatalogResponse>(request).await?;

        Ok(ans.0)
    }

    pub async fn get_tags(&self, image: &str) -> Result<TagsResponse> {
        let request = self.client.get(format!("{}/v2/{}/tags/list", self.url, image));
        let ans = self.send::<TagsResponse>(request).await?;

        Ok(ans.0)
    }

    pub async fn get_manifest(&self, name: &str, reference: &str) -> Result<ManifestResponse> {
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

        let ans = self.send::<Manifest>(request).await?;

        Ok(ManifestResponse {
            manifest: ans.0,
            digest: ans.1.unwrap_or_default(),
            reference: reference.into(),
        })
    }

    pub async fn delete_manifest(&self, name: &str, reference: &str) -> Result<bool> {
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

        match request.send().await {
            Ok(ans) => {
                return if ans.status() == StatusCode::ACCEPTED {
                    Ok(true)
                } else {
                    error!("Status code not 202 ACCEPTED. Status code = {:?}", ans.status());
                    Ok(false)
                }
            }
            Err(e) => {
                error!("Error getting response: {:?}", e);
                bail!(e)
            }
        };
    }

    pub async fn get_config(&self, name: &str, digest: &str) -> Result<(ImageConfigResponse, String)> {
        let request = self.client.get(format!("{}/v2/{}/blobs/{}", self.url, name, digest));
        let ans = self.send::<ImageConfigResponse>(request).await?;

        Ok((ans.0, digest.into()))
    }

    async fn send<T>(&self, request: RequestBuilder) -> Result<(T, Option<String>)>
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

                match res.json::<T>().await {
                    Ok(content) => {
                        return Ok((content, digest));
                    }
                    Err(e) => {
                        error!("Can't parse response: {:?}", e);
                    }
                };
            }
            Err(e) => {
                error!("Error occurred {:?}", e);
            }
        }

        bail!("Error occurred")
    }
}
