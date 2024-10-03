mod api;
mod db;

#[macro_use]
extern crate rocket;

use api::{
    healthcheck::*,
    secrets::endpoints::{get_secret, post_secret},
    swagger::ApiDoc,
};
use db::{get_postgres_connection, EnvShareDb};

use rocket::figment::{providers::Env, Figment};
use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use rocket_db_pools::Database;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[launch]
#[tokio::main]
async fn rocket() -> _ {
    let cors = rocket_cors::CorsOptions {
        allowed_origins: AllowedOrigins::all(),
        allowed_methods: vec![Method::Get, Method::Post]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("Unable to initialize cors policy");

    // Open a connection to the database on the main thread
    let db_url = get_postgres_connection();

    let figment = Figment::from(rocket::Config::default())
        .merge(Env::prefixed("ROCKET_").global())
        .merge(("databases.envshare_db.url", db_url));

    rocket::custom(figment)
        .attach(cors)
        .attach(EnvShareDb::init())
        .mount(
            "/v1",
            routes![healthcheck_endpoint, get_secret, post_secret],
        )
        .mount(
            "/",
            SwaggerUi::new("/swagger-ui/<_..>").url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
}
