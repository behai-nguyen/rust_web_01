/* Date Created: 16/10/2023. */

//! Represents the ``employees`` table in the database: a structure which 
//! mirrors the database table, other related auxiliary structure(s), and 
//! associated CRUD method(s).

use sqlx::{FromRow, Row, Pool, MySql};

use sqlx::types::time::Date;
use serde::{Serialize, Deserialize};

use crate::utils;

/// Represents the ``employees`` table in the database. Values of [`sqlx::types::time::Date`] 
/// fields are converted into Australian date format ``dd/mm/yyyy`` before
/// sending back to the client.
#[derive(FromRow, Debug, Deserialize, Serialize)]
pub struct Employee {
    pub emp_no: i32,
    #[serde(with = "utils::australian_date_format")]
    pub birth_date: Date,
    pub first_name: String,
    pub last_name: String,    
    pub gender: String,
    #[serde(with = "utils::australian_date_format")]
    pub hire_date: Date,
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
/// # Example
/// 
/// ```
/// use async_std::task;
/// 
/// use learn_actix_web::database;
/// use learn_actix_web::models::get_employees;
/// 
/// //..
/// let pool = task::block_on(database::get_mysql_pool(5, "mysql://root:pcb.2176310315865259@localhost:3306/employees"));
/// let query_result = task::block_on(get_employees(&pool, "nguy%", "be%"));
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
            birth_date: row.get(1),
            first_name: row.get(2),
            last_name: row.get(3),
            gender: row.get(4),
            hire_date: row.get(5)
        }
    })
    .fetch_all(pool).await.unwrap()
}

#[cfg(test)]
mod tests {
    use time::macros::date;
    use super::*;

    #[test]
    fn test_employee_serde() {
        let json_str = r#"{
            "emp_no": 67115,
            "birth_date": "14/12/1955",
            "first_name": "Siamak",
            "last_name": "Bernardeschi",
            "gender": "M",
            "hire_date": "26/04/1985"
        }"#;
    
        let emp: Employee = serde_json::from_str(json_str).unwrap();
        assert_eq!(emp.birth_date, date!(1955 - 12 - 14));
        assert_eq!(emp.hire_date, date!(1985 - 04 - 26));
    
        let expected_str = String::from("{\n  \"emp_no\": 67115,\n  \"birth_date\": \"14/12/1955\",\n  \"first_name\": \"Siamak\",\n  \"last_name\": \"Bernardeschi\",\n  \"gender\": \"M\",\n  \"hire_date\": \"26/04/1985\"\n}");
        let serialized = serde_json::to_string_pretty(&emp).unwrap();
        assert_eq!(serialized, expected_str);
    }

    #[test]
    fn test_employee_serde_failure() {
        let json_str = r#"{
            "emp_no": 67115,
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
}
