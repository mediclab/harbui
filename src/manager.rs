use crate::registry_api::types::{ImageConfigResponse, Manifest, ManifestResponse, OCIImageManifestV1Short};
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

    get_manifests(client, &configs, image).await
}

pub async fn get_manifests(
    client: &State<RegistryClient>,
    configs: &HashMap<String, (String, u64)>,
    image: &str,
) -> Vec<ImageManifest> {
    let config_futures = configs.keys().map(|s| client.get_config(image, s));
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
