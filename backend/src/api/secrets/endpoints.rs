use crate::{
    api::{
        apikeys::{ApiKeyError, ApiKeyPermission},
        secrets::types::{Secret, SecretError, TrackedSecret},
    },
    db::{EnvShareDb, SECRETS_DELETE, SECRETS_INSERT, SECRETS_QUERY, SECRETS_UPDATE},
};

use aes_gcm::{Aes256Gcm, Key};
use base64::prelude::*;
use rocket::{
    http::Status,
    response::{self, Responder, Response},
    serde::json::{json, Json},
    Request,
};
use rocket_db_pools::Connection;
use uuid::Uuid;

// -- ENDPOINTS -----------------------------------------------

#[utoipa::path(
    context_path = "/v1",
    security(("bearer_token" = [])),
    request_body = Secret,
    responses(
        (status = 200, body = [Secret], description = "Create a new secret"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error"),
    )
)]
#[post("/secret", format = "json", data = "<secret>")]
pub async fn post_secret(
    key_result: Result<ApiKeyPermission, ApiKeyError>,
    secret: Json<Secret>,
    mut db: Connection<EnvShareDb>,
) -> Result<String, SecretError> {
    match key_result {
        Ok(_) => {
            let (cipher_key, secret) = secret.0.encrypt()?;
            sqlx::query(SECRETS_INSERT)
                .bind(secret.uuid())
                .bind(secret.content())
                .bind(secret.nonce())
                .bind(secret.reads_left())
                .bind(secret.ttl())
                .execute(&mut **db)
                .await
                .map_err(|e| {
                    error!("Error inserting TrackedSecret: {:?}", e);
                    SecretError::DatabaseError
                })?;

            let mut key: Vec<u8> = Vec::new();
            key.extend(cipher_key);
            key.extend_from_slice(secret.uuid().as_bytes());

            Ok(BASE64_STANDARD.encode(key))
        }
        Err(key_error) => Err(key_error.into()),
    }
}

#[utoipa::path(
    context_path = "/v1",
    security(("bearer_token" = [])),
    responses(
        (status = 200, description = "Get a secret by key"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error"),
    )
)]
#[get("/secret/<key>")]
pub async fn get_secret(
    key_result: Result<ApiKeyPermission, ApiKeyError>,
    key: &str,
    mut db: Connection<EnvShareDb>,
) -> Result<Json<TrackedSecret>, SecretError> {
    match key_result {
        Ok(_) => {
            // Decode the key and split it into the cipher_key and uuid
            let key = BASE64_STANDARD
                .decode(key.as_bytes())
                .map_err(|_| SecretError::SecretKeyInvalid)?;
            let (cipher_key, uuid) = key.split_at(32);
            let cipher_key = Key::<Aes256Gcm>::from_slice(cipher_key);
            let uuid = Uuid::from_slice(uuid).map_err(|_| SecretError::SecretKeyInvalid)?;

            // Get the secret from the db
            let mut secret = sqlx::query_as::<_, TrackedSecret>(SECRETS_QUERY)
                .bind(uuid)
                .fetch_one(&mut **db)
                .await
                .map_err(|e| {
                    error!("Error getting TrackedSecret: {:?}", e);
                    SecretError::SecretKeyNotFound
                })?;

            // Update the reads left and update the db
            secret.decrement_reads()?;
            if secret.reads_left() == 0 {
                sqlx::query(SECRETS_DELETE)
                    .bind(secret.uuid())
                    .execute(&mut **db)
                    .await
                    .map_err(|e| {
                        error!("Error deleting TrackedSecret: {:?}", e);
                        SecretError::DatabaseError
                    })?;
            } else {
                sqlx::query(SECRETS_UPDATE)
                    .bind(secret.uuid())
                    .bind(secret.reads_left())
                    .execute(&mut **db)
                    .await
                    .map_err(|e| {
                        error!("Error updating TrackedSecret: {:?}", e);
                        SecretError::DatabaseError
                    })?;
            }

            // Return the decrypted secret
            secret.as_decrypted_json(cipher_key)
        }
        Err(key_error) => Err(key_error.into()),
    }
}

// -- TRAITS -----------------------------------------------

impl From<ApiKeyError> for SecretError {
    fn from(error: ApiKeyError) -> Self {
        SecretError::ApiKeyError(error)
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for SecretError {
    fn respond_to(self, request: &'r Request<'_>) -> response::Result<'o> {
        match self {
            SecretError::ApiKeyError(api_key_error) => api_key_error.respond_to(request),
            SecretError::SecretKeyInvalid => Json(json!({
                "status": Status::BadRequest,
                "message": "Invalid Secret Key".to_owned(),
            }))
            .respond_to(request)
            .map(|res| {
                Response::build_from(res)
                    .status(Status::BadRequest)
                    .finalize()
            }),
            SecretError::EncryptionError => Json(json!({
                "status": Status::BadRequest,
                "message": "Encryption Error".to_owned(),
            }))
            .respond_to(request)
            .map(|res| {
                Response::build_from(res)
                    .status(Status::BadRequest)
                    .finalize()
            }),
            SecretError::DecryptionError => Json(json!({
                "status": Status::BadRequest,
                "message": "Decryption Error".to_owned(),
            }))
            .respond_to(request)
            .map(|res| {
                Response::build_from(res)
                    .status(Status::BadRequest)
                    .finalize()
            }),
            SecretError::SecretExpired => Json(json!({
                "status": Status::BadRequest,
                "message": "Secret Expired".to_owned(),
            }))
            .respond_to(request)
            .map(|res| {
                Response::build_from(res)
                    .status(Status::BadRequest)
                    .finalize()
            }),
            SecretError::SecretKeyNotFound => Json(json!({
                "status": Status::NotFound,
                "message": "Secret Not Found".to_owned(),
            }))
            .respond_to(request)
            .map(|res| {
                Response::build_from(res)
                    .status(Status::BadRequest)
                    .finalize()
            }),
            SecretError::DatabaseError => Json(json!({
                "status": Status::InternalServerError,
                "message": "Database Error".to_owned(),
            }))
            .respond_to(request)
            .map(|res| {
                Response::build_from(res)
                    .status(Status::InternalServerError)
                    .finalize()
            }),
        }
    }
}
