mod controller;
mod model;
mod repository;

use actix_web::{web::Data, App, HttpServer};
use controller::detail_controller;
use repository::detail_repo::DetailRepo;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // work around for unstable async closure
    let db = DetailRepo::init().await;
    let db_data = Data::new(db);
    HttpServer::new(move || App::new().service(detail_controller::new(db_data.clone())))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
