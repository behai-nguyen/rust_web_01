/* Date Created: 07/01/2024. */

//! Modules which are generics. None application specifics. **The plan is refactor it 
//! out into its own crate.**
//! 
//! # Note
//! 
//! Specific tests within this module can be run with:
//! 
//! * ``cargo test bh_libs::tests``

pub mod api_status;

#[cfg(test)]
mod tests {
    use super::*;
    use api_status::ApiStatus;

    #[test]
    fn test_api_status() {
        let status = ApiStatus {
            code: 200,
            text: Some(String::from("test text")),
            session_id: None,
        };

        let expected_str = String::from("{\n  \"code\": 200,\n  \"text\": \"test text\",\n  \"session_id\": null\n}");
        let serialized = serde_json::to_string_pretty(&status).unwrap();
        assert_eq!(serialized, expected_str);

        let status1: ApiStatus = serde_json::from_str(&serialized).unwrap();
        assert_eq!(status1.code, 200);
        assert_eq!(status1.text.unwrap(), "test text");
        assert!(status1.session_id.is_none(), "No session_id");
    }

    #[test]
    fn test_api_status_new() {
        let status = ApiStatus::new(500);

        assert_eq!(status.code, 500);
        assert!(status.text.is_none(), "No text");
        assert!(status.session_id.is_none(), "No session_id");
    }

    #[test]
    fn test_api_status_set() {
        let status = ApiStatus::new(200)
            .set_text("test message")
            .set_session_id("abcd-efgh");

        assert_eq!(status.code, 200);
        assert_eq!(status.text.unwrap(), "test message");
        assert_eq!(status.session_id.unwrap(), "abcd-efgh");
    }
}