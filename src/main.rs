mod controller;
mod model;
mod repository;

use actix_web::{web::Data, App, HttpServer};
use controller::{detail_controller, tech_stack_controller};
use model::{detail_model::Detail, tech_stack_model::TechStack};
use repository::mongodb_repo::MongoDB;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let detail_db_data = Data::new(MongoDB::<Detail>::init("Detail").await);
    let tech_stack_db_data = Data::new(MongoDB::<TechStack>::init("TechStack").await);
    HttpServer::new(move || {
        App::new()
            .app_data(detail_db_data.clone())
            .app_data(tech_stack_db_data.clone())
            .service(detail_controller::new())
            .service(tech_stack_controller::new())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
