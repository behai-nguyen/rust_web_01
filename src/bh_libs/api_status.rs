/* Date Created: 07/01/2024. */

//! Generic API status. Encapsulate result status of server operations in 
//! response to client requests. 
//! 
//! It is modelled after [Python bh-apistatus package](https://bh-apistatus.readthedocs.io/en/latest/).
//! 

// Command to run doc-tests: 
// 
//     * cargo test --doc bh_libs::api_status
//
use serde::{Deserialize, Serialize};
use std::fmt;

// Struct actix_web::http::StatusCode / https://docs.rs/actix-web/latest/actix_web/http/struct.StatusCode.html

/// Encapsulate result status of server operations in response to client requests.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApiStatus {
    /// One of those HTTP codes defined in [`actix_web::http::StatusCode`].
    code: u16,
    /// An optional message.
    message: Option<String>,
    /// An optional web session identifier.
    session_id: Option<String>,
}

/// For debugging etc., display [`ApiStatus`] as JSON.
impl fmt::Display for ApiStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

impl ApiStatus {
    /// Returns an API status with the given code.
    /// 
    /// # Arguments
    /// 
    /// * `code` - an appropriate code defined in [`actix_web::http::StatusCode`].
    /// 
    /// # Examples
    /// 
    /// ```
    /// use actix_web::http::StatusCode;
    /// use learn_actix_web::bh_libs::api_status::ApiStatus;
    /// 
    /// let status = ApiStatus::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16());
    /// ```
    pub fn new(code: u16) -> Self {
        Self { code, message: None, session_id: None }
    }

    /// Returns the value of the ``code`` field.
    pub fn get_code(&self) -> u16 {
        self.code
    }

    /// Sets the `text` field on `self`, and returns itself.
    /// 
    /// # Arguments
    /// 
    /// * `text` - text string to set for `text` field.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use actix_web::http::StatusCode;
    /// use learn_actix_web::bh_libs::api_status::ApiStatus;
    /// 
    /// let status = ApiStatus::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16())
    ///     .set_message("I am testing...");
    /// ```
    pub fn set_message(mut self, text: &str) -> Self {
        self.message = Some(String::from(text));
        self
    }

    /// Returns the value of the ``message`` field.
    pub fn get_message(&self) -> Option<String> {
        self.message.clone()
    }

    /// Sets the `text` field on `self`, and returns itself.
    /// 
    /// # Arguments
    /// 
    /// * `text` - text string to set for `text` field.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use actix_web::http::StatusCode;
    /// use learn_actix_web::bh_libs::api_status::ApiStatus;
    /// 
    /// let status = ApiStatus::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16())
    ///     .set_session_id("abcd-efgh-ijkl-mnop");
    /// ```
    pub fn set_session_id(mut self, session_id: &str) -> Self {
        self.session_id = Some(String::from(session_id));
        self
    }

    /// Returns the value of the ``session_id`` field.
    pub fn get_session_id(&self) -> Option<String> {
        self.session_id.clone()
    }    
}