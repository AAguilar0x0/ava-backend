use super::serialize_object_id;
use argon2::{
    password_hash::{rand_core::OsRng, Error, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(
        rename(deserialize = "_id", serialize = "id"),
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id"
    )]
    pub _id: Option<ObjectId>,
    pub email: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordUpdate {
    pub old_password: String,
    pub new_password: String,
}

pub fn encrypt_password(password: &str) -> Result<String, Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    Ok(argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string())
}

pub fn verify_password(password: &str, hash_password: &str) -> Result<bool, Error> {
    let parsed_hash = PasswordHash::new(hash_password)?;

    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}
