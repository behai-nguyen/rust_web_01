/* Date Created: 16/10/2023. */

//! Application HTTP request handlers.

use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder, web::ReqData};

use tera::{Context, Tera};

use crate::models::{self, Employee};
use models::{get_employees, EmployeeSearch};

use crate::middleware::Msg;

/// Attempts to retrieve employee records based on partial last name and partial 
/// first name, then returns matched records as JSON. Calls to [`get_employees`]
/// method to do database work.
/// 
/// # Arguments
/// 
/// * `-` - un-used.
/// 
/// * `app_state` - [Application state](https://actix.rs/docs/application/#state). 
/// This's where the application MySQL database connection pool is stored.
/// This MySQL database connection pool is used to query data.
/// 
/// * `body` - Effectively the submitted JSON [`actix_web::HttpRequest`] which 
/// has been deserialised to struct [`models::EmployeeSearch`].
/// 
/// # Usage Example
/// 
/// * Route: ``http://localhost:5000/data/employees``
/// * Method: ``POST``
/// * Content Type: ``application/json``
/// * Body: ``{"last_name": "%chi", "first_name": "%ak"}``
/// 
/// # Return
/// 
/// * Success: JSON. Failure: JSON.
/// 
/// * Response status code: looks for [OK](`actix_web::http::StatusCode::OK`),
/// [BAD_REQUEST](`actix_web::http::StatusCode::BAD_REQUEST`) or 
/// [UNAUTHORIZED](`actix_web::http::StatusCode::UNAUTHORIZED`).
/// 
/// * Response status code of [OK](`actix_web::http::StatusCode::OK`) is a successful
/// response. The actual response is a JSON array of serialised [Employee](`crate::models::Employee`).
/// 
/// * Response status codes of the later two indicate a failure response. The actual 
/// response is a JSON serialised of [ApiStatus](`crate::bh_libs::api_status::ApiStatus`).
/// 
/// # TO_DO
/// 
/// For successful responses, make [ApiStatus](`crate::bh_libs::api_status::ApiStatus`) 
/// the header part of the response, the JSON array mentioned is ``data`` field.
/// 
#[post("/employees")]
pub async fn employees_json1(
    _: HttpRequest,
    app_state: web::Data<super::AppState>,
    body: web::Json<EmployeeSearch>
) -> impl Responder {
    let query_result = get_employees(&app_state.db, &body.last_name, &body.first_name).await;
    web::Json(query_result)
}

/// Attempts to retrieve employee records based on partial last name and partial 
/// first name, then returns matched records as JSON. Calls to [`get_employees`]
/// method to do database work.
/// 
/// # Arguments
/// 
/// * `req` - Submitted request, where URL contains path information which are partial
/// last name and partial first name.
/// 
/// * `app_state` - [Application state](https://actix.rs/docs/application/#state). 
/// This's where the application MySQL database connection pool is stored.
/// This MySQL database connection pool is used to query data.
/// 
/// # Usage Example
/// 
/// * Route: ``http://localhost:5000/data/employees/%chi/%ak``
/// * Method: ``GET``
/// 
/// # Return
/// 
/// * Success: JSON. Failure: JSON.
/// 
/// * Response status code: looks for [OK](`actix_web::http::StatusCode::OK`),
/// [UNAUTHORIZED](`actix_web::http::StatusCode::UNAUTHORIZED`).
/// 
/// * Response status code of [OK](`actix_web::http::StatusCode::OK`) is a successful
/// response. The actual response is a JSON array of serialised [Employee](`crate::models::Employee`).
/// 
/// * Response status code of [UNAUTHORIZED](`actix_web::http::StatusCode::UNAUTHORIZED`) 
/// indicates a failure response. The actual response is a JSON serialised of 
/// [ApiStatus](`crate::bh_libs::api_status::ApiStatus`).
/// 
/// # TO_DO
/// 
/// For successful responses, make [ApiStatus](`crate::bh_libs::api_status::ApiStatus`) 
/// the header part of the response, the JSON array mentioned is ``data`` field.
/// 
#[get("/employees/{last_name}/{first_name}")]
pub async fn employees_json2(
    req: HttpRequest,
    app_state: web::Data<super::AppState>,
) -> impl Responder {
    let last_name: String = req.match_info().get("last_name").unwrap().parse::<String>().unwrap();
    let first_name: String = req.match_info().get("first_name").unwrap().parse::<String>().unwrap();

    let query_result = get_employees(&app_state.db, &last_name, &first_name).await;
    web::Json(query_result)
}

/// Generates a complete HTML page with retrieved data from the ``employees`` table,
/// based on ``templates/employees.html`` template.
/// 
/// # Arguments
/// 
/// * `employees` - List of retrieved ``employees`` in JSON format. Column values are
/// rendered as are.
/// 
/// # Return
/// 
/// - HTML string.
/// 
fn render_employees_template(employees: &Vec<Employee>) -> String {
    // Create a new Tera instance and add a template from a string
    let tera = Tera::new("templates/**/*").unwrap();

    let mut ctx = Context::new();

    // Passing data to be rendered to the template engine.
    ctx.insert("employees", employees);

    tera.render("employees.html", &ctx).expect("Failed to render template")
}

/// Attempts to retrieve employee records based on partial last name and partial 
/// first name, then returns matched records as a complete HTML page. Calls to 
/// [`get_employees`] method to do database work.
/// 
/// # Arguments
/// 
/// * `-` - un-used.
/// 
/// * `app_state` - [Application state](https://actix.rs/docs/application/#state). 
/// This's where the application MySQL database connection pool is stored.
/// This MySQL database connection pool is used to query data.
/// 
/// * `body` - Effectively the submitted ``application/x-www-form-urlencoded; charset=UTF-8`` 
/// [`actix_web::HttpRequest`] which has been deserialised to struct [`models::EmployeeSearch`].
/// 
/// # Usage Example
/// 
/// * Route: ``http://localhost:5000/ui/employees``
/// * Method: ``POST``
/// * Content Type: ``application/x-www-form-urlencoded; charset=UTF-8``
/// * Body: ``last_name=%chi&first_name=%ak``
/// 
/// # Return
/// 
/// * Success: HTML. Failure: JSON.
/// 
/// * Response status code: looks for [OK](`actix_web::http::StatusCode::OK`),
/// [BAD_REQUEST](`actix_web::http::StatusCode::BAD_REQUEST`) or 
/// [UNAUTHORIZED](`actix_web::http::StatusCode::UNAUTHORIZED`).
/// 
/// * Response status code of [OK](`actix_web::http::StatusCode::OK`) is a successful
/// response. The actual response is a HTML rendered of [Employee](`crate::models::Employee`).
/// 
/// * Response status codes of the later two indicate a failure response. The actual 
/// response is a JSON serialised of [ApiStatus](`crate::bh_libs::api_status::ApiStatus`).
/// 
#[post("/employees")]
pub async fn employees_html1(
    _: HttpRequest,
    app_state: web::Data<super::AppState>,
    body: web::Form<EmployeeSearch>
) -> impl Responder {
    let query_result = get_employees(&app_state.db, &body.last_name, &body.first_name).await;

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(render_employees_template(&query_result))    
}

/// Attempts to retrieve employee records based on partial last name and partial 
/// first name, then returns matched records as a complete HTML page. Calls to 
/// [`get_employees`] method to do database work.
/// 
/// # Arguments
/// 
/// * `req` - Submitted request, where URL contains path information which are partial
/// last name and partial first name.
/// 
/// * `app_state` - [Application state](https://actix.rs/docs/application/#state). 
/// This's where the application MySQL database connection pool is stored.
/// This MySQL database connection pool is used to query data.
/// 
/// # Usage Example
/// 
/// * Route: ``http://localhost:5000/ui/employees/%chi/%ak``
/// * Method: ``GET``
/// 
/// # Return
/// 
/// * Success: HTML. Failure: JSON.
/// 
/// * Response status code: looks for [OK](`actix_web::http::StatusCode::OK`) 
/// or [UNAUTHORIZED](`actix_web::http::StatusCode::UNAUTHORIZED`).
/// 
/// * Response status code of [OK](`actix_web::http::StatusCode::OK`) is a successful
/// response. The actual response is a HTML rendered of [Employee](`crate::models::Employee`).
/// 
/// * Response status code of [UNAUTHORIZED](`actix_web::http::StatusCode::UNAUTHORIZED`) 
/// indicates a failure response. The actual response is a JSON serialised of 
/// [ApiStatus](`crate::bh_libs::api_status::ApiStatus`).
/// 
#[get("/employees/{last_name}/{first_name}")]
pub async fn employees_html2(
    req: HttpRequest,
    app_state: web::Data<super::AppState>,
) -> impl Responder {
    let last_name: String = req.match_info().get("last_name").unwrap().parse::<String>().unwrap();
    let first_name: String = req.match_info().get("first_name").unwrap().parse::<String>().unwrap();

    let query_result = get_employees(&app_state.db, &last_name, &first_name).await;

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(render_employees_template(&query_result))
}

/// [SayHi](`super::middleware::SayHi`) middleware resource endpoint handler.
/// Extracts the string message attached to the request extension, if found, returns 
/// the message as HTML response. If no message found Otherwise returns HTTP 500 with
/// text ``"No message found."``.
///
/// # Arguments
/// 
/// * `msg` - [Request-local data extractor](https://docs.rs/actix-web/latest/actix_web/web/struct.ReqData.html).
/// Arbitrary data attached to an individual request by the ``SayHi`` middleware.
/// 
/// # Usage Example
/// 
/// * Route: ``http://localhost:5000/helloemployee/%chi/%ak``
/// * Route: ``http://localhost:5000/helloemployee/%xxx/%xxx``
/// * Method: ``GET``
/// 
/// # To trigger HTTP 500:
/// 
/// * In the ``middleware.rs`` module, under the service trait, in
/// ``fn call(&self, req: ServiceRequest) -> Self::Future {``, comment out
/// ``req.extensions_mut().insert(Msg(hello_msg.to_owned()));``. Recompile,
/// then run with either of the examples listed above. We should get HTTP 500.
///
/// # Return
/// 
/// * Success: HTML. Failure: JSON.
/// 
/// * Response status code: looks for [OK](`actix_web::http::StatusCode::OK`) 
/// or [UNAUTHORIZED](`actix_web::http::StatusCode::UNAUTHORIZED`).
/// 
/// * Response status code of [OK](`actix_web::http::StatusCode::OK`) is a successful
/// response. The actual response is a HTML rendered of an [Employee](`crate::models::Employee`).
/// 
/// * Response status codes of [UNAUTHORIZED](`actix_web::http::StatusCode::UNAUTHORIZED`) 
/// indicates a failure response. The actual response is a JSON serialised of 
/// [ApiStatus](`crate::bh_libs::api_status::ApiStatus`).
/// 
pub async fn hi_first_employee_found(msg: Option<ReqData<Msg>>) -> impl Responder {
    tracing::debug!("I am pub async fn hi_first_employee_found(...)");
    match msg {
        None => return HttpResponse::InternalServerError().body("No message found."),

        Some(msg_data) => {
            let Msg(message) = msg_data.into_inner();

            HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(format!("<h1>{}</h1>", message))    
        },
    }
}
