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

pub fn new() -> Scope {
    web::scope("/experiences")
        .service(create_experience)
        .service(get_all_experience)
        .service(get_experience)
        .service(update_experience)
        .service(delete_experience)
}

#[post("")]
pub async fn create_experience(
    db: Data<MongoDB<Experience>>,
    new_experience: Json<Experience>,
) -> HttpResponse {
    let data = Experience {
        _id: None,
        role: new_experience.role.to_owned(),
        company: new_experience.company.to_owned(),
        description: new_experience.description.to_owned(),
        start: new_experience.start.to_owned(),
        end: new_experience.end.to_owned(),
        tech_stack: new_experience.tech_stack.to_owned(),
    };
    crud_controller::create(db, data).await
}

#[get("")]
pub async fn get_all_experience(db: Data<MongoDB<Experience>>) -> HttpResponse {
    crud_controller::get_all(db).await
}

#[get("/{id}")]
pub async fn get_experience(db: Data<MongoDB<Experience>>, path: Path<String>) -> HttpResponse {
    crud_controller::get(db, path).await
}

#[put("/{id}")]
pub async fn update_experience(
    db: Data<MongoDB<Experience>>,
    path: Path<String>,
    new_experience: Json<ExperienceUpdate>,
) -> HttpResponse {
    crud_controller::update(db, path, new_experience).await
}

#[delete("/{id}")]
pub async fn delete_experience(db: Data<MongoDB<Experience>>, path: Path<String>) -> HttpResponse {
    crud_controller::delete(db, path).await
}
