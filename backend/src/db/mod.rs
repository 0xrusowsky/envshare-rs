use rocket_db_pools::{sqlx, Database};
use std::env;

#[derive(Database)]
#[database("envshare_db")]
pub struct EnvShareDb(sqlx::PgPool);

// -- UTILS -----------------------------------------------

pub fn get_postgres_connection() -> String {
    dotenv::dotenv().ok();

    let postgres_port = env::var("POSTGRES_PORT").unwrap_or_else(|_| "5432".to_string());
    let postgres_user = env::var("POSTGRES_USER").expect("POSTGRES_USER must be set");
    let postgres_db = env::var("POSTGRES_DB").expect("POSTGRES_DB must be set");
    let postgres_pwd = env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set");
    let postgres_host = env::var("POSTGRES_HOST").expect("POSTGRES_PASSWORD must be set");

    format!(
        "postgresql://{}:{}@{}:{}/{}",
        postgres_user,
        urlencoding::encode(&postgres_pwd),
        postgres_host,
        postgres_port,
        postgres_db
    )
}

// -- QUERIES ---------------------------------------------

pub const API_KEY_QUERY: &str = r#"SELECT * from apikeys where key = $1"#;
pub const SECRETS_QUERY: &str = r#"SELECT uuid, content, nonce, reads_left, extract(epoch from ttl)::INT8 as ttl from secrets where uuid = $1"#;
pub const SECRETS_INSERT: &str = r#"
INSERT INTO secrets (uuid, content, nonce, reads_left, ttl)
VALUES ($1, $2, $3, $4, to_timestamp($5))"#;
pub const SECRETS_UPDATE: &str = r#"UPDATE secrets SET reads_left = $2 WHERE uuid = $1"#;
pub const SECRETS_DELETE: &str = r#"DELETE from secrets WHERE uuid = $1"#;
pub const SECRETS_DELETE_EXPIRED: &str = r#"DELETE from secrets WHERE ttl < now()"#;
