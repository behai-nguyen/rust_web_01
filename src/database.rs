/* Date Created: 16/10/2023. */

//! Some common database functions.

use sqlx::{mysql::MySqlPoolOptions, Pool, MySql};

/// Attempts to connect to a MySql server. If succeeded, returns a connection pool.
/// Otherwise terminates the application.
/// 
/// # Arguments
/// 
/// * `max_connections` - The maximum total number of database connections in the pool.
/// 
/// * `database_url` - MySQL database connection string.
/// 
/// # Return
/// 
/// - [`sqlx::Pool`]&lt;[`sqlx::MySql`]&gt;
/// 
/// # Example, in a synchronous function:
/// 
/// ```
/// use async_std::task;
/// 
/// use learn_actix_web::database;
/// 
/// fn test() {
///     //...
///     let pool = task::block_on(database::get_mysql_pool(5, "mysql://root:pcb.2176310315865259@localhost:3306/employees"));
/// }
/// ```
/// 
/// # Example, in an asynchronous function:
/// 
/// ```
/// use learn_actix_web::database;
/// 
/// async fn test() {
///     //...
///     let pool = database::get_mysql_pool(5, "mysql://root:pcb.2176310315865259@localhost:3306/employees").await;
/// }
/// ```
pub async fn get_mysql_pool(max_connections: u32, database_url: &str) -> Pool<MySql> {
    let _ = match MySqlPoolOptions::new()
        .max_connections(max_connections)
        .connect(database_url)
        .await
    {
        Ok(pool) => {
            tracing::debug!("🐬 Successfully connected to target MySql server!");
            return pool
        }
        Err(err) => {
            tracing::debug!("💥 Failed to connect to the target MySql server!");
            tracing::debug!("💥 Error: {:?}", err);
            std::process::exit(1);
        }
    };
}