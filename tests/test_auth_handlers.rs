/* Date Created: 05/01/2024. */

//! Integration test cases for endpoint handler methods defined in auth_handlers.rs.
//! 
//! Test the following routes:
//! 
//! * Route: ``http://localhost:5000/ui/home``
//! * Method: ``GET``
//! * Response: ``HTML``
//! 
//! * Route: ``http://localhost:5000/ui/login``
//! * Method: ``GET``
//! * Response: ``HTML``
//! 
//! * Route: ``http://localhost:5000/api/login``
//! * Method: ``POST``
//! * Content Type: ``application/json``
//! * Body: ``{"email": "chirstian.koblick.10004@gmail.com", "password": "password"}``
//! 
//! * Content Type: ``application/x-www-form-urlencoded`` (charset=UTF-8)
//! * Body: ``email=chirstian.koblick.10004@gmail.com&password=password``
//!
//! * Route: ``http://localhost:5000/api/logout``
//! * Method: ``POST``
//! * Response: ``HTML``
//! 
//! To run test for this module only: 
//! 
//!     * cargo test --test test_auth_handlers
//! 
//! To run a specific test method: 
//! 
//!     * cargo test get_home_page_html -- --exact
//!     * cargo test get_login_page_html -- --exact
//!     * cargo test post_login_html -- --exact
//!     * cargo test post_login_html_error_empty -- --exact
//!     * cargo test post_login_html_missing_field -- --exact
//!     * cargo test post_login_json -- --exact
//!     * cargo test post_login_json_error_empty -- --exact
//!     * cargo test post_login_json_missing_field -- --exact
//!     * cargo test post_login_html_failure_1 -- --exact
//!     * cargo test post_login_html_failure_2 -- --exact
//!     * cargo test post_login_html_failure_3 -- --exact
//!     * cargo test post_login_json_failure_1 -- --exact 
//!     * cargo test post_login_json_failure_2 -- --exact
//!     * cargo test post_login_json_failure_3 -- --exact 
//!     * cargo test post_logout_html -- --exact
//! 
use std::collections::HashMap;
use actix_web::http::{StatusCode, header};

mod common;
use common::{spawn_app, make_api_url, make_ui_url};

use learn_actix_web::helper::messages::LOGIN_FAILURE_MSG;

/// * Route: ``http://localhost:5000/ui/home``
/// * Method: ``GET``
/// * Response: ``HTML``
#[actix_web::test]
async fn get_home_page_html() {
    let test_app = &spawn_app().await;

    let client = common::reqwest_client();

    let response = client
        .get(make_ui_url(&test_app.app_url, "/home"))
        .header(header::AUTHORIZATION, &test_app.mock_access_token())
        .send()
        .await
        .expect("Failed to execute request.");

    common::assert_html_home_page(response).await;
}

/// * Route: ``http://localhost:5000/ui/login``
/// * Method: ``GET``
/// * Response: ``HTML``
#[actix_web::test]
async fn get_login_page_html() {
    let test_app = &spawn_app().await;

    let client = common::reqwest_client();

    let response = client
        .get(make_ui_url(&test_app.app_url, "/login"))
        .send()
        .await
        .expect("Failed to execute request.");

    common::assert_html_login_page(response).await;
}

/// * Route: ``http://localhost:5000/api/login``
/// * Method: ``POST``
/// * Content Type: ``application/x-www-form-urlencoded``
/// (``application/x-www-form-urlencoded; charset=UTF-8``).
/// * Body: ``email=chirstian.koblick.10004@gmail.com&password=password``
/// 
#[actix_web::test]
async fn post_login_html() {
    let test_app = &spawn_app().await;

    let client = common::reqwest_client();

    let mut params = HashMap::new();
    params.insert("email", "chirstian.koblick.10004@gmail.com");
    params.insert("password", "password");

    let response = client
        .post(make_api_url(&test_app.app_url, "/login"))
        .form(&params)
        .send()
        .await
        .expect("Failed to execute request.");

    let access_token = "chirstian.koblick.10004@gmail.com";

    common::assert_access_token_in_header(&response, access_token);
    common::assert_access_token_in_cookie(&response, access_token);
    common::assert_html_home_page(response).await;
}

/// * Route: ``http://localhost:5000/api/login``
/// * Method: ``POST``
/// * Content Type: not set.
/// * Body: not set.
/// 
#[actix_web::test]
async fn post_login_html_error_empty() {
    let test_app = &spawn_app().await;

    let client = common::reqwest_client();

    let response = client
        .post(make_api_url(&test_app.app_url, "/login"))
        .send()
        .await
        .expect("Failed to execute request.");

    common::assert_access_token_not_in_header(&response);
    common::assert_access_token_not_in_cookie(&response);
    // NOTE: "Content type error" message!!
    common::assert_json_failure(response, StatusCode::BAD_REQUEST, "Content type error", false).await;
}

/// * Route: ``http://localhost:5000/api/login``
/// * Method: ``POST``
/// * Content Type: ``application/x-www-form-urlencoded``
/// (``application/x-www-form-urlencoded; charset=UTF-8``).
/// * Body: ``emaXXXil=chirstian.koblick.10004@gmail.com&password=password``
/// 
#[actix_web::test]
async fn post_login_html_missing_field() {
    let test_app = &spawn_app().await;

    let client = common::reqwest_client();

    let mut params = HashMap::new();
    params.insert("emaXXXil", "chirstian.koblick.10004@gmail.com");
    params.insert("password", "password");

    let response = client
        .post(make_api_url(&test_app.app_url, "/login"))
        .form(&params)
        .send()
        .await
        .expect("Failed to execute request.");

    common::assert_access_token_not_in_header(&response);
    common::assert_access_token_not_in_cookie(&response);
    // NOTE: "Content type error" message.
    common::assert_json_failure(response, StatusCode::BAD_REQUEST, "Content type error", false).await;
}

/// * Route: ``http://localhost:5000/api/login``
/// * Method: ``POST``
/// * Content Type: ``application/json``
/// * Body: ``{"email": "chirstian.koblick.10004@gmail.com", "password": "password"}``
/// 
#[actix_web::test]
async fn post_login_json() {
    let test_app = &spawn_app().await;

    let client = common::reqwest_client();

    let mut json_data = HashMap::new();
    json_data.insert("email", "saniya.kalloufi.10008@gmail.com");
    json_data.insert("password", "password");

    let response = client
        .post(make_api_url(&test_app.app_url, "/login"))
        .json(&json_data)
        .send()
        .await
        .expect("Failed to execute request.");


    let access_token = "saniya.kalloufi.10008@gmail.com";

    common::assert_access_token_in_header(&response, access_token);
    common::assert_access_token_in_cookie(&response, access_token);    

    let email = "saniya.kalloufi.10008@gmail.com";

    common::assert_json_successful_login(response, email, access_token).await;
}

/// * Route: ``http://localhost:5000/api/login``
/// * Method: ``POST``
/// * Content Type: Not set.
/// * Body: Not set.
/// 
#[actix_web::test]
async fn post_login_json_error_empty() {
    let test_app = &spawn_app().await;

    let client = common::reqwest_client();

    let response = client
        .post(make_api_url(&test_app.app_url, "/login"))
        .send()
        .await
        .expect("Failed to execute request.");


    common::assert_access_token_not_in_header(&response);
    common::assert_access_token_not_in_cookie(&response);
    //NOTE: "Content type error" message.
    common::assert_json_failure(response, StatusCode::BAD_REQUEST, "Content type error", false).await;
}

/// * Route: ``http://localhost:5000/api/login``
/// * Method: ``POST``
/// * Content Type: ``application/json``
/// * Body: ``{"emXXXail": "saniya.kalloufi.10008@gmail.com", "password": "password"}``
/// 
#[actix_web::test]
async fn post_login_json_missing_field() {
    let test_app = &spawn_app().await;

    let client = common::reqwest_client();

    let mut json_data = HashMap::new();
    json_data.insert("emXXXail", "saniya.kalloufi.10008@gmail.com");
    json_data.insert("password", "password");

    let response = client
        .post(make_api_url(&test_app.app_url, "/login"))
        .json(&json_data)
        .send()
        .await
        .expect("Failed to execute request.");


    common::assert_access_token_not_in_header(&response);
    common::assert_access_token_not_in_cookie(&response);
    common::assert_json_failure(response, StatusCode::BAD_REQUEST, "missing field `email`", true).await;
}

/// No match on email. Returns login HTML page with message 
/// [`learn_actix_web::helper::messages::LOGIN_FAILURE_MSG`]
/// 
/// * Route: ``http://localhost:5000/api/login``
/// * Method: ``POST``
/// * Content Type: ``application/x-www-form-urlencoded``
/// (``application/x-www-form-urlencoded; charset=UTF-8``).
/// * Body: ``email=suzette.petXXtey.10024@gmail.com&password=password``
/// 
#[actix_web::test]
async fn post_login_html_failure_1() {
    let test_app = &spawn_app().await;

    let client = common::reqwest_client();

    let mut params = HashMap::new();
    params.insert("email", "suzette.petXXtey.10024@gmail.com");
    params.insert("password", "password");

    let response = client
        .post(make_api_url(&test_app.app_url, "/login"))
        .form(&params)
        .send()
        .await
        .expect("Failed to execute request.");

    common::assert_access_token_not_in_header(&response);
    common::assert_access_token_not_in_cookie(&response);

    // No employee matches the login email.
    common::assert_redirected_html_login_page(response, 
        StatusCode::UNAUTHORIZED, LOGIN_FAILURE_MSG).await;
}


/// Password don't match. Returns login HTML page with message 
/// [`learn_actix_web::helper::messages::LOGIN_FAILURE_MSG`]
/// 
/// * Route: ``http://localhost:5000/api/login``
/// * Method: ``POST``
/// * Content Type: ``application/x-www-form-urlencoded``
/// (``application/x-www-form-urlencoded; charset=UTF-8``).
/// * Body: ``email=suzette.pettey.10024@gmail.com&password=passworKKd``
/// 
#[actix_web::test]
async fn post_login_html_failure_2() {
    let test_app = &spawn_app().await;

    let client = common::reqwest_client();

    let mut params = HashMap::new();
    params.insert("email", "suzette.pettey.10024@gmail.com");
    params.insert("password", "passworKKd");

    let response = client
        .post(make_api_url(&test_app.app_url, "/login"))
        .form(&params)
        .send()
        .await
        .expect("Failed to execute request.");

    common::assert_access_token_not_in_header(&response);
    common::assert_access_token_not_in_cookie(&response);

    // Passwords don't match.
    common::assert_redirected_html_login_page(response,
        StatusCode::UNAUTHORIZED, LOGIN_FAILURE_MSG).await;    
}

/// No email field. Returns login JSON [`learn_actix_web::bh_utils::api_status::ApiStatus`] 
/// with code 500 and message "missing field `email`".
/// 
/// * Route: ``http://localhost:5000/api/login``
/// * Method: ``POST``
/// * Content Type: ``application/x-www-form-urlencoded``
/// (``application/x-www-form-urlencoded; charset=UTF-8``).
/// * Body: ``emXXail=suzette.pettey.10024@gmail.com&password=password``
/// 
#[actix_web::test]
async fn post_login_html_failure_3() {
    let test_app = &spawn_app().await;

    let client = common::reqwest_client();

    let mut params = HashMap::new();
    params.insert("emXXail", "suzette.pettey.10024@gmail.com");
    params.insert("password", "password");

    let response = client
        .post(make_api_url(&test_app.app_url, "/login"))
        .form(&params)
        .send()
        .await
        .expect("Failed to execute request.");

    common::assert_access_token_not_in_header(&response);
    common::assert_access_token_not_in_cookie(&response);
    common::assert_json_failure(response, StatusCode::BAD_REQUEST, 
        "Content type error", false).await;
}

/// No match on email. Returns JSON [`learn_actix_web::bh_utils::api_status::ApiStatus`] 
/// with code 500 and message [`learn_actix_web::helper::messages::LOGIN_FAILURE_MSG`].
/// 
/// * Route: ``http://localhost:5000/api/login``
/// * Method: ``POST``
/// * Content Type: ``application/json``
/// * Body: ``email=suzette.petXXtey.10024@gmail.com&password=password``
/// 
#[actix_web::test]
async fn post_login_json_failure_1() {
    let test_app = &spawn_app().await;

    let client = common::reqwest_client();

    let mut json_data = HashMap::new();
    json_data.insert("email", "suzette.petXXtey.10024@gmail.com");
    json_data.insert("password", "password");

    let response = client
        .post(make_api_url(&test_app.app_url, "/login"))
        .json(&json_data)
        .send()
        .await
        .expect("Failed to execute request.");

    common::assert_access_token_not_in_header(&response);
    common::assert_access_token_not_in_cookie(&response);    
    common::assert_json_failure(response, StatusCode::UNAUTHORIZED, 
        LOGIN_FAILURE_MSG, false).await; 
}

/// No match on password. Returns JSON [`learn_actix_web::bh_utils::api_status::ApiStatus`] 
/// with code 500 and message [`learn_actix_web::helper::messages::LOGIN_FAILURE_MSG`].
/// 
/// * Route: ``http://localhost:5000/api/login``
/// * Method: ``POST``
/// * Content Type: ``application/json``
/// * Body: ``email=suzette.pettey.10024@gmail.com&password=passworKKd``
/// 
#[actix_web::test]
async fn post_login_json_failure_2() {
    let test_app = &spawn_app().await;

    let client = common::reqwest_client();

    let mut json_data = HashMap::new();
    json_data.insert("email", "suzette.pettey.10024@gmail.com");
    json_data.insert("password", "passworKKd");

    let response = client
        .post(make_api_url(&test_app.app_url, "/login"))
        .json(&json_data)
        .send()
        .await
        .expect("Failed to execute request.");

    common::assert_access_token_not_in_header(&response);
    common::assert_access_token_not_in_cookie(&response);    
    common::assert_json_failure(response, StatusCode::UNAUTHORIZED, 
        LOGIN_FAILURE_MSG, false).await;     
}

/// No email field. Returns login JSON [`learn_actix_web::bh_utils::api_status::ApiStatus`] 
/// with code 500 and message "missing field `email`".
/// 
/// * Route: ``http://localhost:5000/api/login``
/// * Method: ``POST``
/// * Content Type: ``application/json``
/// (``application/json; charset=UTF-8``).
/// * Body: ``emXXail=suzette.pettey.10024@gmail.com&password=password``
/// 
#[actix_web::test]
async fn post_login_json_failure_3() {
    let test_app = &spawn_app().await;

    let client = common::reqwest_client();

    let mut json_data = HashMap::new();
    json_data.insert("emXXail", "suzette.pettey.10024@gmail.com");
    json_data.insert("password", "password");

    let response = client
        .post(make_api_url(&test_app.app_url, "/login"))
        .json(&json_data)
        .send()
        .await
        .expect("Failed to execute request.");

    common::assert_access_token_not_in_header(&response);
    common::assert_access_token_not_in_cookie(&response);    
    common::assert_json_failure(response, StatusCode::BAD_REQUEST, 
        "missing field `email`", true).await;
}

/// * Route: ``http://localhost:5000/api/logout``
/// * Method: ``POST``
/// * Response: ``HTML``
#[actix_web::test]
async fn post_logout_html() {
    let test_app = &spawn_app().await;
    
    let client = common::reqwest_client();

    let mut params = HashMap::new();
    params.insert("email", "chirstian.koblick.10004@gmail.com");
    params.insert("password", "password");

    // Must be in logged in state to test logout.
    let _ = client
        .post(make_api_url(&test_app.app_url, "/login"))
        .form(&params)
        .send()
        .await
        .expect("Failed to execute request.");

    let response = client
        .post(make_api_url(&test_app.app_url, "/logout"))
        .send()
        .await
        .expect("Failed to execute request.");

    common::assert_html_login_page(response).await;
}