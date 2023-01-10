mod controller;
mod model;
mod repository;

use actix_web::{web::Data, App, HttpServer};
use controller::detail_controller::{
    create_detail, delete_detail, get_all_detail, get_detail, update_detail,
};
use repository::detail_repo::DetailRepo;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = DetailRepo::init().await;
    let db_data = Data::new(db);
    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(create_detail)
            .service(get_detail)
            .service(update_detail)
            .service(delete_detail)
            .service(get_all_detail)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
