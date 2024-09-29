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
use rocket_db_pools::Database;
// use sqlx::postgres::PgPoolOptions;
// use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[launch]
#[tokio::main]
async fn rocket() -> _ {
    // Open a connection to the database on the main thread
    let db_url = get_postgres_connection();
    // let db_pool = Arc::new(
    //     PgPoolOptions::new()
    //         .max_connections(5)
    //         .connect(&db_url)
    //         .await
    //         .expect("Failed to connect to the database"),
    // );

    let figment = Figment::from(rocket::Config::default())
        .merge(Env::prefixed("ROCKET_").global())
        .merge(("databases.envshare_db.url", db_url));

    rocket::custom(figment)
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
