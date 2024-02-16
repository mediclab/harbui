use crate::registry_api::types::{ImageConfigResponse, Manifest, ManifestResponse, OCIImageManifestV1Short};
use crate::registry_api::RegistryClient;
use crate::types::{ImageManifest, ImageTags, Manifests};
use itertools::Itertools;
use rocket::futures::future::join_all;
use rocket::State;
use rocket_dyn_templates::{context, Template};
use serde_json::json;
use std::collections::HashMap;

#[get("/users")]
pub async fn count_users(client: &State<RegistryClient>) -> String {
    let repositories: Vec<String> = client.get_catalog().await.unwrap_or_default().repositories;

    let users: Vec<String> = repositories
        .iter()
        .map(|s| s.split('/').next().unwrap().to_string())
        .unique()
        .collect::<Vec<String>>();

    json!({ "count": users.len() }).to_string()
}

#[get("/repositories")]
pub async fn count_repositories(client: &State<RegistryClient>) -> String {
    let repositories: Vec<String> = client.get_catalog().await.unwrap_or_default().repositories;

    json!({ "count": repositories.len() }).to_string()
}

#[get("/repositories")]
pub async fn get_repositories(client: &State<RegistryClient>) -> String {
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
                tags: item.tags,
            }
        })
        .collect();

    json!({ "repositories": image_tags }).to_string()
}

#[get("/")]
pub async fn index() -> Template {
    Template::render(
        "index",
        context! {
            page_title: "HarbUI - Docker Registry UI - Images",
        },
    )
}

#[get("/<user>/<name>")]
pub async fn image(client: &State<RegistryClient>, user: &str, name: &str) -> Template {
    let image = format!("{}/{}", user, name);
    let tags = client.get_tags(&format!("{}/{}", user, name)).await.unwrap().tags;

    Template::render(
        "image",
        context! {
            page_title: "HarbUI - Docker Registry UI - Images",
            path: image,
            tags: tags,
        },
    )
}

#[get("/<user>/<name>/<tag>")]
pub async fn get_images_by_tag(client: &State<RegistryClient>, user: &str, name: &str, tag: &str) -> String {
    let image = format!("{}/{}", user, name);
    let image_manifest = client.get_manifest(&image, tag).await.unwrap();

    let manifests = match image_manifest.clone().manifest {
        Manifest::OCIImageIndexV1(m) => get_manifests(client, m.manifests.iter(), &image).await,
        Manifest::DockerDistributionManifestListV2(m) => get_manifests(client, m.manifests.iter(), &image).await,
        Manifest::DockerDistributionManifestV2(m) => {
            let mut configs: HashMap<String, (String, u64)> = HashMap::new();

            configs.insert(
                m.clone().config.digest,
                (image_manifest.clone().digest, m.clone().get_total_size()),
            );

            let config_futures = configs.keys().into_iter().map(|s| client.get_config(&image, s));
            let configs_ans: Vec<(ImageConfigResponse, String)> = join_all(config_futures)
                .await
                .into_iter()
                .filter_map(|ans| ans.ok())
                .collect();

            configs_ans
                .into_iter()
                .map(|(res, digest)| {
                    let manifest_digest = configs.get(&digest).unwrap();

                    ImageManifest {
                        digest: manifest_digest.0.clone(),
                        author: res.author.unwrap_or_default(),
                        os: res.os,
                        architecture: res.architecture,
                        total_size: manifest_digest.1,
                    }
                })
                .collect()
        }
        _ => Vec::new(),
    };

    json!({ "manifests": manifests }).to_string()
}

async fn get_manifests<'a, I>(client: &State<RegistryClient>, manifests_iter: I, image: &str) -> Vec<ImageManifest>
where
    I: Iterator<Item = &'a OCIImageManifestV1Short>,
{
    let futures = manifests_iter.filter_map(|item| {
        if item.platform.os != "unknown" {
            return Some(client.get_manifest(&image, &item.digest));
        }

        None
    });

    let resps: Vec<ManifestResponse> = join_all(futures).await.into_iter().filter_map(|ans| ans.ok()).collect();

    let mut configs: HashMap<String, (String, u64)> = HashMap::new();

    resps.iter().for_each(|item| match &item.manifest {
        Manifest::OCIImageManifestV1(c) => {
            configs.insert(c.config.digest.clone(), (item.digest.clone(), c.get_total_size()));
        }
        Manifest::DockerDistributionManifestV2(c) => {
            configs.insert(c.config.digest.clone(), (item.digest.clone(), c.get_total_size()));
        }
        _ => {}
    });

    let config_futures = configs.keys().into_iter().map(|s| client.get_config(&image, s));

    let configs_ans: Vec<(ImageConfigResponse, String)> = join_all(config_futures)
        .await
        .into_iter()
        .filter_map(|ans| ans.ok())
        .collect();

    configs_ans
        .into_iter()
        .map(|(res, digest)| {
            let manifest_digest = configs.get(&digest).unwrap();

            ImageManifest {
                digest: manifest_digest.0.clone(),
                author: res.author.unwrap_or_default(),
                os: res.os,
                architecture: res.architecture,
                total_size: manifest_digest.1,
            }
        })
        .collect()
}
