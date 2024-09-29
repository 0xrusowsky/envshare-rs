use crate::db::{EnvShareDb, SECRETS_DELETE_EXPIRED};

use rocket::http::Status;
use rocket_db_pools::Connection;

#[utoipa::path(
  context_path = "/v1",
  responses(
      (status = 200, description = "Healthcheck"),
  )
)]
#[get("/_healthcheck")]
pub async fn healthcheck_endpoint(mut db: Connection<EnvShareDb>) -> Status {
    match sqlx::query(SECRETS_DELETE_EXPIRED).execute(&mut **db).await {
        Ok(_) => Status::Ok,
        Err(e) => {
            error!("Error deleting expired secrets: {:?}", e);
            Status::InternalServerError
        }
    }
}
