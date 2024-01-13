/* Date Created: 07/01/2024. */

//! Modules which are application specifics.
//! 
//! # Note
//! 
//! Specific tests within this module can be run with:
//! 
//! * ``cargo test helper::tests``

pub mod endpoint;
pub mod messages;

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::web::Bytes;
    use crate::models::EmployeeLogin;
    use crate::bh_libs::api_status::ApiStatus;
    use messages::CONTENT_TYPE_NOT_RECOGNISED_MSG;
    use endpoint::{err_code_500, extract_employee_login};

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

        assert!(status.is_err(), "Failure");
        // This test is also valid.
        // assert_eq!(status.is_err_and(|status| status.code == err_code_500()), true);

        let api_stt: ApiStatus = status.err().unwrap();

        assert_eq!(api_stt.code, err_code_500());
        assert!(api_stt.text.unwrap().contains("missing field"), "Missing field error");
    }

    #[test]
    fn test_extract_employee_login_json_failure() {
        let body = Bytes::from_static(b"{\"email\": \"chirstian.koblick.10004@gmail.com\", \"passwordX\": \"password\"}");
        let content_type = mime::APPLICATION_JSON.to_string();

        let status = extract_employee_login(&body, &content_type);

        assert!(status.is_err(), "Failure");
        // This test is also valid.
        // assert_eq!(status.is_err_and(|status| status.code == err_code_500()), true);

        let api_stt: ApiStatus = status.err().unwrap();

        assert_eq!(api_stt.code, err_code_500());
        assert!(api_stt.text.unwrap().contains("missing field"), "Missing field error");
    }

    #[test]
    fn test_extract_employee_login_content_type_failure() {
        let body = Bytes::from_static(b"email=chirstian.koblick.10004@gmail.com&password=password");
        let content_type = mime::TEXT_PLAIN.to_string();

        let status = extract_employee_login(&body, &content_type);

        assert!(status.is_err(), "Failure");
        // This test is also valid.
        // assert_eq!(status.is_err_and(|status| status.code == err_code_500()), true);

        let api_stt: ApiStatus = status.err().unwrap();

        assert_eq!(api_stt.code, err_code_500());
        assert_eq!(api_stt.text.unwrap(), CONTENT_TYPE_NOT_RECOGNISED_MSG);
    }
}