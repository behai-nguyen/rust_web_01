/* Date Created: 07/01/2024. */

//! Application request handler helper methods. **Work in progress**.

use actix_web::http::StatusCode;

use serde_json;

use crate::models::{LoginSuccess, LoginSuccessResponse};
use crate::bh_libs::api_status::ApiStatus;

/// Returns an [`actix_web::http::StatusCode`] as an [`u16`].
/// 
pub fn http_status_code(status_code: StatusCode) -> u16 {
    status_code.as_u16()
}

/// Creates and returns a serialised version of [`ApiStatus`].
/// 
/// # Return
/// 
/// * JSON string of [`ApiStatus`].
/// 
pub fn serialise_api_status(
    status_code: StatusCode,
    message: &str,
    session_id: Option<String>
) -> String {
    let mut api_status = ApiStatus::new(http_status_code(status_code))
        .set_message(message);

    if let Some(sess_id) = session_id {
        api_status = api_status.set_session_id(&sess_id);
    }

    serde_json::to_string(&api_status).unwrap()
}

/// Constructs a successful login JSON response.
///
/// Combine [`crate::bh_libs::api_status::ApiStatus`] and 
/// [`crate::models::LoginSuccess`] to make a successful login JSON response in the
/// form:
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
/// # Arguments
/// 
/// * `email` - the email of the successful logged in session. It's the value of
/// ``data.email``.
/// 
/// * `access_token` - the server generated access token. Clients need to include 
/// this value in the request header [`actix_web::http::header::AUTHORIZATION`]
/// on subsequent requests to access protected resources. It's the value of 
/// ``data.access_token``.
/// 
/// # Return
/// 
/// * JSON object as listed above.
/// 
pub fn login_success_json_response(
    email: &str, 
    access_token: &str) -> String {

    let r = LoginSuccessResponse {
        api_status: ApiStatus::new(http_status_code(StatusCode::OK)),
        data: LoginSuccess { email: String::from(email), access_token: String::from(access_token) }
    };

    serde_json::to_string(&r).unwrap()
}