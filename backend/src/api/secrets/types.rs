use crate::api::apikeys::ApiKeyError;

use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use base64::prelude::*;
use chrono::Utc;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize, Serializer};
use serde_json::json;
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

pub enum SecretError {
    ApiKeyError(ApiKeyError),
    DatabaseError,
    EncryptionError,
    DecryptionError,
    SecretKeyInvalid,
    SecretKeyNotFound,
    SecretExpired,
}

#[derive(Debug, Clone, PartialEq, FromRow, ToSchema)]
pub struct TrackedSecret {
    uuid: Uuid,
    content: String,
    nonce: String,
    reads_left: i64,
    ttl: i64,
}

#[derive(Debug, Clone, PartialEq, FromRow, ToSchema, Deserialize)]
pub struct Secret {
    pub content: String,
    pub max_reads: i64,
    pub ttl: i64,
}

impl Secret {
    pub fn encrypt(self) -> Result<([u8; 32], TrackedSecret), SecretError> {
        let key = Aes256Gcm::generate_key(OsRng);
        let nonce = Aes256Gcm::generate_nonce(OsRng);

        let cipher = Aes256Gcm::new(&key);
        let encrypted_bytes = cipher
            .encrypt(&nonce, self.content.as_bytes().as_ref())
            .map_err(|e| {
                error!("Error encrypting secret content: {:?}", e);
                SecretError::EncryptionError
            })?;

        Ok((
            key.into(),
            TrackedSecret {
                uuid: Uuid::new_v4(),
                content: BASE64_STANDARD.encode(encrypted_bytes),
                reads_left: self.max_reads,
                ttl: self.ttl,
                nonce: BASE64_STANDARD.encode(nonce.as_slice()),
            },
        ))
    }
}

impl TrackedSecret {
    // Getters
    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn nonce(&self) -> &str {
        &self.nonce
    }

    pub fn reads_left(&self) -> i64 {
        self.reads_left
    }

    pub fn ttl(&self) -> i64 {
        self.ttl
    }

    pub fn has_expired(&self) -> bool {
        Utc::now().timestamp() >= self.ttl || self.reads_left == 0
    }

    pub fn as_decrypted_json(self, key: &Key<Aes256Gcm>) -> Result<Json<Self>, SecretError> {
        let content = self.decrypt(key)?;

        Ok(Json(TrackedSecret {
            uuid: self.uuid,
            content,
            reads_left: self.reads_left,
            ttl: self.ttl,
            nonce: self.nonce,
        }))
    }

    fn decrypt(&self, key: &Key<Aes256Gcm>) -> Result<String, SecretError> {
        let encrypted_bytes = BASE64_STANDARD.decode(&self.content).map_err(|e| {
            error!("Error decoding base64 encrypted secret: {:?}", e);
            SecretError::DecryptionError
        })?;
        let nonce = BASE64_STANDARD.decode(&self.nonce).map_err(|e| {
            error!("Error decoding base64 nonce: {:?}", e);
            SecretError::DecryptionError
        })?;
        let nonce = Nonce::from_slice(nonce.as_slice());
        let cipher = Aes256Gcm::new(&key);

        let decrypted = cipher
            .decrypt(&nonce, encrypted_bytes.as_slice())
            .map_err(|e| {
                error!("Error decrypting secret content (bytes): {:?}", e);
                SecretError::DecryptionError
            })?;

        Ok(String::from_utf8(decrypted).map_err(|e| {
            error!("Error converting decrypted content to UTF-8: {:?}", e);
            SecretError::DecryptionError
        })?)
    }

    // Setters
    pub fn decrement_reads(&mut self) -> Result<(), SecretError> {
        if self.has_expired() {
            return Err(SecretError::SecretExpired);
        }

        self.reads_left -= 1;
        Ok(())
    }
}

impl Serialize for TrackedSecret {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serde_json::Map::new();

        map.insert("uuid".to_owned(), json!(self.uuid.to_string()));
        map.insert("content".to_owned(), json!(self.content));
        map.insert("reads_left".to_owned(), json!(self.reads_left));
        map.insert("ttl".to_owned(), json!(self.ttl));
        map.insert("nonce".to_owned(), json!(self.nonce));

        serde_json::Value::Object(map).serialize(serializer)
    }
}
