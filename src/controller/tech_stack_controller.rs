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
        .service(create_tech_stack)
        .service(get_all_tech_stack)
        .service(get_tech_stack)
        .service(update_tech_stack)
        .service(delete_tech_stack)
}

#[post("")]
pub async fn create_tech_stack(
    db: Data<MongoDB<TechStack>>,
    new_tech_stack: Json<TechStack>,
) -> HttpResponse {
    let data = TechStack {
        _id: None,
        name: new_tech_stack.name.to_owned(),
        category: new_tech_stack.category.to_owned(),
    };
    crud_controller::create(db, data).await
}

#[get("")]
pub async fn get_all_tech_stack(db: Data<MongoDB<TechStack>>) -> HttpResponse {
    crud_controller::get_all(db).await
}

#[get("/{id}")]
pub async fn get_tech_stack(db: Data<MongoDB<TechStack>>, path: Path<String>) -> HttpResponse {
    crud_controller::get(db, path).await
}

#[put("/{id}")]
pub async fn update_tech_stack(
    db: Data<MongoDB<TechStack>>,
    path: Path<String>,
    new_tech_stack: Json<TechStackUpdate>,
) -> HttpResponse {
    crud_controller::update(db, path, new_tech_stack).await
}

#[delete("/{id}")]
pub async fn delete_tech_stack(db: Data<MongoDB<TechStack>>, path: Path<String>) -> HttpResponse {
    crud_controller::delete(db, path).await
}
