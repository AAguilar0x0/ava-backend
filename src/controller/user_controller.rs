use crate::{
    controller::crud_controller,
    model::user_model::{encrypt_password, verify_password, PasswordUpdate, User, UserUpdate},
    repository::mongodb_repo::MongoDB,
};
use actix_web::{
    delete, get,
    http::StatusCode,
    post, put,
    web::{self, Data, Json, Path},
    HttpResponse, HttpResponseBuilder, Scope,
};
use dotenv;
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use mongodb::bson::{doc, to_document};
use sha2::Sha256;
use std::collections::BTreeMap;

pub fn new() -> Scope {
    web::scope("/users")
        .service(auth)
        .service(update_password)
        .service(create)
        .service(get_all)
        .service(get)
        .service(update)
        .service(delete)
}

#[post("/auth")]
pub async fn auth(db: Data<MongoDB<User>>, credentials: Json<User>) -> HttpResponse {
    let user = match db
        .find_one_record(doc! {"email": credentials.email.clone()})
        .await
    {
        Ok(user) => user,
        Err((status_code, message)) => return HttpResponseBuilder::new(status_code).json(message),
    };

    let is_verified = match verify_password(&credentials.password, &user.password) {
        Ok(val) => val,
        Err(error) => {
            return HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR)
                .json(error.to_string())
        }
    };

    if !is_verified {
        return HttpResponseBuilder::new(StatusCode::UNAUTHORIZED).json("Invalid credentials");
    }

    let secret = match dotenv::var("JWT_SECRET") {
        Ok(secret) => secret,
        Err(error) => {
            return HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR)
                .json(error.to_string())
        }
    };

    let key: Hmac<Sha256> = match Hmac::new_from_slice(secret.as_bytes()) {
        Ok(key) => key,
        Err(error) => {
            return HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR)
                .json(error.to_string())
        }
    };

    let mut claims = BTreeMap::new();
    claims.insert("email", user.email);
    claims.insert(
        "id",
        match user._id {
            Some(id) => id.to_string(),
            None => {
                return HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR)
                    .json("User ID does not exist.")
            }
        },
    );

    match claims.sign_with_key(&key) {
        Ok(token_str) => HttpResponse::Ok().json(token_str),
        Err(error) => {
            HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).json(error.to_string())
        }
    }
}

#[put("/auth/{id}")]
pub async fn update_password(
    db: Data<MongoDB<User>>,
    path: Path<String>,
    passwords: Json<PasswordUpdate>,
) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().json("Invalid ID");
    }
    let result = db.get_record(&id).await;

    let mut user = match result {
        Ok(user) => user,
        Err((status_code, err)) => return HttpResponseBuilder::new(status_code).json(err),
    };

    let is_verified = match verify_password(&passwords.old_password, &user.password) {
        Ok(val) => val,
        Err(error) => {
            return HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR)
                .json(error.to_string())
        }
    };

    if !is_verified {
        return HttpResponseBuilder::new(StatusCode::UNAUTHORIZED).json("Invalid credentials");
    }

    let doc = match encrypt_password(&passwords.new_password) {
        Ok(password) => doc! { "password": password },
        Err(error) => {
            return HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR)
                .json(error.to_string())
        }
    };

    let result = db.update_record(&id, doc).await;

    user.password.clear();

    match result {
        Ok(update_result) => {
            if update_result.matched_count == 1 {
                HttpResponse::Ok().json(user)
            } else {
                HttpResponse::NotFound().json("Specified ID not found")
            }
        }
        Err((status_code, err)) => HttpResponseBuilder::new(status_code).json(err),
    }
}

#[post("")]
pub async fn create(db: Data<MongoDB<User>>, new_user: Json<User>) -> HttpResponse {
    if new_user.password.is_empty() {
        return HttpResponse::BadRequest().json("Invalid empty password");
    }
    let password = match encrypt_password(&new_user.password) {
        Ok(password) => password,
        Err(error) => {
            return HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR)
                .json(error.to_string())
        }
    };
    let data = User {
        _id: None,
        email: new_user.email.to_owned(),
        password,
    };
    crud_controller::create(db, data).await
}

#[get("")]
pub async fn get_all(db: Data<MongoDB<User>>) -> HttpResponse {
    let result = db.get_all_record().await;

    match result {
        Ok(mut records) => {
            records.iter_mut().for_each(|user| {
                user.password.clear();
            });
            return HttpResponse::Ok().json(records);
        }
        Err((status_code, err)) => HttpResponseBuilder::new(status_code).json(err),
    }
}

#[get("/{id}")]
pub async fn get(db: Data<MongoDB<User>>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().json("Invalid ID");
    }
    let result = db.get_record(&id).await;

    match result {
        Ok(mut record) => {
            record.password.clear();
            return HttpResponse::Ok().json(record);
        }
        Err((status_code, err)) => HttpResponseBuilder::new(status_code).json(err),
    }
}

#[put("/{id}")]
pub async fn update(
    db: Data<MongoDB<User>>,
    path: Path<String>,
    new_user: Json<UserUpdate>,
) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().json("Invalid ID");
    };
    let doc = match to_document(&new_user) {
        Ok(data) => data,
        Err(err) => return HttpResponse::BadRequest().json(err.to_string()),
    };
    let result = db.update_record(&id, doc).await;

    match result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated = db.get_record(&id).await;

                match updated {
                    Ok(mut record) => {
                        record.password.clear();
                        return HttpResponse::Ok().json(record);
                    }
                    Err((status_code, err)) => HttpResponseBuilder::new(status_code).json(err),
                }
            } else {
                HttpResponse::NotFound().json("Specified ID not found")
            }
        }
        Err((status_code, err)) => HttpResponseBuilder::new(status_code).json(err),
    }
}

#[delete("/{id}")]
pub async fn delete(db: Data<MongoDB<User>>, path: Path<String>) -> HttpResponse {
    crud_controller::delete(db, path).await
}
