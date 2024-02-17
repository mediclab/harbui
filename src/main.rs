#[macro_use]
extern crate rocket;

use crate::registry_api::{Config as RegistryConfig, RegistryClient};
use dotenv::dotenv;
use envconfig::Envconfig;
use rocket::fs::FileServer;
use rocket_dyn_templates::Template;

mod manager;
mod registry_api;
mod routes;
mod types;

#[derive(Envconfig, Clone)]
struct Config {
    #[envconfig(from = "REGISTRY_DOMAIN")]
    domain: String,
    #[envconfig(from = "HARBUI_VERSION", default = "dev")]
    version: String,
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv().ok();
    pretty_env_logger::init_timed();

    let config = Config::init_from_env().expect("Can't load config from environment");

    let _rocket = rocket::build()
        .manage(config.clone())
        .manage(RegistryClient::new(RegistryConfig {
            base_domain: config.domain,
            is_secured: true,
        }))
        .mount(
            "/",
            routes![
                routes::pages::index,
                routes::pages::image,
                routes::pages::pulling,
                routes::pages::pushing
            ],
        )
        .mount(
            "/api",
            routes![
                routes::api::get_repositories,
                routes::api::get_images_by_tag,
                routes::api::count_users,
                routes::api::count_repositories
            ],
        )
        .mount("/css", FileServer::from("public/css"))
        .attach(Template::fairing())
        .launch()
        .await?;

    Ok(())
}
