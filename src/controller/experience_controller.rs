use crate::{
    controller::crud_controller,
    model::experience_model::{Experience, ExperienceUpdate},
    repository::mongodb_repo::MongoDB,
};
use actix_web::{
    delete, get, post, put,
    web::{self, Data, Json, Path},
    HttpResponse, Scope,
};
// use log::info;

pub fn new() -> Scope {
    // info!("Initializing experience controller...");
    web::scope("/experience")
        .service(create_detail)
        .service(get_all_detail)
        .service(get_detail)
        .service(update_detail)
        .service(delete_detail)
}

#[post("")]
pub async fn create_detail(
    db: Data<MongoDB<Experience>>,
    new_detail: Json<Experience>,
) -> HttpResponse {
    let data = Experience {
        id: None,
        role: new_detail.role.to_owned(),
        company: new_detail.company.to_owned(),
        description: new_detail.description.to_owned(),
        start: new_detail.start.to_owned(),
        end: new_detail.end.to_owned(),
        tech_stack: new_detail.tech_stack.to_owned(),
    };
    crud_controller::create_detail(db, data).await
}

#[get("")]
pub async fn get_all_detail(db: Data<MongoDB<Experience>>) -> HttpResponse {
    crud_controller::get_all_detail(db).await
}

#[get("/{id}")]
pub async fn get_detail(db: Data<MongoDB<Experience>>, path: Path<String>) -> HttpResponse {
    crud_controller::get_detail(db, path).await
}

#[put("/{id}")]
pub async fn update_detail(
    db: Data<MongoDB<Experience>>,
    path: Path<String>,
    new_detail: Json<ExperienceUpdate>,
) -> HttpResponse {
    crud_controller::update_detail(db, path, new_detail).await
}

#[delete("/{id}")]
pub async fn delete_detail(db: Data<MongoDB<Experience>>, path: Path<String>) -> HttpResponse {
    crud_controller::delete_detail(db, path).await
}
