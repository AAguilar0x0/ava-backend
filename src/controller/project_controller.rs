use crate::{
    controller::crud_controller,
    model::project_model::{Project, ProjectUpdate},
    repository::mongodb_repo::MongoDB,
};
use actix_web::{
    delete, get, post, put,
    web::{self, Data, Json, Path},
    HttpResponse, Scope,
};

pub fn new() -> Scope {
    web::scope("/projects")
        .service(create_detail)
        .service(get_all_detail)
        .service(get_detail)
        .service(update_detail)
        .service(delete_detail)
}

#[post("")]
pub async fn create_detail(db: Data<MongoDB<Project>>, new_detail: Json<Project>) -> HttpResponse {
    let data = Project {
        _id: None,
        name: new_detail.name.to_owned(),
        company: new_detail.company.to_owned(),
        repo: new_detail.repo.to_owned(),
        url: new_detail.url.to_owned(),
        tech_stack: new_detail.tech_stack.to_owned(),
    };
    crud_controller::create(db, data).await
}

#[get("")]
pub async fn get_all_detail(db: Data<MongoDB<Project>>) -> HttpResponse {
    crud_controller::get_all(db).await
}

#[get("/{id}")]
pub async fn get_detail(db: Data<MongoDB<Project>>, path: Path<String>) -> HttpResponse {
    crud_controller::get(db, path).await
}

#[put("/{id}")]
pub async fn update_detail(
    db: Data<MongoDB<Project>>,
    path: Path<String>,
    new_detail: Json<ProjectUpdate>,
) -> HttpResponse {
    crud_controller::update(db, path, new_detail).await
}

#[delete("/{id}")]
pub async fn delete_detail(db: Data<MongoDB<Project>>, path: Path<String>) -> HttpResponse {
    crud_controller::delete(db, path).await
}
