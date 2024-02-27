/* Date Created: 27/12/2023. */

//! Integration test cases for endpoint handler methods defined in handlers.rs.
//! 
//! Test the following routes:
//! 
//! * Route: ``http://localhost:5000/data/employees``
//! * Method: ``POST``
//! * Content Type: ``application/json``
//! * Body: ``{"last_name": "%chi", "first_name": "%ak"}``
//! 
//! * Route: ``http://localhost:5000/data/employees/%chi/%ak``; i.e., /employees/{last_name}/{first_name}.
//! * Method: ``GET``
//! 
//! * Route: ``http://localhost:5000/ui/employees``
//! * Method: ``POST``
//! * Content Type: ``application/x-www-form-urlencoded; charset=UTF-8``
//! * Body: ``last_name=%chi&first_name=%ak``
//! 
//! * Route: ``http://localhost:5000/ui/employees/%chi/%ak``; i.e., /employees/{last_name}/{first_name}.
//! * Method: ``GET``
//! 
//! * Route: ``http://localhost:5000/helloemployee/%chi/%ak``; i.e., /employees/{last_name}/{first_name}.
//! * Method: ``GET``
//! 
//! To run test for this module only: 
//! 
//!     * cargo test --test test_handlers
//! 
//! To run a specific test method: 
//! 
//!     * cargo test post_employees_json1 -- --exact
//!     * cargo test post_employees_json1_error_empty -- --exact
//!     * cargo test post_employees_json1_error_missing_field -- --exact
//!     * cargo test get_employees_json2 -- --exact
//!     * cargo test post_employees_html1 -- --exact
//!     * cargo test post_employees_html1_error_empty -- --exact
//!     * cargo test post_employees_html1_missing_field -- --exact
//!     * cargo test get_employees_html2 -- --exact
//!     * cargo test get_helloemployee_has_data -- --exact
//!     * cargo test get_helloemployee_no_data -- --exact
//!     * cargo test post_employees_json1_no_access_token -- --exact
//!     * cargo test get_employees_json2_no_access_token -- --exact
//!     * cargo test get_employees_json2_with_content_type_no_access_token -- --exact
//!     * cargo test post_employees_html1_no_access_token -- --exact
//!     * cargo test get_employees_html2_no_access_token -- --exact
//!     * cargo test get_helloemployee_has_data_no_access_token -- --exact
//!
use std::collections::HashMap;
use time::macros::date;
use actix_web::http::{StatusCode, header, header::ContentType};
use learn_actix_web::models::Employee;

mod common;
use common::{spawn_app, JWT_SECS_VALID_FOR, make_full_url, make_data_url, make_ui_url};

use learn_actix_web::helper::messages::UNAUTHORISED_ACCESS_MSG;

#[actix_web::test]
async fn dummy_test() {
    let b: bool = true;
    assert_eq!(b, true);
}

/// * Route: ``http://localhost:5000/data/employees``
/// * Method: ``POST``
/// * Content Type: ``application/json``
/// * Body: ``{"last_name": "%chi", "first_name": "%ak"}``
#[actix_web::test]
async fn post_employees_json1() {
    let test_app = &spawn_app().await;

    let client = common::reqwest_client();

    let mut json_data = HashMap::new();
    json_data.insert("last_name", "%chi");
    json_data.insert("first_name", "%ak");

    let response = client
        .post(make_data_url(&test_app.app_url, "/employees"))
        .header(header::AUTHORIZATION, &test_app.mock_access_token(JWT_SECS_VALID_FOR))
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

/// * Route: ``http://localhost:5000/data/employees``
/// * Method: ``POST``
/// * Content Type: not set.
/// * Body: not set.
#[actix_web::test]
async fn post_employees_json1_error_empty() {
    let test_app = &spawn_app().await;

    let client = common::reqwest_client();

    let response = client
        .post(make_data_url(&test_app.app_url, "/employees"))
        .header(header::AUTHORIZATION, &test_app.mock_access_token(JWT_SECS_VALID_FOR))
        // .header(header::CONTENT_TYPE, ContentType::json().to_string())
        .send()
        .await
        .expect("Failed to execute request.");

    // NOTE: "Content type error" message.
    common::assert_json_failure(response, StatusCode::BAD_REQUEST, "Content type error", false).await;
}

/// * Route: ``http://localhost:5000/data/employees``
/// * Method: ``POST``
/// * Content Type: ``application/json``
/// * Body: ``{"lasXXXt_name": "%chi", "first_name": "%ak"}``
#[actix_web::test]
async fn post_employees_json1_error_missing_field() {
    let test_app = &spawn_app().await;

    let client = common::reqwest_client();

    let mut json_data = HashMap::new();
    json_data.insert("lasXXXt_name", "%chi");
    json_data.insert("first_name", "%ak");

    let response = client
        .post(make_data_url(&test_app.app_url, "/employees"))
        .header(header::AUTHORIZATION, &test_app.mock_access_token(JWT_SECS_VALID_FOR))
        .json(&json_data)
        .send()
        .await
        .expect("Failed to execute request.");

    common::assert_json_failure(response, StatusCode::BAD_REQUEST, 
        "missing field `last_name`", true).await;
}

/// * Route: ``http://localhost:5000/data/employees/%chi/%ak``; i.e., /employees/{last_name}/{first_name}.
/// * Method: ``GET``
#[actix_web::test]
async fn get_employees_json2() {
    let test_app = &spawn_app().await;

    let client = common::reqwest_client();

    let response = client
        .get(make_data_url(&test_app.app_url, "/employees/%chi/%ak"))
        .header(header::AUTHORIZATION, &test_app.mock_access_token(JWT_SECS_VALID_FOR))
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

/// * Route: ``http://localhost:5000/ui/employees``
/// * Method: ``POST``
/// * Content Type: ``application/x-www-form-urlencoded; charset=UTF-8``
/// * Body: ``last_name=%chi&first_name=%ak``
#[actix_web::test]
async fn post_employees_html1() {
    let test_app = &spawn_app().await;

    let client = common::reqwest_client();

    let mut params = HashMap::new();
    params.insert("last_name", "%chi");
    params.insert("first_name", "%ak");

    let response = client
        .post(make_ui_url(&test_app.app_url, "/employees"))
        .header(header::AUTHORIZATION, &test_app.mock_access_token(JWT_SECS_VALID_FOR))
        .form(&params)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::OK);

    let res = response.text().await;
    assert!(res.is_ok(), "Should have a HTML response.");

    // This should now always succeed.
    if let Ok(html) = res {
        assert!(html.contains("<td>Siamak</td>"), "HTML: first name Siamak not found.");
        assert!(html.contains("<td>Bernardeschi</td>"), "HTML: last name Bernardeschi not found.");
    }
}

/// * Route: ``http://localhost:5000/ui/employees``
/// * Method: ``POST``
/// * Content Type: not set.
/// * Body: not set.
#[actix_web::test]
async fn post_employees_html1_error_empty() {
    let test_app = &spawn_app().await;

    let client = common::reqwest_client();

    let response = client
        .post(make_ui_url(&test_app.app_url, "/employees"))
        .header(header::AUTHORIZATION, &test_app.mock_access_token(JWT_SECS_VALID_FOR))
        .send()
        .await
        .expect("Failed to execute request.");

    //NOTE: "Content type error." message.
    common::assert_json_failure(response, StatusCode::BAD_REQUEST, "Content type error.", false).await;
}

/// * Route: ``http://localhost:5000/ui/employees``
/// * Method: ``POST``
/// * Content Type: ``application/x-www-form-urlencoded; charset=UTF-8``
/// * Body: ``last_name=%chi&first_YYYname=%ak``
#[actix_web::test]
async fn post_employees_html1_missing_field() {
    let test_app = &spawn_app().await;

    let client = common::reqwest_client();

    let mut params = HashMap::new();
    params.insert("last_name", "%chi");
    params.insert("first_YYYname", "%ak");

    let response = client
        .post(make_ui_url(&test_app.app_url, "/employees"))
        .header(header::AUTHORIZATION, &test_app.mock_access_token(JWT_SECS_VALID_FOR))
        .form(&params)
        .send()
        .await
        .expect("Failed to execute request.");

    common::assert_json_failure(response, StatusCode::BAD_REQUEST, 
        "missing field `first_name`", true).await;
}

/// * Route: ``http://localhost:5000/ui/employees/%chi/%ak``; i.e., /employees/{last_name}/{first_name}.
/// * Method: ``GET``
#[actix_web::test]
async fn get_employees_html2() {
    let test_app = &spawn_app().await;

    let client = common::reqwest_client();

    let response = client
        .get(make_ui_url(&test_app.app_url, "/employees/%chi/%ak"))
        .header(header::AUTHORIZATION, &test_app.mock_access_token(JWT_SECS_VALID_FOR))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::OK);

    let res = response.text().await;
    assert!(res.is_ok(), "Should have a HTML response.");

    // This should now always succeed.
    if let Ok(html) = res {
        assert!(html.contains("<td>Siamak</td>"), "HTML: first name Siamak not found.");
        assert!(html.contains("<td>Bernardeschi</td>"), "HTML: last name Bernardeschi not found.");
    }
}

/// * Route: ``http://localhost:5000/helloemployee/%chi/%ak``; i.e., /helloemployee/{last_name}/{first_name}.
/// * Method: ``GET``
#[actix_web::test]
async fn get_helloemployee_has_data() {
    let test_app = &spawn_app().await;

    let client = common::reqwest_client();

    let response = client
        .get(make_full_url(&test_app.app_url, "/helloemployee/%chi/%ak"))
        .header(header::AUTHORIZATION, &test_app.mock_access_token(JWT_SECS_VALID_FOR))
        .send()
        .await
        .expect("Failed to execute request.");    

    assert_eq!(response.status(), StatusCode::OK);

    let res = response.text().await;
    assert!(res.is_ok(), "Should have a HTML response.");

    // This should now always succeed.
    if let Ok(html) = res {
        assert!(html.contains("Hi first employee found"), "HTML response error.");
    }
}

/// * Route: ``http://localhost:5000/helloemployee/%xxx/%xxx``; i.e., /employees/{last_name}/{first_name}.
/// * Method: ``GET``
#[actix_web::test]
async fn get_helloemployee_no_data() {
    let test_app = &spawn_app().await;

    let client = common::reqwest_client();

    let response = client
        .get(make_full_url(&test_app.app_url, "/helloemployee/%xx/%yy"))
        .header(header::AUTHORIZATION, &test_app.mock_access_token(JWT_SECS_VALID_FOR))
        .send()
        .await
        .expect("Failed to execute request.");    

    assert_eq!(response.status(), StatusCode::OK);

    let res = response.text().await;
    assert!(res.is_ok(), "Should have a HTML response.");

    // This should now always succeed.
    if let Ok(html) = res {
        assert!(html.contains("No employee found"), "HTML response error.");
    }
}

/// * Route: ``http://localhost:5000/data/employees``
/// * Method: ``POST``
/// * Content Type: ``application/json``
/// * Body: ``{"last_name": "%chi", "first_name": "%ak"}``
#[actix_web::test]
async fn post_employees_json1_no_access_token() {
    let test_app = &spawn_app().await;

    let client = common::reqwest_client();

    let mut json_data = HashMap::new();
    json_data.insert("last_name", "%chi");
    json_data.insert("first_name", "%ak");

    let response = client
        .post(make_data_url(&test_app.app_url, "/employees"))
        //.post(make_data_url("http://localhost:5000", "/employees"))
        .json(&json_data)
        .send()
        .await
        .expect("Failed to execute request.");

    common::assert_access_token_not_in_header(&response);
    common::assert_access_token_not_in_cookie(&response);
    common::assert_json_failure(response, StatusCode::UNAUTHORIZED,
        UNAUTHORISED_ACCESS_MSG, false).await;
}

/// * Route: ``http://localhost:5000/data/employees/%chi/%ak``; i.e., /employees/{last_name}/{first_name}.
/// * Method: ``GET``
/// 
/// Note, test that when content type is blank, authentication middleware redirected to 
/// "/ui/login" results in HTML response, even though this route is JSON response route
/// when the request is successfully served.
/// 
/// See get_employees_json2_with_content_type_no_access_token() where content type is set.
/// 
#[actix_web::test]
async fn get_employees_json2_no_access_token() {
    let test_app = &spawn_app().await;

    let client = common::reqwest_client();

    let response = client
        .get(make_data_url(&test_app.app_url, "/employees/%chi/%ak"))
        .send()
        .await
        .expect("Failed to execute request.");

    common::assert_access_token_not_in_header(&response);
    common::assert_access_token_not_in_cookie(&response);
    // ==> See get_employees_json2_with_content_type_no_access_token().
    common::assert_redirected_html_login_page(response, 
        StatusCode::UNAUTHORIZED, UNAUTHORISED_ACCESS_MSG).await;
}

/// * Route: ``http://localhost:5000/data/employees/%chi/%ak``; i.e., /employees/{last_name}/{first_name}.
/// * Method: ``GET``
/// 
/// Note, test that when content type is blank, authentication middleware redirected to 
/// "/ui/login" results in HTML response, even though this route is JSON response route
/// when the request is successfully served.
/// 
/// When content type is set to ``application/json``, the failure response is in JSON.
/// 
/// See get_employees_json2_no_access_token() where content type is blank.
/// 
#[actix_web::test]
async fn get_employees_json2_with_content_type_no_access_token() {
    let test_app = &spawn_app().await;

    let client = common::reqwest_client();

    let response = client        
        .get(make_data_url(&test_app.app_url, "/employees/%chi/%ak"))
        // ==> This is the test!! See get_employees_json2_no_access_token().
        .header(header::CONTENT_TYPE, ContentType::json().to_string())
        .send()
        .await
        .expect("Failed to execute request.");

    common::assert_access_token_not_in_header(&response);
    common::assert_access_token_not_in_cookie(&response);
    // ==> This is the test!! See get_employees_json2_no_access_token().
    common::assert_json_failure(response, StatusCode::UNAUTHORIZED,
        UNAUTHORISED_ACCESS_MSG, false).await;
}

/// * Route: ``http://localhost:5000/ui/employees``
/// * Method: ``POST``
/// * Content Type: ``application/x-www-form-urlencoded; charset=UTF-8``
/// * Body: ``last_name=%chi&first_name=%ak``
#[actix_web::test]
async fn post_employees_html1_no_access_token() {
    let test_app = &spawn_app().await;

    let client = common::reqwest_client();

    let mut params = HashMap::new();
    params.insert("last_name", "%chi");
    params.insert("first_name", "%ak");

    let response = client
        .post(make_ui_url(&test_app.app_url, "/employees"))
        .form(&params)
        .send()
        .await
        .expect("Failed to execute request.");

    common::assert_access_token_not_in_header(&response);
    common::assert_access_token_not_in_cookie(&response);
    common::assert_redirected_html_login_page(response, 
        StatusCode::UNAUTHORIZED, UNAUTHORISED_ACCESS_MSG).await;
}

/// * Route: ``http://localhost:5000/ui/employees/%chi/%ak``; i.e., /employees/{last_name}/{first_name}.
/// * Method: ``GET``
#[actix_web::test]
async fn get_employees_html2_no_access_token() {
    let test_app = &spawn_app().await;

    let client = common::reqwest_client();

    let response = client
        .get(make_ui_url(&test_app.app_url, "/employees/%chi/%ak"))
        .send()
        .await
        .expect("Failed to execute request.");

    common::assert_access_token_not_in_header(&response);
    common::assert_access_token_not_in_cookie(&response);
    common::assert_redirected_html_login_page(response, 
        StatusCode::UNAUTHORIZED, UNAUTHORISED_ACCESS_MSG).await;
}

/// * Route: ``http://localhost:5000/helloemployee/%chi/%ak``; i.e., /helloemployee/{last_name}/{first_name}.
/// * Method: ``GET``
#[actix_web::test]
async fn get_helloemployee_has_data_no_access_token() {
    let test_app = &spawn_app().await;

    let client = common::reqwest_client();

    let response = client
        .get(make_full_url(&test_app.app_url, "/helloemployee/%chi/%ak"))
        .send()
        .await
        .expect("Failed to execute request.");    

    common::assert_access_token_not_in_header(&response);
    common::assert_access_token_not_in_cookie(&response);
    common::assert_redirected_html_login_page(response, 
        StatusCode::UNAUTHORIZED, UNAUTHORISED_ACCESS_MSG).await;
}
