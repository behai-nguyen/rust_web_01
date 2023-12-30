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
use std::collections::HashMap;
use time::macros::date;
use actix_web::http::StatusCode;
use learn_actix_web::models::Employee;

mod common;
use common::{spawn_app, make_full_url, make_data_url, make_ui_url};

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
    let root_url = &spawn_app();

    let client = reqwest::Client::new();

    let mut json_data = HashMap::new();
    json_data.insert("last_name", "%chi");
    json_data.insert("first_name", "%ak");

    let response = client
        .post(make_data_url(root_url, "/employees"))
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

/// * Route: ``http://localhost:5000/data/employees/%chi/%ak``; i.e., /employees/{last_name}/{first_name}.
/// * Method: ``GET``
#[actix_web::test]
async fn get_employees_json2() {
    let root_url = &spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(make_data_url(root_url, "/employees/%chi/%ak"))
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
    let root_url = &spawn_app();    

    let client = reqwest::Client::new();

    let mut params = HashMap::new();
    params.insert("last_name", "%chi");
    params.insert("first_name", "%ak");

    let response = client
        .post(make_ui_url(root_url, "/employees"))
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

/// * Route: ``http://localhost:5000/ui/employees/%chi/%ak``; i.e., /employees/{last_name}/{first_name}.
/// * Method: ``GET``
#[actix_web::test]
async fn get_employees_html2() {
    let root_url = &spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(make_ui_url(root_url, "/employees/%chi/%ak"))
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
    let root_url = &spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(make_full_url(root_url, "/helloemployee/%chi/%ak"))
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

    /*
    dotenv().ok();

    let config = config::Config::init();

    assert_eq!(config.database_url, "mysql://root:pcb.2176310315865259@localhost:3306/employees");

    let pool = task::block_on(database::get_mysql_pool(config.max_connections, &config.database_url));

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState {
                db: pool.clone()
            }))        
    )
    .await;

    let req = test::TestRequest::get().uri("/helloemployee/%chi/%ak").to_request();
    let resp: Result<actix_web::dev::ServiceResponse, actix_web::Error> = test::try_call_service(&app, req).await;

    match resp {
        Err(_err) => assert_eq!(false, true),
        Ok(r) => {
            assert_eq!(r.status(), StatusCode::NOT_FOUND);
        }
    }
    */
}

/// * Route: ``http://localhost:5000/helloemployee/%xxx/%xxx``; i.e., /employees/{last_name}/{first_name}.
/// * Method: ``GET``
#[actix_web::test]
async fn get_helloemployee_no_data() {
    let root_url = &spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(make_full_url(root_url, "/helloemployee/%xx/%yy"))
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
