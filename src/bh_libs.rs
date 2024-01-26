/* Date Created: 07/01/2024. */

//! Modules which are generics. None application specifics. **The plan is refactor it 
//! out into its own crate.**
//! 

// To run test for this module only:
// 
//     * cargo test bh_libs::tests
// 
pub mod api_status;

#[cfg(test)]
mod tests {
    use super::*;
    use api_status::ApiStatus;

    #[test]
    fn test_api_status() {
        let status = ApiStatus::new(200)
            .set_message("test text");

        let expected_str = String::from("{\n  \"code\": 200,\n  \"message\": \"test text\",\n  \"session_id\": null\n}");
        let serialized = serde_json::to_string_pretty(&status).unwrap();
        assert_eq!(serialized, expected_str);

        let status1: ApiStatus = serde_json::from_str(&serialized).unwrap();
        assert_eq!(status1.get_code(), 200);
        assert_eq!(status1.get_message().unwrap(), "test text");
        assert!(status1.get_session_id().is_none(), "No session_id");
    }

    #[test]
    fn test_api_status_new() {
        let status = ApiStatus::new(500);

        assert_eq!(status.get_code(), 500);
        assert!(status.get_message().is_none(), "No text");
        assert!(status.get_session_id().is_none(), "No session_id");
    }

    #[test]
    fn test_api_status_set() {
        let status = ApiStatus::new(200)
            .set_message("test message")
            .set_session_id("abcd-efgh");

        assert_eq!(status.get_code(), 200);
        assert_eq!(status.get_message().unwrap(), "test message");
        assert_eq!(status.get_session_id().unwrap(), "abcd-efgh");
    }
}