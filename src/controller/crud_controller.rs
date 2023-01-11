use crate::repository::mongodb_repo::MongoDB;
use actix_web::{
    web::{Data, Json, Path},
    HttpResponse, HttpResponseBuilder,
};
use mongodb::bson::to_document;
use serde::{de::DeserializeOwned, Serialize};

pub async fn create_detail<T>(db: Data<MongoDB<T>>, data: T) -> HttpResponse
where
    T: Serialize + DeserializeOwned + Unpin + Send + Sync,
{
    let result = db.create_record(data).await;

    match result {
        Ok(detail) => HttpResponse::Ok().json(detail),
        Err((status_code, err)) => HttpResponseBuilder::new(status_code).json(err),
    }
}

pub async fn get_all_detail<T>(db: Data<MongoDB<T>>) -> HttpResponse
where
    T: Serialize + DeserializeOwned + Unpin + Send + Sync,
{
    let result = db.get_all_record().await;

    match result {
        Ok(details) => HttpResponse::Ok().json(details),
        Err((status_code, err)) => HttpResponseBuilder::new(status_code).json(err),
    }
}

pub async fn get_detail<T>(db: Data<MongoDB<T>>, path: Path<String>) -> HttpResponse
where
    T: Serialize + DeserializeOwned + Unpin + Send + Sync,
{
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().json("Invalid ID");
    }
    let result = db.get_record(&id).await;

    match result {
        Ok(detail) => HttpResponse::Ok().json(detail),
        Err((status_code, err)) => HttpResponseBuilder::new(status_code).json(err),
    }
}

pub async fn update_detail<T, U>(
    db: Data<MongoDB<T>>,
    path: Path<String>,
    new_detail: Json<U>,
) -> HttpResponse
where
    T: Serialize + DeserializeOwned + Unpin + Send + Sync,
    U: Serialize,
{
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().json("Invalid ID");
    };
    let doc = match to_document(&new_detail) {
        Ok(data) => data,
        Err(err) => return HttpResponse::BadRequest().json(err.to_string()),
    };
    let result = db.update_record(&id, doc).await;

    match result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_detail = db.get_record(&id).await;

                match updated_detail {
                    Ok(detail) => HttpResponse::Ok().json(detail),
                    Err((status_code, err)) => HttpResponseBuilder::new(status_code).json(err),
                }
            } else {
                HttpResponse::NotFound().json("Specified ID not found")
            }
        }
        Err((status_code, err)) => HttpResponseBuilder::new(status_code).json(err),
    }
}

pub async fn delete_detail<T>(db: Data<MongoDB<T>>, path: Path<String>) -> HttpResponse
where
    T: Serialize + DeserializeOwned + Unpin + Send + Sync,
{
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().json("Invalid ID");
    };
    let result = db.delete_record(&id).await;

    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                HttpResponse::Ok().json("Successfully deleted!")
            } else {
                HttpResponse::NotFound().json("Specified ID not found!")
            }
        }
        Err((status_code, err)) => HttpResponseBuilder::new(status_code).json(err),
    }
}
