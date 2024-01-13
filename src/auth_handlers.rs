/* Date Created: 01/12/2023. */

//! Application authentication-related request handlers. Responsible for 
//! serving login page, managing login request, serving home page, etc.

use tera::{Context, Tera};

use actix_web::{get, post, HttpMessage, HttpRequest, HttpResponse, Responder};
use actix_web::http::header::ContentType;
use actix_web::web::Bytes;

use crate::helper::messages::REQUEST_BODY_EMPTY_MSG;
use crate::helper::endpoint::{err_code_500, extract_employee_login};
use crate::bh_libs::api_status::ApiStatus;
use crate::models::{EmployeeLogin, LoginSuccess};

/// Renders the login page and return the complete content as a 
/// [`std::string::String`].
fn render_login_page() -> String {
    // Create a new Tera instance and add a template from a string
    let tera = Tera::new("templates/auth/**/*").unwrap();

    let ctx = Context::new();

    tera.render("login.html", &ctx).expect("Failed to render template")
}

/// Serves the login page as HTML.
/// 
/// * Route: ``http://localhost:5000/ui/login``
/// * Method: ``GET``
/// 
#[get("/login")]
pub async fn login_page(
) -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(render_login_page())
}

/// Manages actual login requests. 
/// 
/// *This current implementation does not do any login or authentication. Its sole 
/// purpose is to demonstrate accepting request body in both ``application/x-www-form-urlencoded``
/// and ``application/json`` content types.*
/// 
/// # Arguments
/// 
/// * `request` - Submitted request.
/// 
/// * `body` - [`actix_web::web::Bytes`], the actual submitted login data. 
/// It is either one of these two conent types: [`mime::APPLICATION_WWW_FORM_URLENCODED`] 
/// and [`mime::APPLICATION_JSON`].
/// 
/// # Response
/// 
/// **Work in progress.** This implementation is as follow:
/// 
/// * When failed deserialising, **always** returns a JSON serialised of 
/// [`crate::common::api_status::ApiStatus`].
/// 
/// * When succeeded deserialising:
/// 
///     - If the original request content type is ``application/x-www-form-urlencoded``,
///       then returns a HTML. The final implementation would return a complete home page.
/// 
///     - If the original request content type is ``application/json``, then returns a
///        JSON serialised of [`crate::models::LoginSuccess`].
/// 
/// # Valid Usage
/// 
/// * Route: ``http://localhost:5000/api/login``
/// * Method: ``POST``
/// * Content type: ``application/x-www-form-urlencoded``; 
/// request body: ``email=chirstian.koblick.10004@gmail.com&password=password``.
/// * Content type: ``application/json``; 
/// request body: ``{"email": "chirstian.koblick.10004@gmail.com", "password": "password"}``.
/// 
#[post("/login")]
pub async fn login(
    request: HttpRequest,
    body: Bytes
) -> HttpResponse {
    // No content. Returns an error.
    if body.len() == 0  {
        return HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(
                serde_json::to_string(&ApiStatus::new(err_code_500())
                    .set_text(REQUEST_BODY_EMPTY_MSG)).unwrap()
            );
    }

    // Attempts to extract -- deserialising -- request body into EmployeeLogin.
    let api_status = extract_employee_login(&body, request.content_type());
    // Failed to deserialise request body. Returns the error as is.
    if api_status.is_err() {
        return HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(serde_json::to_string(&api_status.err().unwrap()).unwrap());
    }

    // Succeeded to deserialise request body.
    let emp_login: EmployeeLogin = api_status.unwrap();

    // The request content type is "application/x-www-form-urlencoded", returns HTML content.
    // 
    // The final implementation would return the home page.
    if request.content_type() == ContentType::form_url_encoded().to_string() {
        return HttpResponse::Ok()
            .content_type(ContentType::html())
            .body(format!("<h1>Successfully login 🚀 email: {}, token: {}</h1>", &emp_login.email, &emp_login.email))
    }
    else {
        // The request content type is "application/json", returns a JSON content.
        // 
        // This implementation is probably very closed the final implementation: the token field
        // is the authentication token which the users need to include in the future requests
        // to get authenticated and hence access to protected resources.
        return HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(serde_json::to_string(
                &LoginSuccess { 
                    email: String::from(&emp_login.email), 
                    token: String::from(&emp_login.email) }
                ).unwrap())
    }
}
