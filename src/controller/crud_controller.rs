use crate::repository::mongodb_repo::MongoDB;
use actix_web::{
    web::{Data, Json, Path},
    HttpResponse, HttpResponseBuilder,
};
use mongodb::bson::{doc, to_document};
use serde::{de::DeserializeOwned, Serialize};

pub async fn create<T>(db: Data<MongoDB<T>>, data: T) -> HttpResponse
where
    T: Serialize + DeserializeOwned + Unpin + Send + Sync,
{
    let result = db.create_record(data).await;

    match result {
        Ok(record) => match record.inserted_id.as_object_id() {
            Some(object_id) => HttpResponse::Ok().json(doc! { "id": object_id.to_string() }),
            None => HttpResponse::Ok().json(record),
        },
        Err((status_code, err)) => HttpResponseBuilder::new(status_code).json(err),
    }
}

pub async fn get_all<T>(db: Data<MongoDB<T>>) -> HttpResponse
where
    T: Serialize + DeserializeOwned + Unpin + Send + Sync,
{
    let result = db.get_all_record().await;

    match result {
        Ok(records) => HttpResponse::Ok().json(records),
        Err((status_code, err)) => HttpResponseBuilder::new(status_code).json(err),
    }
}

pub async fn get<T>(db: Data<MongoDB<T>>, path: Path<String>) -> HttpResponse
where
    T: Serialize + DeserializeOwned + Unpin + Send + Sync,
{
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().json("Invalid ID");
    }
    let result = db.get_record(&id).await;

    match result {
        Ok(record) => HttpResponse::Ok().json(record),
        Err((status_code, err)) => HttpResponseBuilder::new(status_code).json(err),
    }
}

pub async fn update<T, U>(db: Data<MongoDB<T>>, path: Path<String>, new: Json<U>) -> HttpResponse
where
    T: Serialize + DeserializeOwned + Unpin + Send + Sync,
    U: Serialize,
{
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().json("Invalid ID");
    };
    let doc = match to_document(&new) {
        Ok(data) => data,
        Err(err) => return HttpResponse::BadRequest().json(err.to_string()),
    };
    let result = db.update_record(&id, doc).await;

    match result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated = db.get_record(&id).await;

                match updated {
                    Ok(record) => HttpResponse::Ok().json(record),
                    Err((status_code, err)) => HttpResponseBuilder::new(status_code).json(err),
                }
            } else {
                HttpResponse::NotFound().json("Specified ID not found")
            }
        }
        Err((status_code, err)) => HttpResponseBuilder::new(status_code).json(err),
    }
}

pub async fn delete<T>(db: Data<MongoDB<T>>, path: Path<String>) -> HttpResponse
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
