/* Date Created: 27/12/2023. */

//! The application HTTP server, call by the application ``main()`` function, 
//! as well as integration test methods.

use std::{fs::File, io::Read as _,};
use std::net::TcpListener;
use dotenv::dotenv;
use sqlx::{Pool, MySql};
use actix_web::{
    dev::Server, error, cookie::{Key, SameSite}, 
    http::{header, StatusCode}, web, App, HttpServer
};
use openssl::{pkey::{PKey, Private}, ssl::{SslAcceptorBuilder, SslAcceptor, SslMethod},};
use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use actix_identity::IdentityMiddleware;
use actix_cors::Cors;

pub mod bh_libs;
pub mod helper;

pub mod config;
pub mod database;
pub mod models;
pub mod handlers;

pub mod middleware;

pub mod auth_middleware;
pub mod auth_handlers;

use crate::helper::app_utils::make_api_status_response;

pub struct AppState {
    db: Pool<MySql>
}

/// See https://github.com/actix/examples/tree/master/https-tls/openssl
/// 
fn load_encrypted_private_key() -> PKey<Private> {
    // let mut file = File::open("key.pem").unwrap();
    let mut file = File::open("./cert/key-pass.pem").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read file");

    PKey::private_key_from_pem_passphrase(&buffer, b"I am installing SSL").unwrap()
}

/// See https://github.com/actix/examples/tree/master/https-tls/openssl
/// 
fn ssl_builder() -> SslAcceptorBuilder {
    // build TLS config from files
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();

    // set the encrypted private key
    builder
        .set_private_key(&load_encrypted_private_key())
        .unwrap();

    builder.set_certificate_chain_file("./cert/cert-pass.pem").unwrap();

    builder
}

/// A global error handler for ``application/json`` data extractor,
/// [`actix_web::types::json`].
/// 
/// Basically, malformed submitted data would result in a JSON response, which is a
/// serialised [`crate::bh_libs::api_status::ApiStatus`] with ``code`` of ``400`` for 
/// [`actix_web::http::StatusCode::BAD_REQUEST`], ``message`` is the actual deserialised
/// error message.
/// 
fn json_config() -> web::JsonConfig {
    // custom `Json` extractor configuration
    web::JsonConfig::default()
        // limit request payload size
        .limit(4096)
        // only accept application/json content type
        .content_type(|mime| mime == mime::APPLICATION_JSON)
        // use custom error handler
        .error_handler(|err, _req| {
            let err_str: String = String::from(err.to_string());
            error::InternalError::from_response(err, 
                make_api_status_response(StatusCode::BAD_REQUEST, &err_str, None)).into()
        })
}

/// A global error handler for ``application/x-www-form-urlencoded`` data extractor,
/// [`actix_web::types::form`].
/// 
/// Basically, malformed submitted data would result in a JSON response, which is a
/// serialised [`crate::bh_libs::api_status::ApiStatus`] with ``code`` of ``400`` for 
/// [`actix_web::http::StatusCode::BAD_REQUEST`], ``message`` is the actual deserialised
/// error message.
/// 
fn form_config() -> web::FormConfig {
    web::FormConfig::default()
        // limit request payload size
        .limit(4096)
        // use custom error handler
        .error_handler(|err, _req| {
            let err_str: String = String::from(err.to_string());
            error::InternalError::from_response(err, 
                make_api_status_response(StatusCode::BAD_REQUEST, &err_str, None)).into()
        })
}

/// The application HTTP server.
/// 
/// # Return
/// 
/// - [`core::result::Result`]. On successful [`actix_web::dev::Server`]. On failure 
/// [`std::io::Error`].
/// 
pub async fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    dotenv().ok();
    let config = config::Config::init();

    let pool = database::get_mysql_pool(config.max_connections, &config.database_url).await;

    let secret_key = Key::generate();
    let redis_store = RedisSessionStore::new("redis://127.0.0.1:6379")
        .await
        .unwrap();

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
            .app_data(json_config())
            .app_data(form_config())
            .wrap(auth_middleware::CheckLogin)
            .wrap(IdentityMiddleware::default())
            .wrap(SessionMiddleware::builder(
                    redis_store.clone(),
                    secret_key.clone()
                )
                .cookie_secure(true)
                .cookie_same_site(SameSite::None)
                .build(),
            )
            .wrap(cors)
            .service(
                web::scope("/data")
                    .service(handlers::employees_json1)
                    .service(handlers::employees_json2),
            )
            .service(
                web::scope("/ui")
                    .service(handlers::employees_html1)
                    .service(handlers::employees_html2)
                    .service(auth_handlers::login_page)
                    .service(auth_handlers::home_page),
            )
            .service(
                web::scope("/api")
                    .service(auth_handlers::login)
                    .service(auth_handlers::logout),
            )
            .service(
                web::resource("/helloemployee/{last_name}/{first_name}")
                    .wrap(middleware::SayHi)
                    .route(web::get().to(handlers::hi_first_employee_found))
            )
    })
    // .listen(listener)?
    .listen_openssl(listener, ssl_builder())?
    .run();

    Ok(server)
}