/* Date Created: 11/10/2023. */

//! Web application entry function.

use dotenv::dotenv;
use sqlx::{Pool, MySql};
use async_std::task;
use actix_web::{http::header, web, App, HttpServer};
use actix_cors::Cors;

mod config;
mod database;
mod utils;
mod models;
mod handlers;

mod middleware;

pub struct AppState {
    db: Pool<MySql>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let config = config::Config::init();

    let pool = task::block_on(database::get_mysql_pool(config.max_connections, &config.database_url));

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&config.allowed_origin)
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .max_age(config.max_age)
            .supports_credentials();

        App::new()
            .app_data(web::Data::new(AppState {
                db: pool.clone()
            }))
            .wrap(cors)
            .service(
                web::scope("/data")
                    .service(handlers::employees_json1)
                    .service(handlers::employees_json2),
            )
            .service(
                web::scope("/ui")
                    .service(handlers::employees_html1)
                    .service(handlers::employees_html2),
            )
            .service(
                web::resource("/helloemployee/{last_name}/{first_name}")
                    .wrap(middleware::SayHi)
                    .route(web::get().to(handlers::hi_first_employee_found))
            )
    })
    .bind(("0.0.0.0", 5000))?
    .run()
    .await
}
