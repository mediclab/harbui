#[macro_use]
extern crate rocket;

use crate::registry_api::{Config as RegistryConfig, RegistryClient};
use crate::types::Config as AppConfig;
use dotenv::dotenv;
use envconfig::Envconfig;
use rocket::fs::FileServer;
use rocket_dyn_templates::Template;

mod manager;
mod registry_api;
mod routes;
mod types;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv().ok();
    pretty_env_logger::init_timed();

    let config = AppConfig::init_from_env().expect("Can't load config from environment");
    let registry_config = RegistryConfig {
        base_uri: config.host.clone(),
        is_secured: !config.unsecured,
        http_basic_user: config.http_basic_user.clone(),
        http_basic_pass: config.http_basic_pass.clone(),
    };

    let _rocket = rocket::build()
        .manage(config.clone())
        .manage(RegistryClient::new(&registry_config))
        .mount(
            "/",
            routes![
                routes::pages::index,
                routes::pages::image,
                routes::pages::pulling,
                routes::pages::pushing,
            ],
        )
        .mount(
            "/api",
            routes![
                routes::api::get_repositories,
                routes::api::get_images_by_tag,
                routes::api::count_users,
                routes::api::count_repositories,
                routes::api::delete_image,
            ],
        )
        .mount("/assets", FileServer::from("public"))
        .attach(Template::fairing())
        .launch()
        .await?;

    Ok(())
}
