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
#[derive(FromRow, Serialize)]
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
/// mod database;
/// mod models;
/// 
/// use models::get_employees;
/// //..
/// let pool = task::block_on(database::get_mysql_pool(5, "mysql://root:pcb.2176310315865259@localhost:3306/employees"));
/// let query_result = task::block_on(get_employees(pool, "nguy%", "be%"));
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
