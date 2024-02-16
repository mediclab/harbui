#[macro_use]
extern crate rocket;

use crate::registry_api::{Config as RegistryConfig, RegistryClient};
use dotenv::dotenv;
use envconfig::Envconfig;
use rocket::fs::FileServer;

use rocket_dyn_templates::Template;

mod registry_api;
mod routes;
mod types;

#[derive(Envconfig)]
struct Config {
    #[envconfig(from = "REGISTRY_DOMAIN")]
    domain: String,
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv().ok();
    pretty_env_logger::init_timed();

    let config = Config::init_from_env().expect("Can't load config from environment");

    let _rocket = rocket::build()
        .manage(RegistryClient::new(RegistryConfig {
            base_domain: config.domain,
            is_secured: true,
        }))
        .mount("/", routes![routes::index])
        .mount("/api", routes![routes::get_repositories, routes::get_images_by_tag])
        .mount("/count", routes![routes::count_users, routes::count_repositories])
        .mount("/image", routes![routes::image])
        .mount("/css", FileServer::from("public/css"))
        .attach(Template::fairing())
        .launch()
        .await?;

    Ok(())
}
