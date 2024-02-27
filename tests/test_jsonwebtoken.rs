/* Date Created: 21/02/2024. */

//! JSON Web Token scenario tests.
//! 
//! To run test for this module only: 
//! 
//!     * cargo test --test test_jsonwebtoken
//! 
//! To run a specific test method: 
//! 
//!     * cargo test login_then_request_data -- --exact
//!     * cargo test invalid_token_request_data -- --exact
//!     * cargo test expired_token_request_data -- --exact
//!     * cargo test token_update_sequence -- --exact
//! 
use std::collections::HashMap;
use actix_web::http::{StatusCode, header};
use time::macros::date;

mod common;
use common::{spawn_app, make_api_url, make_data_url, jwt_secret_key, make_ui_url};

use learn_actix_web::models::Employee;

use learn_actix_web::helper::messages::{ 
    // TOKEN_INVALID_MSG,
    TOKEN_EXPIRED_MSG,
    TOKEN_OTHER_ERR_MSG,
};

use learn_actix_web::helper::jwt_utils::{
    JWTPayload,
    make_token,
    make_bearer_token,
    decode_token,
};

/// Test the following scenario:
/// 
///    1. Login
///    2. Extract the access token
///    3. Post to https://0.0.0.0:5000/data/employees
///       Body: {"last_name": "%chi", "first_name": "%ak"}
///    4. Should succeed.
/// 
#[actix_web::test]
async fn login_then_request_data() {
    let test_app = &spawn_app().await;

    let client = common::reqwest_client();

    // 1. Login
    let mut json_data = HashMap::new();
    json_data.insert("email", "saniya.kalloufi.10008@gmail.com");
    json_data.insert("password", "password");

    let response = client
        .post(make_api_url(&test_app.app_url, "/login"))
        .json(&json_data)
        .send()
        .await
        .expect("Failed to execute request.");

    // 2. Extract the access token
    let token = response.headers().get(header::AUTHORIZATION).unwrap().to_str().unwrap();

    // 3. Post to https://0.0.0.0:5000/data/employees
    let mut json_data = HashMap::new();
    json_data.insert("last_name", "%chi");
    json_data.insert("first_name", "%ak");

    let response = client
        .post(make_data_url(&test_app.app_url, "/employees"))
        .header(header::AUTHORIZATION, make_bearer_token(token))
        .json(&json_data)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::OK);

    let res = response.json::<Vec<Employee>>().await;
    assert!(res.is_ok(), "Should have a JSON response.");

    // This should now always succeed.    
    if let Ok(json_list) = res {
        assert!(json_list.len() >= 1, "Should have at least one employee.");

        let emp = &json_list[0];
        assert_eq!(emp.emp_no, 67115);

        assert_eq!(emp.birth_date, date!(1955 - 12 - 14));
        assert_eq!(emp.hire_date, date!(1985 - 04 - 26));
    }
}

/// Test the following scenario:
/// 
///    1. Using an email as access token.
///    2. Post to https://0.0.0.0:5000/data/employees
///       Body: {"last_name": "%chi", "first_name": "%ak"}
///    3. Should fail.
/// 
#[actix_web::test]
async fn invalid_token_request_data() {
    let test_app = &spawn_app().await;

    let client = common::reqwest_client();

    // 1. Extract the access token
    let token = "Bearer.saniya.kalloufi.10008@gmail.com";

    // 3. Post to https://0.0.0.0:5000/data/employees
    let mut json_data = HashMap::new();
    json_data.insert("last_name", "%chi");
    json_data.insert("first_name", "%ak");

    let response = client
        .post(make_data_url(&test_app.app_url, "/employees"))
        .header(header::AUTHORIZATION, token)
        .json(&json_data)
        .send()
        .await
        .expect("Failed to execute request.");

    // 3. Should fail.
    common::assert_json_failure(response, StatusCode::UNAUTHORIZED, TOKEN_OTHER_ERR_MSG, false).await;
}

/// Test the following scenario:
/// 
///    1. Manually create a token which is valid for 5 seconds.
///    2. Sleep for 7 seconds to expire the token.
///    3. Post to https://0.0.0.0:5000/data/employees
///       Body: {"last_name": "%chi", "first_name": "%ak"}
/// 
///       using the expired token.
/// 
///    4. Should fail.
/// 
#[actix_web::test]
async fn expired_token_request_data() {
    let test_app = &spawn_app().await;

    let client = common::reqwest_client();

    // 1. Manually create a token which is valid for 5 seconds.
    let token = make_bearer_token( &make_token("mayumi.schueller.10054@gmail.com", jwt_secret_key().as_ref(), 5) );

    // 2. Sleep for 7 seconds to expire the token.
    let sleep_time = std::time::Duration::from_secs(7);
    std::thread::sleep(sleep_time);

    // 3. Post to https://0.0.0.0:5000/data/employees using the expired token.
    let mut json_data = HashMap::new();
    json_data.insert("last_name", "%chi");
    json_data.insert("first_name", "%ak");

    let response = client
        .post(make_data_url(&test_app.app_url, "/employees"))
        .header(header::AUTHORIZATION, token)
        .json(&json_data)
        .send()
        .await
        .expect("Failed to execute request.");

    // 4. Should fail.
    common::assert_json_failure(response, StatusCode::UNAUTHORIZED, TOKEN_EXPIRED_MSG, false).await;
}

/// Test the following scenario:
/// 
///    1. Login.
///    2. Make multiple requests. Sleep between requests.
///    3. After each request, extract the access token and test that they are 
///       different to the immediate previous one.
///    4. Also the immediate previous one is still valid.
/// 
#[actix_web::test]
async fn token_update_sequence() {
    let email = "saniya.kalloufi.10008@gmail.com";

    let test_app = &spawn_app().await;

    let client = common::reqwest_client();

    // 1. Login
    let mut json_data = HashMap::new();
    json_data.insert("email", email);
    json_data.insert("password", "password");

    let response = client
        .post(make_api_url(&test_app.app_url, "/login"))
        .json(&json_data)
        .send()
        .await
        .expect("Failed to execute request.");

    // Original token.
    let token_1 = response.headers().get(header::AUTHORIZATION).unwrap().to_str().unwrap();

    // Sleep for 2 seconds to expire the token.
    let sleep_time = std::time::Duration::from_secs(2);
    std::thread::sleep(sleep_time);    

    // 2. Make another request. Post to https://0.0.0.0:5000/data/employees
    let mut json_data1 = HashMap::new();
    json_data1.insert("last_name", "%chi");
    json_data1.insert("first_name", "%ak");

    let response = client
        .post(make_data_url(&test_app.app_url, "/employees"))
        .header(header::AUTHORIZATION, make_bearer_token(token_1))
        .json(&json_data1)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::OK);

    // let res = response.json::<Vec<Employee>>().await;
    // assert!(res.is_ok(), "Should have a JSON response.");

    // Updated token.
    let token_2 = response.headers().get(header::AUTHORIZATION).unwrap().to_str().unwrap();

    // token_1 != token_2.
    assert_ne!(token_1, token_2, "token_1, token_2 should be different.");

    // Sleep for 2 seconds to expire the token.
    let sleep_time = std::time::Duration::from_secs(2);
    std::thread::sleep(sleep_time);    

    // 3. Make another request. Post to https://0.0.0.0:5000//employees/%ri%/%was%
    let response = client
        .get(make_ui_url(&test_app.app_url, "/employees/%ri%/%was%"))
        .header(header::AUTHORIZATION, make_bearer_token(token_2))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::OK);
    
    // let res = response.text().await;
    // assert!(res.is_ok(), "Should have a HTML response.");    

    // Updated token.
    let token_3 = response.headers().get(header::AUTHORIZATION).unwrap().to_str().unwrap();

    // token_2 != token_3.
    assert_ne!(token_2, token_3, "token_2, token_3 should be different.");

    // Assert that all tokens are still valid.
    let res1 = decode_token(&token_1, jwt_secret_key().as_ref());
    assert_eq!(res1.is_ok(), true, "Decode token_1");

    let res2 = decode_token(&token_2, jwt_secret_key().as_ref());
    assert_eq!(res2.is_ok(), true, "Decode token_2");

    let res3 = decode_token(&token_3, jwt_secret_key().as_ref());
    assert_eq!(res3.is_ok(), true, "Decode token_3");

    // Extract tokens' payloads and check payloads data.
    // Check email.
    let payload1 = res1.unwrap();
    assert_eq!(payload1.email(), email, "token_1 email");

    let payload2 = res2.unwrap();
    assert_eq!(payload2.email(), email, "token_1 email");

    let payload3 = res3.unwrap();
    assert_eq!(payload3.email(), email, "token_1 email");

    // Assert tokens issued at are in ascending order.
    assert_eq!(payload1.issued_at() == payload2.issued_at(), true, "payload1.issued_at() == payload2.issued_at()");
    assert_eq!(payload2.issued_at() == payload3.issued_at(), true, "payload2.issued_at() == payload3.issued_at()");

    // Assert tokens expiry are in ascending order.
    assert_eq!(payload1.expiry() < payload2.expiry(), true, "payload1.expiry() < payload2.expiry()");
    assert_eq!(payload2.expiry() < payload3.expiry(), true, "payload2.expiry() < payload3.expiry()");

    // Assert tokens last active are in ascending order.
    assert_eq!(payload1.last_active() < payload2.last_active(), true, "payload1.last_active() < payload2.last_active()");
    assert_eq!(payload2.last_active() < payload3.last_active(), true, "payload2.last_active() < payload3.last_active()");
}