//! Unit tests for the Gmail client module.
//!
//! These tests focus on testing the individual components and methods of the Gmail client
//! that can be tested without requiring actual Gmail API calls.

/// Test module for Gmail client functionality
mod gmail_client_tests {
    use cull_gmail::ClientConfig;

    /// Test the default max results constant
    #[test]
    fn test_default_max_results() {
        let default_max = cull_gmail::DEFAULT_MAX_RESULTS;
        assert_eq!(default_max, "200");

        // Verify it can be parsed as u32
        let parsed: u32 = default_max
            .parse()
            .expect("DEFAULT_MAX_RESULTS should be a valid u32");
        assert_eq!(parsed, 200);
    }

    /// Test that DEFAULT_MAX_RESULTS is a reasonable value for Gmail API
    #[test]
    fn test_default_max_results_range() {
        let default_max: u32 = cull_gmail::DEFAULT_MAX_RESULTS
            .parse()
            .expect("DEFAULT_MAX_RESULTS should be a valid u32");

        // Gmail API supports up to 500 results per page
        assert!(default_max > 0, "Max results should be positive");
        assert!(
            default_max <= 500,
            "Max results should not exceed Gmail API limit"
        );
        assert!(
            default_max >= 10,
            "Max results should be reasonable for performance"
        );
    }

    /// Test that ClientConfig builder compiles and creates a config
    #[test]
    fn test_client_config_builder_works() {
        let _config = ClientConfig::builder()
            .with_client_id("test-id")
            .with_client_secret("test-secret")
            .build();
        // Test passes if we reach here without panicking
    }
}
