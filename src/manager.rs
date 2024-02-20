use crate::registry_api::types::{ImageConfigResponse, Manifest, OCIImageManifestV1Short, RegistryAnswer};
use crate::registry_api::RegistryClient;
use crate::types::ImageManifest;
use rocket::futures::future::join_all;
use rocket::State;
use std::collections::HashMap;

pub async fn get_manifests_from_list<'a, I>(
    client: &State<RegistryClient>,
    manifests_iter: I,
    image: &str,
) -> Vec<ImageManifest>
where
    I: Iterator<Item = &'a OCIImageManifestV1Short>,
{
    let futures = manifests_iter.filter_map(|item| {
        if item.platform.os != "unknown" {
            return Some(client.get_manifest(image, &item.digest));
        }

        None
    });

    let ans: Vec<RegistryAnswer<Manifest>> = join_all(futures).await.into_iter().filter_map(|ans| ans.ok()).collect();

    let mut configs: HashMap<String, (String, u64)> = HashMap::new();

    ans.into_iter().for_each(|item| {
        let manifest_digest = item.digest.unwrap();
        match item.content {
            Manifest::OCIImageManifestV1(c) => {
                let config_digest = c.config.digest.clone();
                configs.insert(config_digest, (manifest_digest, c.get_total_size()));
            }
            Manifest::DockerDistributionManifestV2(c) => {
                let config_digest = c.config.digest.clone();
                configs.insert(config_digest, (manifest_digest, c.get_total_size()));
            }
            _ => {}
        }
    });

    get_manifests(client, &configs, image).await
}

pub async fn get_manifests(
    client: &State<RegistryClient>,
    configs: &HashMap<String, (String, u64)>,
    image: &str,
) -> Vec<ImageManifest> {
    let config_futures = configs.keys().map(|s| client.get_config(image, s));
    let configs_ans: Vec<RegistryAnswer<ImageConfigResponse>> = join_all(config_futures)
        .await
        .into_iter()
        .filter_map(|ans| ans.ok())
        .collect();

    configs_ans
        .into_iter()
        .map(|ans| {
            let manifest_digest = configs.get(&ans.digest.unwrap()).unwrap();

            ImageManifest {
                digest: manifest_digest.0.clone(),
                author: ans.content.author.unwrap_or_default(),
                os: ans.content.os,
                architecture: ans.content.architecture,
                total_size: manifest_digest.1,
            }
        })
        .collect()
}
