use crate::{
    controller::crud_controller,
    model::tech_stack_model::{TechStack, TechStackUpdate},
    repository::mongodb_repo::MongoDB,
};
use actix_web::{
    delete, get, post, put,
    web::{self, Data, Json, Path},
    HttpResponse, Scope,
};

pub fn new() -> Scope {
    web::scope("/tech-stack")
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
        _id: None,
        name: new_detail.name.to_owned(),
        category: new_detail.category.to_owned(),
    };
    crud_controller::create_detail(db, data).await
}

#[get("")]
pub async fn get_all_detail(db: Data<MongoDB<TechStack>>) -> HttpResponse {
    crud_controller::get_all_detail(db).await
}

#[get("/{id}")]
pub async fn get_detail(db: Data<MongoDB<TechStack>>, path: Path<String>) -> HttpResponse {
    crud_controller::get_detail(db, path).await
}

#[put("/{id}")]
pub async fn update_detail(
    db: Data<MongoDB<TechStack>>,
    path: Path<String>,
    new_detail: Json<TechStackUpdate>,
) -> HttpResponse {
    crud_controller::update_detail(db, path, new_detail).await
}

#[delete("/{id}")]
pub async fn delete_detail(db: Data<MongoDB<TechStack>>, path: Path<String>) -> HttpResponse {
    crud_controller::delete_detail(db, path).await
}
