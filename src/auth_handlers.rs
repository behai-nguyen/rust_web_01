/* Date Created: 01/12/2023. */

//! Application authentication-related request handlers. Responsible for 
//! serving login page, managing login request, serving home page, etc.

use tera::{Context, Tera};
use actix_web::{
    HttpMessage, get, post, web, web::Bytes, HttpRequest, 
    HttpResponse, Responder, Either
};
use actix_web::http::{header, StatusCode, header::ContentType};

use actix_identity::Identity;

use argon2::{password_hash::{PasswordHash, PasswordVerifier}, Argon2};

use crate::helper::constants::{
    REDIRECT_MESSAGE,
    ORIGINAL_CONTENT_TYPE
};
use crate::helper::messages::LOGIN_FAILURE_MSG;
use crate::helper::endpoint::{
    http_status_code, 
    extract_employee_login,
    login_success_json_response
};
use crate::helper::app_utils::{
    build_login_redirect_cookie,
    remove_login_redirect_cookie,
    build_authorization_cookie,
    remove_authorization_cookie,
    remove_original_content_type_cookie
};
use crate::bh_libs::api_status::ApiStatus;
use crate::models::{EmployeeLogin, select_employee};

/// Renders the login page and return the complete content as a 
/// [`std::string::String`].
fn render_login_page(message: &str) -> String {
    // Create a new Tera instance and add a template from a string
    let tera = Tera::new("templates/auth/**/*").unwrap();

    let mut ctx = Context::new();
    // Passing data to be rendered to the template engine.
    if message.len() > 0 {
        ctx.insert("message", &message);
    }

    tera.render("login.html", &ctx).expect("Failed to render template")
}

/// Renders the home page and return the complete content as a 
/// [`std::string::String`].
fn render_home_page(_req: &HttpRequest) -> String {
    // Create a new Tera instance and add a template from a string
    let tera = Tera::new("templates/auth/**/*").unwrap();

    let ctx = Context::new();

    tera.render("home.html", &ctx).expect("Failed to render template")
}

/// ``First state`` is being just selecting a matching record base on exact email.
/// If one matched, then compare password.
/// 
/// If the original request content type is ``application/x-www-form-urlencoded``, 
/// then redirect to login page with a message whose value is param ``message``.
/// 
/// The message is set in a server only cookie: I haven't been able to find a better 
/// solution.
/// 
/// If the original request content type is ``application/json``, then returns a
/// [`crate::bh_libs::api_status::ApiStatus`].
/// 
fn first_stage_login_error_response(
    request: &HttpRequest,
    message: &str
) -> HttpResponse {
    if request.content_type() == ContentType::form_url_encoded().to_string() {
        HttpResponse::Ok()
            .status(StatusCode::SEE_OTHER)
            .append_header((header::LOCATION, "/ui/login"))
            // Note this per-request server-side only cookie.
            .cookie(build_login_redirect_cookie(&request, message))
            .finish()
    }
    else {
        HttpResponse::Ok()
            .status(StatusCode::UNAUTHORIZED)
            .content_type(ContentType::json())
            .body(serde_json::to_string(
                &ApiStatus::new(http_status_code(StatusCode::UNAUTHORIZED))
                    .set_message(message)).unwrap())
    }
}

/// A login process helper method. 
/// 
/// A database record has been successfully retrieved using exact email matched.
/// This methods de-hashes the database password and compares it against the 
/// submitted password, if matches, returns successful. If passwords don't match,
/// calls to [`first_stage_login_error_response`] to return appropriate error 
/// response.
/// 
/// Called by ``/api/login`` handler method [`login`].
/// 
/// # Arguments
/// 
/// * `request` - the original login request.
/// 
/// * submitted_login - the deserialised submitted login info.
/// 
/// * selected_login - email matched record read from database.
/// 
/// # Return
/// 
/// * Ok(()) - if passwords matched.
/// 
/// * [`first_stage_login_error_response`] - if passwords don't match.
/// 
fn match_password_response(
    request: &HttpRequest,
    submitted_login: &EmployeeLogin, 
    selected_login: &EmployeeLogin
) -> Result<(), HttpResponse> {
    let password_hash = String::from(&selected_login.password);
    let parsed_hash = PasswordHash::new(&password_hash).expect("Failed to parse hashed password.");
    if Argon2::default().verify_password(submitted_login.password.as_bytes(), &parsed_hash).is_ok() {
        return Ok(());
    };

    Err(first_stage_login_error_response(request, LOGIN_FAILURE_MSG))
}

/// Serves the “login page” response conditionally as HTML or JSON.
/// 
/// * Route: ``http://0.0.0.0:5000/ui/login``
/// * Method: ``GET``
/// 
/// This route can be called via: direct access from browser address bar, redirected 
/// to by the [`login`] handler, redirected to by the authentication / check logged 
/// in middleware [`super::auth_middleware`].
/// 
/// These different calls would result in different responses.
/// 
/// # Use Case Scenarios
/// 
/// ## ❶ Direct access from browser address bar 
/// 
/// Serve the login page HTML as is.
/// 
/// Note, the request content type is not available, it is blank.
/// 
/// ## ❷ Redirected to by the [`login`] handler 
/// 
/// This occurs when requests to login fail in some manner. 
/// 
/// The response HTTP status code always gets set to an appropriate error code.
/// 
/// The actual response depends the request content type:
/// 
/// * ``application/x-www-form-urlencoded``: serves the login page HTML with message
/// [`crate::helper::messages::LOGIN_FAILURE_MSG`].
/// 
/// * Blank: defaulted to ``application/x-www-form-urlencoded``. *Although it should
/// not be blank in this redirection scenario*.
/// 
/// * ``application/json``: serves JSON of [`crate::bh_libs::api_status::ApiStatus`],
/// the ``code`` field gets set to the value of the response HTTP status code above,
/// the ``message`` field gets set to [`crate::helper::messages::LOGIN_FAILURE_MSG`].
/// 
/// ## ❸ Redirected to by [`super::auth_middleware`] 
/// 
/// This occurs when original requests are to access protected resources but not 
/// logged in / authenticated.
/// 
/// If the original request, which gets redirected, has a content type, the middleware 
/// would retain and pass it to this handler. 
/// [See detail documentation](`crate::helper::app_utils::build_original_content_type_cookie#use-case-scenarios`).
/// 
/// The response HTTP status code always gets set to [`actix_web::http::StatusCode::UNAUTHORIZED`].
/// 
/// The actual response depends the request content type:
/// 
/// * ``application/x-www-form-urlencoded``: serves the login page HTML with message
/// [`crate::helper::messages::UNAUTHORISED_ACCESS_MSG`].
/// 
/// * Blank: defaulted to ``application/x-www-form-urlencoded``.
/// 
/// * ``application/json``: serves JSON of [`crate::bh_libs::api_status::ApiStatus`],
/// the ``code`` field gets set to the value of the response HTTP status code above,
/// the ``message`` field gets set to [`crate::helper::messages::UNAUTHORISED_ACCESS_MSG`].
/// 
/// ### Examples of when content type is blank
/// 
/// * When users run ``http://localhost:5000/ui/login`` directly on browser address bar. 
/// This is normal usage of this route.
/// 
/// * When users run ``http://localhost:5000/data/employees/%chi/%ak`` directly on 
/// browser address bar. 
/// 
/// A successful response is in JSON. But in this case, the response is the login page 
/// HTML with message [`crate::helper::messages::UNAUTHORISED_ACCESS_MSG`].
/// 
/// See ``tests/test_handlers.rs``' method ``get_employees_json2_no_access_token()``.
/// 
/// ### Explicitly setting content type
/// 
/// Instead of running ``http://localhost:5000/data/employees/%chi/%ak`` directly on 
/// browser address bar, call it via Ajax (for example), whereby there is an opportunity
/// to set content type. 
/// 
/// See ``tests/test_handlers.rs``' method 
/// ``get_employees_json2_with_content_type_no_access_token()``.
/// 
/// # Cookies Clean Up
/// 
/// The redirection mechanism relies on two per-request server-side cookies
/// [`crate::helper::constants::REDIRECT_MESSAGE`] and 
/// [`crate::helper::constants::ORIGINAL_CONTENT_TYPE`].
/// 
/// The later is created in [`super::auth_middleware`] by calling
/// [`crate::helper::app_utils::build_original_content_type_cookie`].
/// 
/// Looks for cookie [`crate::helper::constants::REDIRECT_MESSAGE`] value, and 
/// passes it to the template engine. Always removes this cookie.
/// 
/// If this handler is the final consumer of these two per-request cookies.
/// 
/// It should and must remove these two per-request cookies finished.
///
#[get("/login")]
pub async fn login_page(
    request: HttpRequest
) -> Either<impl Responder, HttpResponse> {
    let mut content_type: String = String::from(request.content_type());
    let mut status_code = StatusCode::OK;
    let mut message = String::from("");

    // Always checks for cookie REDIRECT_MESSAGE.
    if let Some(cookie) = request.cookie(REDIRECT_MESSAGE) {
        message = String::from(cookie.value());
        status_code = StatusCode::UNAUTHORIZED;

        if let Some(cookie) = request.cookie(ORIGINAL_CONTENT_TYPE) {
            if content_type.len() == 0 {
                content_type = String::from(cookie.value());
            }
        }
    }

    // When is content_type blank?
    //
    // 1. When users run http://localhost:5000/ui/login directly on browser address bar.
    //    This is normal usage of this route.
    // 
    // 2. When users run http://localhost:5000/data/employees/%chi/%ak directly on browser 
    //    address bar. This is a protected resource. The successful response is in JSON.
    //    
    //    But when not logged in, the authentication middleware redirected to this handler,
    //    the original request (from the address bar) does not have a content type, and
    //    so the redirected request cannot have one.
    //
    if content_type.len() == 0 || 
       content_type == ContentType::form_url_encoded().to_string() {
        Either::Right( 
            HttpResponse::Ok()
                .status(status_code)
                .content_type(ContentType::html())
                // Always removes cookie REDIRECT_MESSAGE.
                .cookie(remove_login_redirect_cookie(&request))
                // Always removes cookie ORIGINAL_CONTENT_TYPE.
                .cookie(remove_original_content_type_cookie(&request))
                .body(render_login_page(&message)) 
        )
    }
    else {
        Either::Left( 
            ApiStatus::new(http_status_code(status_code)).set_message(&message) 
        )
    }
}

/// Manages actual login requests. 
/// 
/// It accepts request body in both ``application/x-www-form-urlencoded`` and 
/// ``application/json`` content types.
/// 
/// *This current implementation does only email and password matching. And no session
/// time out. That is, once logged in, it stays valid forever till explicitly being 
/// logged out, or the browser is terminated.*
/// 
/// The log in process is simple: 
/// 
/// 1. Deserialising the submitted byte stream into [`crate::models::EmployeeLogin`].
/// If fails, log in fails. 
/// 
/// 2. The ``submitted email`` is used to identify an employee from the database. 
/// If no employee found, log in fails. 
/// 
/// 3. The ``database password`` is de-hashed and compared against the ``submitted password``, 
/// if does not match, log in fails. 
/// 
/// 4. If none fails, log in succeeds.
/// 
/// # Arguments
/// 
/// * `request` - Submitted request.
/// 
/// * `body` - [`actix_web::web::Bytes`], the actual submitted login data. 
/// It is either one of these two conent types: [`mime::APPLICATION_WWW_FORM_URLENCODED`] 
/// and [`mime::APPLICATION_JSON`].
/// 
/// # Response - Failure
/// 
/// * When failed deserialising, **always** returns a JSON serialised of 
/// [`crate::bh_libs::api_status::ApiStatus`].
/// 
/// * In cases where none employee matched or when passwords don't match:
/// 
///     - If the original request content type is ``application/x-www-form-urlencoded``,
///       then redirect to login page with an appropriate message and appropriate
///       HTTP status code.
/// 
///     - If the original request content type is ``application/json``, then returns a
///       JSON serialised of [`crate::bh_libs::api_status::ApiStatus`] with appropriate
///       HTTP status code set for ``code`` field.
/// 
/// # Response - Successful
/// 
/// * If the original request content type is ``application/x-www-form-urlencoded``,
///   then redirect to home page.
/// 
/// * If the original request content type is ``application/json``, then returns a
///   JSON in the form:
/// 
///   ``
///   {
///       "code": 200,
///       "message": null,
///       "session_id": null,
///       "data": {
///           "email": "behai_nguyen@hotmail.com",
///           "access_token": "xxxx...zzzz"
///       }
///   }
///   ``
/// 
/// The header part is [`crate::bh_libs::api_status::ApiStatus`], the ``data`` 
/// object is [`crate::models::LoginSuccess`].
/// 
/// # Response - Successful: Header and Cookie
/// 
/// The response always contains both a header (and a cookie) with same name
/// [`actix_web::http::header::AUTHORIZATION`], and same value: which
/// is the access token.
/// 
/// This access token is also the value of ``data.token`` field in case of
/// ``application/json`` responses above.
/// 
/// The clients need to remember this after a successfully logged in, on subsequent 
/// requests to access protected resources, they need to include this access token
/// in the requests' header [`actix_web::http::header::AUTHORIZATION`].
/// 
/// # Valid Usage
/// 
/// * Route: ``http://0.0.0.0:5000/api/login``
/// * Method: ``POST``
/// * Content type: ``application/x-www-form-urlencoded``; 
/// request body: ``email=chirstian.koblick.10004@gmail.com&password=password``.
/// 
/// * Content type: ``application/json``; 
/// request body: ``{"email": "chirstian.koblick.10004@gmail.com", "password": "password"}``.
/// 
#[post("/login")]
pub async fn login(
    request: HttpRequest,
    app_state: web::Data<super::AppState>,
    body: Bytes
) -> Either<impl Responder, HttpResponse> {
    // Attempts to extract -- deserialising -- request body into EmployeeLogin.
    let res = extract_employee_login(&body, request.content_type());
    if res.is_err() {
        return Either::Left(res.err().unwrap());
    }

    // Succeeded to deserialise request body.
    let submitted_login: EmployeeLogin = res.unwrap();
    let query_result = select_employee(&app_state.db, &submitted_login.email).await;

    if query_result.is_none() {
        return Either::Right(first_stage_login_error_response(&request, LOGIN_FAILURE_MSG));
    }

    let selected_login = query_result.unwrap();

    let res = match_password_response(&request, &submitted_login, &selected_login);
    if res.is_err() {
        return Either::Right(res.err().unwrap());
    }

    // TO_DO: Work in progress -- future implementations will formalise access token.
    let access_token = &selected_login.email;

    // https://docs.rs/actix-identity/latest/actix_identity/
    // Attach a verified user identity to the active session
    Identity::login(&request.extensions(), String::from(access_token)).unwrap();

    // The request content type is "application/x-www-form-urlencoded", returns the home page.
    if request.content_type() == ContentType::form_url_encoded().to_string() {
        Either::Right( HttpResponse::Ok()
            // Note this header.
            .append_header((header::AUTHORIZATION, String::from(access_token)))
            // Note this client-side cookie.
            .cookie(build_authorization_cookie(&request, access_token))
            .content_type(ContentType::html())
            .body(render_home_page(&request))
        )
    }
    else {
        // The request content type is "application/json", returns a JSON content of
        // LoginSuccessResponse.
        // 
        // Token field is the access token which the users need to include in the future 
        // requests to get authenticated and hence access to protected resources.		
        Either::Right( HttpResponse::Ok()
            // Note this header.
            .append_header((header::AUTHORIZATION, String::from(access_token)))
            // Note this client-side cookie.
            .cookie(build_authorization_cookie(&request, access_token))
            .content_type(ContentType::json())
            .body(login_success_json_response(&selected_login.email, &access_token))
        )
    }
}

/// Serves the home page for logged in sessions.
/// 
/// ``/ui/home`` gets redirected to by [`super::auth_middleware`], or by direct
/// access from address bar.
/// 
/// # Usage Example
/// 
/// * Route: ``http://0.0.0.0:5000/ui/home``
/// * Method: ``GET``
///
#[get("/home")]
pub async fn home_page(
    request: HttpRequest
) -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        // Always removes cookie ORIGINAL_CONTENT_TYPE.
        // If this handler was called as a result of auth_middleware.rs redirection,
        // then this cookie would have been set, but it is unused in this case. Gets
        // rid of it as a matter of cleanliness.
        .cookie(remove_original_content_type_cookie(&request))
        .body(render_home_page(&request))
}

/// Log out a logged in session by clearing user's [`actix_identity::Identity`] and 
/// also removes [`actix_web::http::header::AUTHORIZATION`] cookie.
/// 
/// TO_DO redirects to '/ui/login'. TO_DO: possibly use content type to returns 
/// either HTML or JSON.
/// 
/// # Arguments
/// 
/// # Usage Example
/// 
/// * Route: ``http://0.0.0.0:5000/api/logout``
/// * Method: ``GET``
///
#[post("/logout")]
async fn logout(
    request: HttpRequest,
    user: Identity
) -> impl Responder {
    user.logout();

    HttpResponse::Ok()
        // Note the cookie.
        .cookie(remove_authorization_cookie(&request))
        .status(StatusCode::SEE_OTHER)
        .append_header((header::LOCATION, "/ui/login"))
        .finish()
}
