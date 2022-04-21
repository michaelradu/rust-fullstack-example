mod api;
mod repository;

use actix_web::{middleware::Logger, web::scope, web::Data, App, HttpServer};
use actix_web_lab::web::spa;
use api::task::{complete_task, fail_task, get_task, pause_task, start_task, submit_task};
use repository::ddb::DDBRepository;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let config = aws_config::load_from_env().await;
    HttpServer::new(move || {
        let ddb_repo: DDBRepository = DDBRepository::init(String::from("task"), config.clone());
        let ddb_data = Data::new(ddb_repo);
        let logger = Logger::default();

        App::new()
            .wrap(logger)
            .app_data(ddb_data)
            .service(
                scope("/api")
                    .service(get_task)
                    .service(submit_task)
                    .service(start_task)
                    .service(complete_task)
                    .service(pause_task)
                    .service(fail_task),
            )
            .service(
                spa()
                    .index_file("./dist/index.html")
                    .static_resources_mount("/")
                    .static_resources_location("./dist")
                    .finish(),
            )
    })
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}
