mod controller;
mod model;
mod repository;

use actix_web::{
    middleware::Logger,
    web::{self, Data},
    App, HttpServer,
};
use controller::{
    detail_controller, experience_controller, project_controller, tech_stack_controller,
    user_controller,
};
use dotenv::dotenv;
use env_logger::Env;
use log::info;
use model::{
    detail_model::Detail, experience_model::Experience, project_model::Project,
    tech_stack_model::TechStack, user_model::User,
};
use repository::mongodb_repo::{new, MongoDB};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    info!("Initializing database...");
    let mut db = new("ava").await;
    let detail_db_data = Data::new(MongoDB::<Detail>::init(&mut db, "Detail").await);
    let tech_stack_db_data = Data::new(MongoDB::<TechStack>::init(&mut db, "TechStack").await);
    let project_db_data = Data::new(MongoDB::<Project>::init(&mut db, "Project").await);
    let experience_db_data = Data::new(MongoDB::<Experience>::init(&mut db, "Experience").await);
    let user_db_data = Data::new(MongoDB::<User>::init(&mut db, "User").await);
    info!("Starting server...");
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(detail_db_data.clone())
            .app_data(tech_stack_db_data.clone())
            .app_data(project_db_data.clone())
            .app_data(experience_db_data.clone())
            .app_data(user_db_data.clone())
            .service(
                web::scope("/api")
                    .service(detail_controller::new())
                    .service(tech_stack_controller::new())
                    .service(project_controller::new())
                    .service(experience_controller::new())
                    .service(user_controller::new()),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
