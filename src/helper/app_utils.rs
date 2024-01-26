/* Date Created: 19/12/2023. */

//! Application helper / utility functions.

use actix_web::{
    http::{header, StatusCode, header::ContentType}, HttpRequest, cookie::{Cookie, SameSite}, 
    HttpResponse, Responder, body::BoxBody, //HttpMessage
};

use crate::helper::constants::{
    REDIRECT_MESSAGE,
    ORIGINAL_CONTENT_TYPE
};
use crate::bh_libs::api_status::ApiStatus;
// use crate::helper::html_renderer::render_home_page;
// use crate::models::LoginSuccessResponse;

/// Creates and returns a cookie.
/// 
/// # Arguments
/// 
/// * `request` - the original HTTP request. 
/// 
/// * `name` - the name of the cookie.
/// 
/// * `value` - the value of the cookie.
/// 
/// * `server_only` - if this cookie a server-side only. I.e., not accessible
/// by JavaScript.
/// 
/// * `removal` - if this cookie is to be removed. When ``true``, just pass
/// a blank string for ``value``.
/// 
/// # Return
/// 
/// * [`actix_web::cookie::Cookie`].
/// 
pub fn build_cookie<'a>(
    request: &'a HttpRequest,
    name: &'a str,
    value: &'a str,
    server_only: bool,
    removal: bool
) -> Cookie<'a> {
    // Header "host" should always be in the request headers.
    let host = request.headers().get("host").unwrap().to_str().unwrap().to_owned();
    // Remove the port if any.
    let parts = host.split(":");

    let mut cookie = Cookie::build(name, value)
        .domain(String::from(parts.collect::<Vec<&str>>()[0]))
        .path("/")
        .secure(false)
        .http_only(server_only)
        .same_site(SameSite::Strict)
        .finish();

    if removal {
        cookie.make_removal();
    }

    cookie
}

/// Creates and returns a server-side cookie whose name is [`crate::helper::constants::REDIRECT_MESSAGE`].
/// 
/// # Use Case Scenarios
/// 
/// 1. Login has failed to identify a database record using email; or passwords do not match. 
/// Redirect to login page. See [`crate::auth_handlers::login_page`].
/// 
/// 2. When users attempt to access protected resources while not logged in. Redirect to login page. 
/// See [`crate::auth_middleware`].
/// 
/// Calls this method to create this server-side cookie whose value is an appropriate redirect 
/// message. 
/// 
/// **The presence of this server-side cookie also means that [`crate::auth_handlers::login_page`]
/// is called via a redirected request, that is, it is called in response to an (authentication) 
/// error condition.**
/// 
/// Note, [`crate::auth_handlers::login_page`] is the final consumer of this server-side cookie 
/// [`crate::helper::constants::REDIRECT_MESSAGE`] and [`crate::helper::constants::ORIGINAL_CONTENT_TYPE`].
/// When done cosuming, it should and must remove them.
/// 
/// # Arguments
/// 
/// * `request` - the original HTTP request. 
/// 
/// * `value` - the text message.
/// 
/// # Return
/// 
/// * [`actix_web::cookie::Cookie`].
/// 
pub fn build_login_redirect_cookie<'a> (
    request: &'a HttpRequest,
    value: &'a str
) -> Cookie<'a> {
    build_cookie(request, REDIRECT_MESSAGE, value, true, false)
}

/// Creates and returns a server-side cookie to be removed, and whose name is 
/// [`crate::helper::constants::REDIRECT_MESSAGE`].
/// 
/// # Arguments
/// 
/// * `request` - the original HTTP request. 
/// 
/// # Return
/// 
/// * [`actix_web::cookie::Cookie`].
/// 
pub fn remove_login_redirect_cookie(request: &HttpRequest) -> Cookie {
    build_cookie(request, REDIRECT_MESSAGE, "", true, true)
}

/// Creates and returns a cookie whose name is [`actix_web::http::header::AUTHORIZATION`].
/// 
/// This cookie is client-side accessible, i.e., by JavaScript.
/// 
/// # Arguments
/// 
/// * `request` - the original HTTP request. 
/// 
/// * `access_token` - the access token value.
/// 
/// # Return
/// 
/// * [`actix_web::cookie::Cookie`].
/// 
pub fn build_authorization_cookie<'a>(
    request: &'a HttpRequest,
    access_token: &'a str
) -> Cookie<'a> {
    build_cookie(request, header::AUTHORIZATION.as_str(), access_token, false, false)
}

/// Creates and returns a cookie to be removed, and whose name is 
/// [`actix_web::http::header::AUTHORIZATION`].
/// 
/// # Arguments
/// 
/// * `request` - the original HTTP request. 
/// 
/// # Return
/// 
/// * [`actix_web::cookie::Cookie`].
/// 
pub fn remove_authorization_cookie(request: &HttpRequest) -> Cookie {
    build_cookie(request, header::AUTHORIZATION.as_str(), "", false, true)
}

/// Creates and returns a server-side cookie whose name is [`crate::helper::constants::ORIGINAL_CONTENT_TYPE`].
/// 
/// # Use Case Scenarios
/// 
/// ## ❶ In an unauthorised state, i.e., not logged in, users make a request to the following 
/// route, for example:
/// 
/// * URL: ``http://localhost:5000/data/employees``
/// * Method: ``POST``
/// * Content type: ``application/json``
/// * Body: ``{"last_name": "%chi", "first_name": "%ak"}``
/// * Response: JSON.
/// 
/// The authentication / check logged in middleware would redirect this request to login page.
/// 
/// Login page handler needs the content type to determine whether to returns the HTML login 
/// page, or a JSON of [`crate::bh_libs::api_status::ApiStatus`].
/// 
/// The redirected request is independent of the original request, **and does not seem to have
/// the content type of the original request**. So:
/// 
/// 1. So when redirects to any route, the middleware always sets the original request content 
/// type as value of this server-side cookie, by calling this function.
/// 
/// 2. In addition, if redirect to  ``/ui/login``, i.e. [`crate::auth_handlers::login_page`],
/// then the server-side cookie [`crate::helper::constants::REDIRECT_MESSAGE`], see 
/// [`build_login_redirect_cookie`] and [`remove_login_redirect_cookie`], is also set (with 
/// [`crate::helper::messages::UNAUTHORISED_ACCESS_MSG`]).
/// 
/// 3. Finally, when the redirected request arrives at the handler method [`crate::auth_handlers::login_page`], 
/// this method can extract data from both **per-request** server-side cookies 
/// [REDIRECT_MESSAGE](`crate::helper::constants::REDIRECT_MESSAGE`) and
/// [ORIGINAL_CONTENT_TYPE](`crate::helper::constants::ORIGINAL_CONTENT_TYPE`).
/// 
/// [login_page](`crate::auth_handlers::login_page`) is the only consumer of both of these 
/// per-request server-side cookies, when finished, it must remove both.
///
/// The response in this case would be JSON of [`crate::bh_libs::api_status::ApiStatus`] with ``code`` 
/// field value set to [`actix_web::http::StatusCode::UNAUTHORIZED`] and ``message`` field value set 
/// to [`crate::helper::messages::UNAUTHORISED_ACCESS_MSG`].
/// 
/// The response HTTP status code is also set to [`actix_web::http::StatusCode::UNAUTHORIZED`].
/// 
/// When done cosuming, it should and must remove both of these per-request cookies.
/// 
/// ## ❷ In an unauthorised state, i.e., not logged in, users make a request to the following 
/// route, for example, by running it directly on browser address bar:
/// 
/// * URL: ``http://localhost:5000/data/employees/%chi/%ak``
/// * Method: ``GET``
/// * Response: JSON.
/// 
/// All steps discussed in ❶ take place. But the content type is not available originally, 
/// it is defaulted to ``application/x-www-form-urlencoded`` by [`crate::auth_handlers::login_page`].
/// 
/// The response by [`crate::auth_handlers::login_page`] is then the actual HTML page with
/// message [`crate::helper::messages::UNAUTHORISED_ACCESS_MSG`].
/// 
/// Similar to the ❶ above, the response HTTP status code is also set to 
/// [`actix_web::http::StatusCode::UNAUTHORIZED`].
/// 
/// # Arguments
/// 
/// * `request` - the original HTTP request. 
/// 
/// * `value` - the content type string.
/// 
/// # Return
/// 
/// * [`actix_web::cookie::Cookie`].
/// 
pub fn build_original_content_type_cookie<'a>(
    request: &'a HttpRequest,
    content_type: &'a str
) -> Cookie<'a> {
    build_cookie(request, ORIGINAL_CONTENT_TYPE, content_type, true, false)
}

/// Creates and returns a server-side cookie to be removed, and whose name is 
/// [`crate::helper::constants::ORIGINAL_CONTENT_TYPE`].
/// 
/// # Arguments
/// 
/// * `request` - the original HTTP request. 
/// 
/// # Return
/// 
/// * [`actix_web::cookie::Cookie`].
/// 
pub fn remove_original_content_type_cookie(
    request: &HttpRequest,
) -> Cookie {
    build_cookie(request, ORIGINAL_CONTENT_TYPE, "", true, true)
}

/// See [Response with custom type](https://actix.rs/docs/handlers#response-with-custom-type).
/// 
/// # Note
/// 
/// * **TO_DO**: watch out for ``.cookie(remove_login_redirect_cookie(request))`` and
/// ``.cookie(remove_original_content_type_cookie(request))``. This trait is first used
/// in [`crate::auth_handlers::login_page`].
/// 
/// *Removing these cookies might cause problem when using this trait in some other 
/// functions.*
/// 
impl Responder for ApiStatus {
    type Body = BoxBody;

    fn respond_to(self, request: &HttpRequest) -> HttpResponse<Self::Body> {
        // Create response and set content type
        HttpResponse::Ok()
            .status( StatusCode::from_u16(self.get_code()).unwrap() )
            .content_type(ContentType::json())
            // Always removes cookie REDIRECT_MESSAGE.
            .cookie(remove_login_redirect_cookie(request))
            // Always removes cookie ORIGINAL_CONTENT_TYPE.
            .cookie(remove_original_content_type_cookie(request))            
            .body(serde_json::to_string(&self).unwrap())
    }
}

/*
impl Responder for LoginSuccessResponse {
    type Body = BoxBody;

    fn respond_to(self, request: &HttpRequest) -> HttpResponse<Self::Body> {
        // The request content type is "application/x-www-form-urlencoded", returns the home page.
        if request.content_type() == ContentType::form_url_encoded().to_string() {
            HttpResponse::Ok()
                // Note this header.
                .append_header((header::AUTHORIZATION, String::from(&self.data.access_token)))
                // Note this client-side cookie.
                .cookie(build_authentication_cookie(&request, &self.data.access_token))
                .content_type(ContentType::html())
                .body(render_home_page(&request))
        }
        else {
            // The request content type is "application/json", returns a JSON content of
            // LoginSuccessResponse.
            // 
            // Token field is the access token which the users need to include in the future 
            // requests to get authenticated and hence access to protected resources.		
            HttpResponse::Ok()
                // Note this header.
                .append_header((header::AUTHORIZATION, String::from(&self.data.access_token)))
                // Note this client-side cookie.
                .cookie(build_authentication_cookie(&request, &self.data.access_token))
                .content_type(ContentType::json())
                .body(serde_json::to_string(&self).unwrap())
        }
    }
}
*/
