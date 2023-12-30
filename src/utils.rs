/* Date Created: 16/10/2023. */

//! Implements some generic utility functions.
//! 
//! # References
//! 
//! * [Date in a custom format](https://serde.rs/custom-date-format.html#date-in-a-custom-format).
//! 

/// Serialises MySQL date into Australian date format ``dd/mm/yyyy``.
/// Deserialises Australian date format ``dd/mm/yyyy`` to MySQL date ``yyyy-mm-dd``.
pub mod australian_date_format {
    use sqlx::types::time::Date;
    use time::macros::format_description;
    use serde::{self, Serializer, Deserialize, Deserializer};

    pub fn serialize<S>(
        date: &Date,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let format = format_description!("[day]/[month]/[year]");
        let s = &date.format(&format).unwrap();
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<Date, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let format = format_description!("[day]/[month]/[year]");
        match Date::parse(&s, &format).ok() {
            Some(dt) => Ok(dt),
            None => Err(serde::de::Error::custom(format!("Error deserialise {} to YYYY-MM-DD", &s)))
        }
    }
}
