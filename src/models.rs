/* Date Created: 16/10/2023. */

//! Represents the ``employees`` table in the database: a structure which 
//! mirrors the database table, other related auxiliary structure(s), and 
//! associated CRUD method(s).
//! 

// To run all doc tests:
// 
//     * cargo test --doc models
//
// Can't run a specific doc test.
// 

use sqlx::{FromRow, Row, Pool, MySql};

use sqlx::types::time::Date;
use serde::{Serialize, Deserialize};

use crate::bh_libs::{
    australian_date::australian_date_format,
    api_status::ApiStatus
};

/// Represents the ``employees`` table in the database. Values of [`sqlx::types::time::Date`] 
/// fields are converted into Australian date format ``dd/mm/yyyy`` before
/// sending back to the client.
#[derive(FromRow, Debug, Deserialize, Serialize)]
pub struct Employee {
    pub emp_no: i32,
    pub email: String,
    #[serde(with = "australian_date_format")]
    pub birth_date: Date,
    pub first_name: String,
    pub last_name: String,    
    pub gender: String,
    #[serde(with = "australian_date_format")]
    pub hire_date: Date,
}

/// Represents a login submission. That is, user login request data are capture into this struct.
#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct EmployeeLogin {
    /// User input. The exact email which will be matched against ``employees.email``.
    pub email: String,
    /// User input. The exact plain text password. If there is matched on ``email``, this 
    /// password will be hashed, then compared to the one retrieved from database.
    pub password: String,
}

/// Represents a result data of a successful login request.
/// **Work in progress**. 
/// 
#[derive(Serialize, Deserialize, Debug)]
pub struct LoginSuccess {
    /// Logged in user email.
    pub email: String,
    /// The login authentication token.
    pub access_token: String,
    pub token_type: String,
}

/// Represents a JSON response of a successful login request.
#[derive(Serialize, Deserialize)]
pub struct LoginSuccessResponse {
    #[serde(flatten)]
    /// **Work in progress**. All API responses should have this data.
    pub api_status: ApiStatus,
    /// The actual data component of the response.
    pub data: LoginSuccess
}

/// An auxiliary structure which represents: 
/// 
/// * a JSON POST request body. E.g.: ``{"last_name": "%chi", "first_name": "%ak"}``.
/// Request content type is ``application/json``.
///
/// * a x-www-form-urlencoded POST request body. E.g.: ``last_name=%chi&first_name=%ak``.
/// Request content type is ``application/x-www-form-urlencoded; charset=UTF-8``.
/// 
/// Both ``last_name`` and ``first_name`` are partial strings. That is,
/// each should have at least either a leading or a trailing ``%`` character.
#[derive(Debug, Deserialize)]
pub struct EmployeeSearch {
    /// A MySQL partial string compared using LIKE operator.
    pub last_name: String,
    /// A MySQL partial string compared using LIKE operator.
    pub first_name: String,
}

/// Attempts to retrieve data from the ``employees`` table based on partial
/// last name and partial first name.
/// 
/// # Arguments
/// 
/// * `pool` - [`sqlx::Pool`]&lt;[`sqlx::MySql`]&gt;, an already established MySQL connection.
/// 
/// * `last_name` - Partial ``employees``'s last name. A string slice which is a MySQL partial 
/// string. I.e.: ``%chi``.
/// 
/// * `first_name` - Partial ``employees``'s first name. A string slice which is a MySQL partial 
/// string. I.e.: ``%ak``.
/// 
/// # Return
/// 
/// - [`std::vec::Vec`]&lt;[`Employee`]&gt; - which is rows from table ``employees`` which partially
/// match arguments `last_name` and `first_name`.
/// 
/// # Example, in a synchronous function:
/// 
/// ```
/// use async_std::task;
/// 
/// use learn_actix_web::database;
/// use learn_actix_web::models::get_employees;
/// 
/// fn test() {
///     //..
///     let pool = task::block_on(database::get_mysql_pool(5, "mysql://root:pcb.2176310315865259@localhost:3306/employees"));
///     let query_result = task::block_on(get_employees(&pool, "nguy%", "be%"));
/// }
/// ```
/// 
/// # Example, in an asynchronous function:
/// 
/// ```
/// use learn_actix_web::database;
/// use learn_actix_web::models::get_employees;
/// 
/// async fn test() {
///     //...
///     let pool = database::get_mysql_pool(5, "mysql://root:pcb.2176310315865259@localhost:3306/employees").await;
///     let query_result = get_employees(&pool, "nguy%", "be%").await;
/// }
/// ```
pub async fn get_employees(
    pool: &Pool<MySql>,
    last_name: &str,
    first_name: &str
) -> Vec<Employee> {
    sqlx::query("call get_employees(?, ?)")
    .bind(last_name).bind(first_name)
    .map(|row: sqlx::mysql::MySqlRow| { 
        Employee {
            emp_no: row.get(0),
            email: row.get(1),
            birth_date: row.get(3),
            first_name: row.get(4),
            last_name: row.get(5),
            gender: row.get(6),
            hire_date: row.get(7)
        }
    })
    .fetch_all(pool).await.unwrap()
}

/// Attempts to retrieve a single record from the ``employees`` table based on 
/// the exact email.
/// 
/// # Arguments
/// 
/// * `email` - the exact email to match against ``employees.email``.
/// 
/// # Return
/// 
/// - [`std::option::Option`]&lt;[`EmployeeLogin`]&gt; - that is, a single row
/// if found, otherwise nothing.
/// 
/// # Example, in a synchronous function:
/// 
/// ```
/// use async_std::task;
/// 
/// use learn_actix_web::database;
/// use learn_actix_web::models::select_employee;
/// 
/// fn main() {
///     let pool = task::block_on(database::get_mysql_pool(5, "mysql://root:pcb.2176310315865259@localhost:3306/employees"));
///     let query_result = task::block_on(select_employee(&pool, "chirstian.koblick.10004@gmail.com"));
/// 
///     if let Some(emp_login) = query_result {
///         println!("Employee Login {:#?}", emp_login);
///     }
///     else {
///         println!("No employee found.");
///     }
/// }
/// ```
/// 
/// # Example, in an asynchronous function:
/// 
/// ```
/// use learn_actix_web::database;
/// use learn_actix_web::models::select_employee;
///
/// #[tokio::main]
/// async fn main() {
///     let pool = database::get_mysql_pool(5, "mysql://root:pcb.2176310315865259@localhost:3306/employees").await;
///     let query_result = select_employee(&pool, "chirstian.koblick.10004@gmail.com").await;
///
///     if let Some(emp_login) = query_result {
///         println!("Employee Login {:#?}", emp_login);
///     }
///     else {
///         println!("No employee found.");
///     }
/// }
/// ```
pub async fn select_employee(
    pool: &Pool<MySql>,
    email: &str ) -> Option<EmployeeLogin> {
    let sql = format!("SELECT email, password FROM employees WHERE email = '{}'", email);

    sqlx::query(&sql)
    .map(|row: sqlx::mysql::MySqlRow| { 
        EmployeeLogin {
            email: row.get(0),
            password: row.get(1),
        }
    })
    .fetch_optional(pool).await.unwrap()
}

/// To these tests below:
/// 
///    * cargo test models::tests
/// 
/// To run a specific test method: 
/// 
///    * cargo test models::tests::test_employee_serde -- --exact
///    * cargo test models::tests::test_employee_serde_failure -- --exact
///    * cargo test models::tests::test_login_success_response -- --exact
/// 
#[cfg(test)]
mod tests {
    use time::macros::date;
    use super::*;
    use crate::helper::constants::TOKEN_TYPE;

    #[test]
    fn test_employee_serde() {
        let json_str = r#"{
            "emp_no": 67115,
            "email": "siamak.bernardeschi.67115@gmail.com",
            "birth_date": "14/12/1955",
            "first_name": "Siamak",
            "last_name": "Bernardeschi",
            "gender": "M",
            "hire_date": "26/04/1985"
        }"#;
    
        let emp: Employee = serde_json::from_str(json_str).unwrap();
        assert_eq!(emp.birth_date, date!(1955 - 12 - 14));
        assert_eq!(emp.hire_date, date!(1985 - 04 - 26));
    
        let expected_str = String::from("{\n  \"emp_no\": 67115,\n  \"email\": \"siamak.bernardeschi.67115@gmail.com\",\n  \"birth_date\": \"14/12/1955\",\n  \"first_name\": \"Siamak\",\n  \"last_name\": \"Bernardeschi\",\n  \"gender\": \"M\",\n  \"hire_date\": \"26/04/1985\"\n}");
        let serialized = serde_json::to_string_pretty(&emp).unwrap();
        assert_eq!(serialized, expected_str);
    }
    
    #[test]
    fn test_employee_serde_failure() {
        let json_str = r#"{
            "emp_no": 67115,
            "email": "siamak.bernardeschi.67115@gmail.com",
            "birth_date": "30/02/1955",
            "first_name": "Siamak",
            "last_name": "Bernardeschi",
            "gender": "M",
            "hire_date": "26/04/1985"
        }"#;
    
        match serde_json::from_str::<Employee>(json_str) {
            Ok(_emp) => assert!(true == false, "Expect deserialisation error."),
            Err(err) => assert!(err.to_string()
                .contains("Error deserialise 30/02/1955 to YYYY-MM-DD"), "Expect birth_date deserialisation error"),
        };
    }

    #[test]
    fn test_login_success_response() {
        let api_status = ApiStatus::new(200)
            .set_message("text message");
    
        let login_success = LoginSuccess {
            email: String::from("behai_nguyen@hotmail.com"),
            access_token: String::from("abcd-efgh-ijkl-mnop"),
            token_type: String::from(TOKEN_TYPE)
        };

        let lsr = LoginSuccessResponse {api_status, data: login_success};
        let lsr_str = serde_json::to_string(&lsr).unwrap();

        let lsr_obj = serde_json::from_str::<LoginSuccessResponse>(&lsr_str).unwrap();

        assert_eq!(lsr_obj.api_status.get_code(), 200);
        assert_eq!(lsr_obj.api_status.get_message(), Some(String::from("text message")));
        assert_eq!(lsr_obj.api_status.get_message().unwrap(), "text message");
        assert_eq!(lsr_obj.api_status.get_session_id(), None);
        assert_eq!(lsr_obj.data.email, "behai_nguyen@hotmail.com");
        assert_eq!(lsr_obj.data.access_token, "abcd-efgh-ijkl-mnop");    
    }
}