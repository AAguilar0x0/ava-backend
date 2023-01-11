use crate::{model::tech_stack_model::TechStack, repository::mongodb_repo::MongoDB};
use actix_web::{
    delete, get, post, put,
    web::{self, Data, Json, Path},
    HttpResponse, HttpResponseBuilder, Scope,
};
use mongodb::bson::{doc, oid::ObjectId};

pub fn new() -> Scope {
    web::scope("/tech_stack")
        .service(create_detail)
        .service(get_all_detail)
        .service(get_detail)
        .service(update_detail)
        .service(delete_detail)
}

#[post("")]
pub async fn create_detail(
    db: Data<MongoDB<TechStack>>,
    new_detail: Json<TechStack>,
) -> HttpResponse {
    let data = TechStack {
        id: None,
        name: new_detail.name.to_owned(),
        category: new_detail.category.to_owned(),
    };

    let result = db.create_record(data).await;

    match result {
        Ok(detail) => HttpResponse::Ok().json(detail),
        Err((status_code, err)) => HttpResponseBuilder::new(status_code).json(err),
    }
}

#[get("")]
pub async fn get_all_detail(db: Data<MongoDB<TechStack>>) -> HttpResponse {
    let result = db.get_all_record().await;

    match result {
        Ok(details) => HttpResponse::Ok().json(details),
        Err((status_code, err)) => HttpResponseBuilder::new(status_code).json(err),
    }
}

#[get("/{id}")]
pub async fn get_detail(db: Data<MongoDB<TechStack>>, path: Path<String>) -> HttpResponse {
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

#[put("/{id}")]
pub async fn update_detail(
    db: Data<MongoDB<TechStack>>,
    path: Path<String>,
    new_detail: Json<TechStack>,
) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().json("Invalid ID");
    };
    let data = TechStack {
        id: Some(match ObjectId::parse_str(&id) {
            Ok(id) => id,
            Err(_) => return HttpResponse::BadRequest().json("Invalid ID".to_owned()),
        }),
        name: new_detail.name.to_owned(),
        category: new_detail.category.to_owned(),
    };

    let filter = doc! {"_id": data.id};
    let new_doc = doc! {
        "$set":
            {
                "id": data.id,
                "name": data.name,
                "category": data.category,
            },
    };
    let result = db.update_record(filter, new_doc).await;

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

#[delete("/{id}")]
pub async fn delete_detail(db: Data<MongoDB<TechStack>>, path: Path<String>) -> HttpResponse {
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
