use crate::{
    controller::crud_controller,
    model::detail_model::{Detail, DetailUpdate},
    repository::mongodb_repo::MongoDB,
};
use actix_web::{
    delete, get, post, put,
    web::{self, Data, Json, Path},
    HttpResponse, Scope,
};

pub fn new() -> Scope {
    web::scope("/detail")
        .service(create_detail)
        .service(get_all_detail)
        .service(get_detail)
        .service(update_detail)
        .service(delete_detail)
}

#[post("")]
pub async fn create_detail(db: Data<MongoDB<Detail>>, new_detail: Json<Detail>) -> HttpResponse {
    let data = Detail {
        id: None,
        name: new_detail.name.to_owned(),
        description: new_detail.description.to_owned(),
        image: new_detail.image.to_owned(),
    };
    crud_controller::create_detail(db, data).await
}

#[get("")]
pub async fn get_all_detail(db: Data<MongoDB<Detail>>) -> HttpResponse {
    crud_controller::get_all_detail(db).await
}

#[get("/{id}")]
pub async fn get_detail(db: Data<MongoDB<Detail>>, path: Path<String>) -> HttpResponse {
    crud_controller::get_detail(db, path).await
}

#[put("/{id}")]
pub async fn update_detail(
    db: Data<MongoDB<Detail>>,
    path: Path<String>,
    new_detail: Json<DetailUpdate>,
) -> HttpResponse {
    crud_controller::update_detail(db, path, new_detail).await
}

#[delete("/{id}")]
pub async fn delete_detail(db: Data<MongoDB<Detail>>, path: Path<String>) -> HttpResponse {
    crud_controller::delete_detail(db, path).await
}
