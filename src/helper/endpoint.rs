/* Date Created: 07/01/2024. */

//! Application request handler helper methods. **Work in progress**.

use actix_web::web::Bytes;
use actix_web::http::StatusCode;

use serde_html_form::from_bytes;

use crate::models::EmployeeLogin;
use crate::bh_libs::api_status::ApiStatus;
use crate::helper::messages::CONTENT_TYPE_NOT_RECOGNISED_MSG;

/// HTTP Internal Server Error code, 500.
pub fn err_code_500() -> u16 {
    StatusCode::INTERNAL_SERVER_ERROR.as_u16()
}

/// Attempts to deserialise user submitted employee login information into struct
/// [`EmployeeLogin`]. The submitted data can be in [`mime::APPLICATION_WWW_FORM_URLENCODED`]
/// or [`mime::APPLICATION_JSON`]. Any other content types are invalid.
/// 
/// *Note, the objective is to turn this method into generic later on. I've attempted, and failed.*
/// 
/// # Arguments
/// 
/// * `body` - [`actix_web::web::Bytes`], the actual submitted data.
/// 
/// * `content_type` - a string slice [`str`]. Valid values are 
/// [`mime::APPLICATION_WWW_FORM_URLENCODED`] and [`mime::APPLICATION_JSON`].
/// 
/// # Return
/// 
/// - [`std::result::Result`]&lt;[`EmployeeLogin`], [`ApiStatus`]&gt; - that is,
/// returns [`EmployeeLogin`] if deserialisation (extraction) is successful, [`ApiStatus`] otherwise.
/// 
/// # Usage Example
/// 
/// * See [`crate::auth_handlers::login`]'s implementation, i.e., actual code.
/// 
pub fn extract_employee_login(
    body: &Bytes, 
    content_type: &str
) -> Result<EmployeeLogin, ApiStatus> {

    // Content type and associated extraction function.
    struct Extractor {
        content_type: String,
        handler: fn(body: &Bytes) -> Result<EmployeeLogin, ApiStatus>,
    }

    // A list of Extractor.
    let mut extractors: Vec<Extractor> = vec![];

    // "application/x-www-form-urlencoded" content type and extraction function. 
    // That is, a function which deserialises a byte stream presentation of 
    // "application/x-www-form-urlencoded" to EmployeeLogin.
    extractors.push(Extractor { 
        content_type: mime::APPLICATION_WWW_FORM_URLENCODED.to_string(), 
        handler: |body: &Bytes| -> Result<EmployeeLogin, ApiStatus> {
            match from_bytes::<EmployeeLogin>(&body.to_owned().to_vec()) {
                Ok(e) => Ok(e),
                Err(e) => Err(ApiStatus::new(err_code_500()).set_text(&e.to_string()))
            }
        }
    });

    // "application/json" content type and extraction function. That is, a function which 
    // deserialises a byte stream presentation of "application/json" to EmployeeLogin.
    extractors.push(Extractor {
        content_type: mime::APPLICATION_JSON.to_string(),
        handler: |body: &Bytes| -> Result<EmployeeLogin, ApiStatus> {
            // From https://stackoverflow.com/a/67340858
            match serde_json::from_slice(&body.to_owned()) {
                Ok(e) => Ok(e),
                Err(e) => Err(ApiStatus::new(err_code_500()).set_text(&e.to_string()))
            }
        }
    });

    // If param content_type is a recognised content type, then attempt to 
    // deserialise the byte stream.
    for extractor in extractors {
        if extractor.content_type == content_type {
            return (extractor.handler)(body);
        }
    }

    // Param content_type is not recognised, return ApiStatus with an 
    // appropriate message.
    Err(ApiStatus::new(err_code_500()).set_text(CONTENT_TYPE_NOT_RECOGNISED_MSG))
}
