use crate::manager::{get_manifests, get_manifests_from_list};
use crate::registry_api::{types::Manifest, RegistryClient};
use crate::routes::response_types::CountResponse;
use crate::types::{ImageManifest, ImageTags};
use itertools::Itertools;
use rocket::http::Status;
use rocket::{futures::future::join_all, serde::json::Json, State};
use std::collections::HashMap;

#[get("/count/users")]
pub async fn count_users(client: &State<RegistryClient>) -> Json<CountResponse> {
    let repositories: Vec<String> = client.get_catalog().await.unwrap_or_default().repositories;

    let users: Vec<String> = repositories
        .iter()
        .map(|s| s.split('/').next().unwrap().to_string())
        .unique()
        .collect::<Vec<String>>();

    Json(CountResponse { count: users.len() })
}

#[get("/count/repositories")]
pub async fn count_repositories(client: &State<RegistryClient>) -> Json<CountResponse> {
    let repositories: Vec<String> = client.get_catalog().await.unwrap_or_default().repositories;

    Json(CountResponse {
        count: repositories.len(),
    })
}

#[get("/repositories")]
pub async fn get_repositories(client: &State<RegistryClient>) -> Json<Vec<ImageTags>> {
    let repositories: Vec<String> = client.get_catalog().await.unwrap_or_default().repositories;
    let futures = repositories.iter().map(|item| client.get_tags(item));

    let image_tags: Vec<ImageTags> = join_all(futures)
        .await
        .into_iter()
        .filter(|ans| ans.is_ok())
        .map(|ans| {
            let item = ans.unwrap();
            ImageTags {
                image: item.name,
                tags: item.tags.unwrap_or(vec!["Tags not found :(".to_owned()]),
            }
        })
        .collect();

    Json(image_tags)
}

#[get("/<user>/<name>/<tag>")]
pub async fn get_images_by_tag(
    client: &State<RegistryClient>,
    user: &str,
    name: &str,
    tag: &str,
) -> Json<Vec<ImageManifest>> {
    let image = format!("{}/{}", user, name);
    let image_manifest = &client.get_manifest(&image, tag).await.unwrap();

    let manifests = match &image_manifest.manifest {
        Manifest::OCIImageIndexV1(m) => get_manifests_from_list(client, m.manifests.iter(), &image).await,
        Manifest::DockerDistributionManifestListV2(m) => {
            get_manifests_from_list(client, m.manifests.iter(), &image).await
        }
        Manifest::DockerDistributionManifestV2(m) => {
            let mut configs: HashMap<String, (String, u64)> = HashMap::new();

            configs.insert(
                m.config.digest.clone(),
                (image_manifest.digest.clone(), m.get_total_size()),
            );

            get_manifests(client, &configs, &image).await
        }
        Manifest::OCIImageManifestV1(m) => {
            let mut configs: HashMap<String, (String, u64)> = HashMap::new();

            configs.insert(
                m.config.digest.clone(),
                (image_manifest.digest.clone(), m.get_total_size()),
            );

            get_manifests(client, &configs, &image).await
        }
    };

    Json(manifests)
}

#[delete("/<user>/<name>/<tag>")]
pub async fn delete_image(client: &State<RegistryClient>, user: &str, name: &str, tag: &str) -> Status {
    let image = format!("{}/{}", user, name);
    let image_manifest = &client.get_manifest(&image, tag).await.unwrap();
    let digest = image_manifest.digest.clone();

    if let Ok(ans) = client.delete_manifest(&image, &digest).await {
        return if ans {
            Status::Accepted
        } else {
            Status::MethodNotAllowed
        };
    }

    return Status::UnprocessableEntity;
}
