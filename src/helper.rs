/* Date Created: 07/01/2024. */

//! Modules which are application specifics.
//! 
//! # Note
//! 
//! Specific tests within this module can be run with:
//! 
//! * ``cargo test helper::tests``

pub mod constants;
pub mod app_utils;
pub mod endpoint;
pub mod messages;

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::web::Bytes;
    use actix_web::http::StatusCode;
    use crate::models::EmployeeLogin;
    use crate::bh_libs::api_status::ApiStatus;
    use messages::{
        CONTENT_TYPE_NOT_RECOGNISED_MSG,
        REQUEST_BODY_EMPTY_MSG
    };
    use endpoint::{http_status_code, extract_employee_login};

    #[test]
    fn test_extract_employee_login_url_encoded_success() {
        let body = Bytes::from_static(b"email=chirstian.koblick.10004@gmail.com&password=password");
        let content_type = mime::APPLICATION_WWW_FORM_URLENCODED.to_string();

        let status = extract_employee_login(&body, &content_type);

        assert!(status.is_ok(), "Success");

        // Should always succeed at this point.
        let employee_login: EmployeeLogin = status.unwrap();
        assert_eq!(employee_login.email, "chirstian.koblick.10004@gmail.com");
        assert_eq!(employee_login.password, "password");
    }

    #[test]
    fn test_extract_employee_login_json_success() {
        let body = Bytes::from_static(b"{\"email\": \"chirstian.koblick.10004@gmail.com\", \"password\": \"password\"}");
        let content_type = mime::APPLICATION_JSON.to_string();

        let status = extract_employee_login(&body, &content_type);

        assert!(status.is_ok(), "Success");

        // Should always succeed at this point.
        let employee_login: EmployeeLogin = status.unwrap();
        assert_eq!(employee_login.email, "chirstian.koblick.10004@gmail.com");
        assert_eq!(employee_login.password, "password");
    }

    #[test]
    fn test_extract_employee_login_url_encoded_failure() {
        let body = Bytes::from_static(b"");
        let content_type = mime::APPLICATION_WWW_FORM_URLENCODED.to_string();

        let status = extract_employee_login(&body, &content_type);

        let bad_request: u16 = http_status_code(StatusCode::BAD_REQUEST);

        assert!(status.is_err(), "Failure");
        // This test is also valid.
        assert_eq!(status.as_ref().is_err_and(|s| s.get_code() == bad_request), true);

        let api_stt: ApiStatus = status.err().unwrap();

        assert_eq!(api_stt.get_code(), bad_request);
        assert_eq!(api_stt.get_message().unwrap(), REQUEST_BODY_EMPTY_MSG);
    }

    #[test]
    fn test_extract_employee_login_json_failure() {
        let body = Bytes::from_static(b"{\"email\": \"chirstian.koblick.10004@gmail.com\", \"passwordX\": \"password\"}");
        let content_type = mime::APPLICATION_JSON.to_string();

        let status = extract_employee_login(&body, &content_type);

        let bad_request: u16 = http_status_code(StatusCode::BAD_REQUEST);

        assert!(status.is_err(), "Failure");
        // This test is also valid.
        assert_eq!(status.as_ref().is_err_and(|s| s.get_code() == bad_request), true);

        let api_stt: ApiStatus = status.err().unwrap();

        assert_eq!(api_stt.get_code(), bad_request);
        assert!(api_stt.get_message().unwrap().contains("missing field"), "Missing field error");
    }

    #[test]
    fn test_extract_employee_login_content_type_failure() {
        let body = Bytes::from_static(b"email=chirstian.koblick.10004@gmail.com&password=password");
        let content_type = mime::TEXT_PLAIN.to_string();

        let status = extract_employee_login(&body, &content_type);

        let bad_request: u16 = http_status_code(StatusCode::BAD_REQUEST);

        assert!(status.is_err(), "Failure");
        // This test is also valid.
        assert_eq!(status.as_ref().is_err_and(|s| s.get_code() == bad_request), true);

        let api_stt: ApiStatus = status.err().unwrap();

        assert_eq!(api_stt.get_code(), bad_request);
        assert_eq!(api_stt.get_message().unwrap(), CONTENT_TYPE_NOT_RECOGNISED_MSG);
    }
}