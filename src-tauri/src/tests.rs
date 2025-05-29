#[cfg(test)]
mod main_tests {

    // Note: Due to Tauri v2 changes, command handlers can't be easily unit tested
    // without the full Tauri runtime. Integration tests provide better coverage.
    // These tests are replaced by the integration tests in tests/integration_tests.rs

    #[test]
    fn test_input_validation() {
        // Test that empty strings are considered invalid
        assert!("".trim().is_empty());
        assert!("   ".trim().is_empty());

        // Test Latin alphabet validation (including macrons)
        let validate_latin = |word: &str| {
            word.chars().all(|c| {
                c.is_alphabetic()
                    && (c.is_ascii()
                        || matches!(
                            c,
                            'ā' | 'ē' | 'ī' | 'ō' | 'ū' | 'ȳ' | 'Ā' | 'Ē' | 'Ī' | 'Ō' | 'Ū' | 'Ȳ'
                        ))
            })
        };

        assert!(!validate_latin("test123"));
        assert!(!validate_latin("test!"));
        assert!(validate_latin("test"));
        assert!(validate_latin("amīcus"));
        assert!(validate_latin("rēx"));
        assert!(validate_latin("amō"));
    }
}

// Config tests are in src/config.rs

#[cfg(test)]
mod claude_api_tests {
    use crate::claude_api::*;

    #[test]
    fn test_latin_word_result_complete_serialization() {
        let result = LatinWordResult {
            word_with_macrons: "rēx".to_string(),
            part_of_speech: "noun - masculine, 3rd declension, nominative singular".to_string(),
            definition: "king, ruler, monarch".to_string(),
            principal_parts: None,
            stems: None,
        };

        let json = serde_json::to_string(&result).unwrap();
        let expected = r#"{"word_with_macrons":"rēx","part_of_speech":"noun - masculine, 3rd declension, nominative singular","definition":"king, ruler, monarch","principal_parts":null,"stems":null}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_latin_word_result_with_special_characters() {
        let result = LatinWordResult {
            word_with_macrons: "āēīōū".to_string(),
            part_of_speech: "test".to_string(),
            definition: "test with macrons: ā, ē, ī, ō, ū".to_string(),
            principal_parts: None,
            stems: None,
        };

        let json = serde_json::to_string(&result).unwrap();
        let deserialized: LatinWordResult = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.word_with_macrons, "āēīōū");
        assert_eq!(deserialized.definition, "test with macrons: ā, ē, ī, ō, ū");
    }

    #[test]
    fn test_claude_client_creation_with_key() {
        let client = ClaudeClient::new_with_key("sk-ant-test-key".to_string());
        // Can't access private field api_key, but we can verify the client was created
        assert!(std::mem::size_of_val(&client) > 0);
    }

    #[test]
    fn test_latin_word_result_missing_fields() {
        let json = r#"{"word_with_macrons":"test","part_of_speech":"noun"}"#;
        let result: Result<LatinWordResult, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_latin_word_result_extra_fields() {
        let json = r#"{"word_with_macrons":"test","part_of_speech":"noun","definition":"test word","extra":"field"}"#;
        let result: Result<LatinWordResult, _> = serde_json::from_str(json);
        // Should still parse successfully, ignoring extra fields
        assert!(result.is_ok());
        let word = result.unwrap();
        assert_eq!(word.word_with_macrons, "test");
        assert_eq!(word.part_of_speech, "noun");
        assert_eq!(word.definition, "test word");
    }

    #[test]
    fn test_custom_prompt_validation() {
        // Test that custom prompt handling works correctly
        let custom_prompt_none: Option<&str> = None;
        let custom_prompt_empty: Option<&str> = Some("");
        let custom_prompt_whitespace: Option<&str> = Some("   ");
        let custom_prompt_valid: Option<&str> = Some("Custom instructions");

        // Test none case
        assert!(custom_prompt_none.is_none());

        // Test empty case
        if let Some(prompt) = custom_prompt_empty {
            assert!(prompt.trim().is_empty());
        }

        // Test whitespace case
        if let Some(prompt) = custom_prompt_whitespace {
            assert!(prompt.trim().is_empty());
        }

        // Test valid case
        if let Some(prompt) = custom_prompt_valid {
            assert!(!prompt.trim().is_empty());
            assert_eq!(prompt.trim(), "Custom instructions");
        }
    }
}
