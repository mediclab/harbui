use crate::manager::{get_manifests, get_manifests_from_list};
use crate::registry_api::{types::Manifest, RegistryClient};
use crate::routes::types::{ApiAnswer, ApiError, ApiResponse, ConfigResponse, CountResponse, ImageManifestResponse};
use crate::types::{Config, ImageTags};
use itertools::Itertools;
use rocket::{futures::future::join_all, State};
use std::collections::HashMap;

#[get("/count/users")]
pub async fn count_users(client: &State<RegistryClient>) -> ApiResponse<CountResponse> {
    let repositories = match client.get_catalog().await {
        Ok(r) => r.content.repositories,
        Err(_) => Vec::new(),
    };

    let users: Vec<String> = repositories
        .iter()
        .map(|s| s.split('/').next().unwrap().to_string())
        .unique()
        .collect::<Vec<String>>();

    ApiAnswer::success(CountResponse { count: users.len() })
}

#[get("/count/repositories")]
pub async fn count_repositories(client: &State<RegistryClient>) -> ApiResponse<CountResponse> {
    let repos = match client.get_catalog().await {
        Ok(r) => r.content.repositories,
        Err(_) => Vec::new(),
    };

    ApiAnswer::success(CountResponse { count: repos.len() })
}

#[get("/repositories")]
pub async fn get_repositories(client: &State<RegistryClient>) -> ApiResponse<Vec<ImageTags>> {
    let repos = match client.get_catalog().await {
        Ok(r) => r.content.repositories,
        Err(_) => Vec::new(),
    };
    let futures = repos.iter().map(|item| client.get_tags(item));
    let image_tags: Vec<ImageTags> = join_all(futures)
        .await
        .into_iter()
        .filter(|ans| ans.is_ok())
        .map(|ans| {
            let item = ans.unwrap().content;
            ImageTags {
                image: item.name,
                tags: item.tags.unwrap_or(vec!["Tags not found :(".to_owned()]),
            }
        })
        .collect();

    ApiAnswer::success(image_tags)
}

#[get("/<user>/<name>/tags")]
pub async fn get_tags(client: &State<RegistryClient>, user: &str, name: &str) -> ApiResponse<Vec<String>> {
    let image = format!("{}/{}", user, name);

    if let Ok(ans) = client.get_tags(&image).await {
        ApiAnswer::success(ans.content.tags.unwrap_or_default())
    } else {
        ApiAnswer::success(Vec::new())
    }
}

#[get("/config")]
pub async fn get_config(state: &State<Config>) -> ApiResponse<ConfigResponse> {
    ApiAnswer::success(ConfigResponse {
        registry_domain: state.host.clone(),
        version: state.version.clone(),
    })
}

#[get("/<user>/<name>/<tag>")]
pub async fn get_images_by_tag(
    client: &State<RegistryClient>,
    user: &str,
    name: &str,
    tag: &str,
) -> ApiResponse<ImageManifestResponse> {
    let image = format!("{}/{}", user, name);
    let image_manifest = match client.get_manifest(&image, tag).await {
        Ok(c) => c,
        Err(e) => {
            error!("Caught error {:?}", e);
            return Err(ApiError::unprocessable(&e.to_string()));
        }
    };

    let manifests = match &image_manifest.content {
        Manifest::OCIImageIndexV1(m) => get_manifests_from_list(client, m.manifests.iter(), &image).await,
        Manifest::DockerDistributionManifestListV2(m) => {
            get_manifests_from_list(client, m.manifests.iter(), &image).await
        }
        Manifest::DockerDistributionManifestV2(m) => {
            let mut configs: HashMap<String, (String, u64)> = HashMap::new();
            let config_digest = m.config.digest.clone();
            let manifest_digest = image_manifest.digest.unwrap();

            configs.insert(config_digest, (manifest_digest, m.get_total_size()));
            get_manifests(client, &configs, &image).await
        }
        Manifest::OCIImageManifestV1(m) => {
            let mut configs: HashMap<String, (String, u64)> = HashMap::new();
            let config_digest = m.config.digest.clone();
            let manifest_digest = image_manifest.digest.unwrap();

            configs.insert(config_digest, (manifest_digest, m.get_total_size()));
            get_manifests(client, &configs, &image).await
        }
    };

    ApiAnswer::success(ImageManifestResponse {
        image,
        tag: tag.to_owned(),
        manifests,
    })
}

#[delete("/<user>/<name>/<tag>")]
pub async fn delete_image(client: &State<RegistryClient>, user: &str, name: &str, tag: &str) -> ApiResponse<String> {
    let image = format!("{}/{}", user, name);
    let image_manifest = match client.get_manifest(&image, tag).await {
        Ok(m) => m,
        Err(err) => return Err(ApiError::not_found(&err.message)),
    };

    if let Err(err) = client
        .delete_manifest(&image, &image_manifest.digest.unwrap_or_default())
        .await
    {
        return Err(ApiError::unprocessable(&err.message));
    }

    ApiAnswer::success("{}".to_string())
}
