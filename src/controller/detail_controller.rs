use crate::{model::detail_model::Detail, repository::detail_repo::DetailRepo};
use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    HttpResponse, HttpResponseBuilder,
};
use mongodb::bson::oid::ObjectId;

#[post("/detail")]
pub async fn create_detail(db: Data<DetailRepo>, new_detail: Json<Detail>) -> HttpResponse {
    let data = Detail {
        id: None,
        name: new_detail.name.to_owned(),
        description: new_detail.description.to_owned(),
        image: new_detail.image.to_owned(),
    };

    let result = db.create_record(data).await;

    match result {
        Ok(detail) => HttpResponse::Ok().json(detail),
        Err((status_code, err)) => HttpResponseBuilder::new(status_code).json(err),
    }
}

#[get("/detail/{id}")]
pub async fn get_detail(db: Data<DetailRepo>, path: Path<String>) -> HttpResponse {
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

#[put("/detail/{id}")]
pub async fn update_detail(
    db: Data<DetailRepo>,
    path: Path<String>,
    new_detail: Json<Detail>,
) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().json("Invalid ID");
    };
    let data = Detail {
        id: Some(match ObjectId::parse_str(&id) {
            Ok(id) => id,
            Err(err) => return HttpResponse::BadRequest().json(err.to_string()),
        }),
        name: new_detail.name.to_owned(),
        description: new_detail.description.to_owned(),
        image: new_detail.image.to_owned(),
    };

    let result = db.update_record(&id, data).await;

    match result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_detail = db.get_record(&id).await;

                return match updated_detail {
                    Ok(detail) => HttpResponse::Ok().json(detail),
                    Err((status_code, err)) => HttpResponseBuilder::new(status_code).json(err),
                };
            } else {
                return HttpResponse::NotFound().json("No detail found with specified ID");
            }
        }
        Err((status_code, err)) => HttpResponseBuilder::new(status_code).json(err),
    }
}

#[delete("/detail/{id}")]
pub async fn delete_detail(db: Data<DetailRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().json("Invalid ID");
    };
    let result = db.delete_record(&id).await;

    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return HttpResponse::Ok().json("Detail successfully deleted!");
            } else {
                return HttpResponse::NotFound().json("Detail with specified ID not found!");
            }
        }
        Err((status_code, err)) => HttpResponseBuilder::new(status_code).json(err),
    }
}

#[get("/details")]
pub async fn get_all_detail(db: Data<DetailRepo>) -> HttpResponse {
    let result = db.get_all_record().await;

    match result {
        Ok(details) => HttpResponse::Ok().json(details),
        Err((status_code, err)) => HttpResponseBuilder::new(status_code).json(err),
    }
}
