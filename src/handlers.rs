/* Date Created: 16/10/2023. */

//! Application HTTP request handlers.

use async_std::task;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};

use tera::{Context, Tera};

use crate::models::{self, Employee};
use models::{get_employees, EmployeeSearch};

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
#[post("/employees")]
pub async fn employees_json1(
    _: HttpRequest,
    app_state: web::Data<super::AppState>,
    body: web::Json<EmployeeSearch>
) -> impl Responder {
    let query_result = task::block_on(get_employees(&app_state.db, &body.last_name, &body.first_name));
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
#[get("/employees/{last_name}/{first_name}")]
pub async fn employees_json2(
    req: HttpRequest,
    app_state: web::Data<super::AppState>,
) -> impl Responder {
    let last_name: String = req.match_info().get("last_name").unwrap().parse::<String>().unwrap();
    let first_name: String = req.match_info().get("first_name").unwrap().parse::<String>().unwrap();

    let query_result = task::block_on(get_employees(&app_state.db, &last_name, &first_name));
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
#[post("/employees")]
pub async fn employees_html1(
    _: HttpRequest,
    app_state: web::Data<super::AppState>,
    body: web::Form<EmployeeSearch>
) -> impl Responder {
    let query_result = task::block_on(get_employees(&app_state.db, &body.last_name, &body.first_name));

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
#[get("/employees/{last_name}/{first_name}")]
pub async fn employees_html2(
    req: HttpRequest,
    app_state: web::Data<super::AppState>,
) -> impl Responder {
    let last_name: String = req.match_info().get("last_name").unwrap().parse::<String>().unwrap();
    let first_name: String = req.match_info().get("first_name").unwrap().parse::<String>().unwrap();

    let query_result = task::block_on(get_employees(&app_state.db, &last_name, &first_name));

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(render_employees_template(&query_result))    
}
