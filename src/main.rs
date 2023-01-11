mod controller;
mod model;
mod repository;

use actix_web::{web::Data, App, HttpServer};
use controller::detail_controller;
use model::detail_model::Detail;
use repository::mongodb_repo::MongoDB;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let detail_db = MongoDB::<Detail>::init("Detail").await;
    let db_data = Data::new(detail_db);
    HttpServer::new(move || App::new().service(detail_controller::new(db_data.clone())))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
