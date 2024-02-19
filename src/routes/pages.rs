use crate::registry_api::RegistryClient;
use crate::types::Config as AppConfig;
use rocket::State;
use rocket_dyn_templates::{context, Template};

#[get("/")]
pub async fn index(config: &State<AppConfig>) -> Template {
    Template::render(
        "index",
        context! {
            page_title: "HarbUI - Docker Registry UI - Images",
            version: config.version.clone(),
            active_link: "images",
        },
    )
}

#[get("/pulling")]
pub async fn pulling(config: &State<AppConfig>) -> Template {
    Template::render(
        "pulling",
        context! {
            page_title: "HarbUI - Docker Registry UI - Pulling",
            version: config.version.clone(),
            domain: config.host.clone(),
            active_link: "pulling",
        },
    )
}

#[get("/pushing")]
pub async fn pushing(config: &State<AppConfig>) -> Template {
    Template::render(
        "pushing",
        context! {
            page_title: "HarbUI - Docker Registry UI - Pushing",
            version: config.version.clone(),
            domain: config.host.clone(),
            active_link: "pushing",
        },
    )
}

#[get("/image/<user>/<name>")]
pub async fn image(client: &State<RegistryClient>, config: &State<AppConfig>, user: &str, name: &str) -> Template {
    let image = format!("{}/{}", user, name);
    let tags = client.get_tags(&format!("{}/{}", user, name)).await.unwrap().tags;

    Template::render(
        "image",
        context! {
            page_title: format!("HarbUI - Docker Registry UI - {}", &image),
            path: image,
            tags: tags,
            domain: config.host.clone(),
            version: config.version.clone(),
            active_link: "images",
        },
    )
}
