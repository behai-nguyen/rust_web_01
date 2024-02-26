/* Date Created: 16/10/2023. */

//! Loads information from ``.env`` file into a Rust structure.

/// Represents the content of ``.env`` file.
#[derive(Debug, Clone)]
pub struct Config {
    pub max_connections: u32,
    pub database_url: String,
    pub allowed_origin: String,
    /// Based on this thread: [Why would someone use ``usize`` over ``u32``?](https://users.rust-lang.org/t/why-would-someone-use-usize-over-u32/105229/33),
    /// I changed ``max_age`` to ``u32``, it results in the bellow error:
    /// 
    /// ```text
    /// pub fn max_age(mut self, max_age: impl Into<Option<usize>>) -> Cors {
    ///                                   ^^^^^^^^^^^^^^^^^^^ required by this bound in `Cors::max_age`
    /// ```
    pub max_age: usize,
    pub jwt_secret_key: String,
    pub jwt_mins_valid_for: u64,
}

impl Config {
    /// Reads the ``.env`` file content into an instance of [`Config`].
    /// 
    /// # Return
    /// 
    /// - [`Config`]
    /// 
    /// # Example
    /// 
    /// ```
    /// use dotenv::dotenv;
    /// 
    /// use learn_actix_web::config;
    /// // ...
    /// dotenv().ok();
    /// let config = config::Config::init();
    /// ```
    pub fn init() -> Config {
        Config {
            max_connections: std::env::var("MAX_CONNECTIONS")
                .expect("MAX_CONNECTIONS must be specified")
                .parse::<u32>().unwrap(),

            database_url: std::env::var("DATABASE_URL").expect("DATABASE_URL must be specified"),

            allowed_origin: std::env::var("ALLOWED_ORIGIN").expect("ALLOWED_ORIGIN must be specified"),

            max_age: std::env::var("MAX_AGE")
                .expect("MAX_AGE must be specified")
                .parse::<usize>().unwrap(),

            jwt_secret_key: std::env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY must be specified"),

            jwt_mins_valid_for: std::env::var("JWT_MINS_VALID_FOR")
                .expect("JWT_MINS_VALID_FOR must be specified")
                .parse::<u64>().unwrap(),
        }
    }
}

/// To run these tests below:
/// 
///    * cargo test config::tests
/// 
/// To run a specific test method: 
/// 
///    * cargo test config::tests::test_init -- --exact
/// 
#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use super::*;

    #[test]
    fn test_init() {
        dotenv().ok();
        let config = Config::init();

        assert_eq!(config.max_connections, 15);
        assert_eq!(config.database_url, "mysql://root:pcb.2176310315865259@localhost:3306/employees");
        assert_eq!(config.allowed_origin, "http://localhost");
        assert_eq!(config.max_age, 3600);
        assert_eq!(config.jwt_secret_key, "007: The Spy Who Loved Me");
        assert_eq!(config.jwt_mins_valid_for, 30);
    }
}