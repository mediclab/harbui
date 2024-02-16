use crate::registry_api::types::*;
use anyhow::{bail, Result};
use reqwest::header::ACCEPT;
use reqwest::RequestBuilder;
use serde::de::DeserializeOwned;

pub mod types;

#[derive(Clone, Debug)]
pub struct Config {
    pub base_domain: String,
    pub is_secured: bool,
}

#[derive(Clone, Debug)]
pub struct RegistryClient {
    client: reqwest::Client,
    url: String,
}

impl RegistryClient {
    pub fn new(config: Config) -> Self {
        let client = reqwest::Client::builder().build().unwrap();
        let url = if config.is_secured {
            format!("https://{}", config.base_domain)
        } else {
            format!("http://{}", config.base_domain)
        };

        Self { client, url }
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

    pub async fn get_config(&self, name: &str, digest: &str) -> Result<(ImageConfigResponse, String)> {
        let request = self.client.get(format!("{}/v2/{}/blobs/{}", self.url, name, digest));
        let ans = self.send::<ImageConfigResponse>(request).await?;

        Ok((ans.0, digest.into()))
    }

    async fn send<T>(&self, request: RequestBuilder) -> Result<(T, Option<String>)>
    where
        T: DeserializeOwned,
    {
        match request.send().await {
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
