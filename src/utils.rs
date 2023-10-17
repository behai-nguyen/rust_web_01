/* Date Created: 16/10/2023. */

//! Implements some generic utility functions.
//! 
//! # References
//! 
//! * [Date in a custom format](https://serde.rs/custom-date-format.html#date-in-a-custom-format).
//! 

/// Serialises MySQL date into Australian date format ``dd/mm/yyyy``.
pub mod australian_date_format {
    use sqlx::types::time::Date;
    use time::macros::format_description;
    use serde::{self, Serializer};

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
}
