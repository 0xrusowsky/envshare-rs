use rocket::serde::Deserialize;
use sqlx::FromRow;

use crate::db::{EnvShareDb, API_KEY_QUERY};
use rocket::http::Status;
use rocket::outcome::Outcome::{Error, Forward, Success};
use rocket::request::{FromRequest, Outcome, Request};
use rocket::response::{self, Responder, Response};
use rocket::serde::json::{json, Json};
use rocket_db_pools::Connection;
// use tracing::error;

#[derive(Debug, Deserialize, FromRow)]
pub struct ApiKeyPermission {
    pub key: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKeyPermission {
    type Error = ApiKeyError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.guard::<Connection<EnvShareDb>>().await {
            Success(mut db) => match req.headers().get_one("Authorization") {
                None => Error((Status::Unauthorized, ApiKeyError::Missing)),
                Some(key) => match key.strip_prefix("Bearer ") {
                    None => Error((Status::Unauthorized, ApiKeyError::Invalid)),
                    Some(key) => {
                        let result = sqlx::query_as::<_, ApiKeyPermission>(API_KEY_QUERY)
                            .bind(key)
                            .fetch_one(&mut **db)
                            .await;
                        match result {
                            Ok(key) => Success(key),
                            Err(e) => {
                                error!("Database error: {}", e);
                                Error((Status::Unauthorized, ApiKeyError::Invalid))
                            }
                        }
                    }
                },
            },
            Error(_) => Error((Status::InternalServerError, ApiKeyError::DatabaseError)),
            Forward(_) => Error((Status::InternalServerError, ApiKeyError::DatabaseError)),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ApiKeyError {
    Missing,
    Invalid,
    DatabaseError,
}

impl<'r> Responder<'r, 'static> for ApiKeyError {
    fn respond_to(self, request: &'r Request<'_>) -> response::Result<'static> {
        let error_message = match self {
            ApiKeyError::Missing => "API key is missing.",
            ApiKeyError::Invalid => "API key is invalid.",
            ApiKeyError::DatabaseError => "Database error.",
        };

        let status = match self {
            ApiKeyError::Missing => Status::Unauthorized,
            ApiKeyError::Invalid => Status::Unauthorized,
            ApiKeyError::DatabaseError => Status::InternalServerError,
        };

        // Create a JSON response body with the error message
        let json_response = Json(json!({
            "status": status,
            "message": error_message
        }));

        // Use the request passed to `respond_to` to generate the response
        json_response.respond_to(request).map(|res| {
            // Customize the response further if needed, e.g., by setting a custom status
            Response::build_from(res).status(status).finalize()
        })
    }
}
