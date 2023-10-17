/* Date Created: 16/10/2023. */

//! Loads information from ``.env`` file into a Rust structure.

/// Represents the content of ``.env`` file.
#[derive(Debug, Clone)]
pub struct Config {
    pub max_connections: u32,
    pub database_url: String,
    pub allowed_origin: String,
    pub max_age: usize,
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
    /// mod config;
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
        }
    }
}
