/* Date Created: 27/12/2023. */

//! The application HTTP server, call by the application ``main()`` function, 
//! as well as integration test methods.

use std::net::TcpListener;
use dotenv::dotenv;
use sqlx::{Pool, MySql};
use async_std::task;
use actix_web::{http::header, web, App, HttpServer};
use actix_web::dev::Server;
use actix_cors::Cors;

pub mod config;
pub mod database;
pub mod utils;
pub mod models;
pub mod handlers;

pub mod middleware;

pub struct AppState {
    db: Pool<MySql>,
}

/// The application HTTP server.
/// 
/// # Return
/// 
/// - [`core::result::Result`]. On successful [`actix_web::dev::Server`]. On failure 
/// [`std::io::Error`].
/// 
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    dotenv().ok();
    let config = config::Config::init();

    let pool = task::block_on(database::get_mysql_pool(config.max_connections, &config.database_url));

    let server = HttpServer::new(move || {
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
    .listen(listener)?
    .run();

    Ok(server)
}
