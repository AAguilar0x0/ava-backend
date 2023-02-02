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
        .service(create_project)
        .service(get_all_project)
        .service(get_project)
        .service(update_project)
        .service(delete_project)
}

#[post("")]
pub async fn create_project(
    db: Data<MongoDB<Project>>,
    new_project: Json<Project>,
) -> HttpResponse {
    let data = Project {
        _id: None,
        name: new_project.name.to_owned(),
        description: new_project.description.to_owned(),
        repo: new_project.repo.to_owned(),
        url: new_project.url.to_owned(),
        tech_stack: new_project.tech_stack.to_owned(),
    };
    crud_controller::create(db, data).await
}

#[get("")]
pub async fn get_all_project(db: Data<MongoDB<Project>>) -> HttpResponse {
    crud_controller::get_all(db).await
}

#[get("/{id}")]
pub async fn get_project(db: Data<MongoDB<Project>>, path: Path<String>) -> HttpResponse {
    crud_controller::get(db, path).await
}

#[put("/{id}")]
pub async fn update_project(
    db: Data<MongoDB<Project>>,
    path: Path<String>,
    new_project: Json<ProjectUpdate>,
) -> HttpResponse {
    crud_controller::update(db, path, new_project).await
}

#[delete("/{id}")]
pub async fn delete_project(db: Data<MongoDB<Project>>, path: Path<String>) -> HttpResponse {
    crud_controller::delete(db, path).await
}
