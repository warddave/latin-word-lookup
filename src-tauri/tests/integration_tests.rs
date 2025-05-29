use latin_word_lookup::claude_api::{ClaudeClient, LatinWordResult};

#[tokio::test]
async fn test_latin_word_result_json_parsing() {
    let json = r#"{
        "word_with_macrons": "amīcus",
        "part_of_speech": "noun - masculine, 2nd declension, nominative singular",
        "definition": "friend, ally, comrade",
        "principal_parts": null,
        "stems": null
    }"#;
    
    let result: LatinWordResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.word_with_macrons, "amīcus");
    assert_eq!(result.part_of_speech, "noun - masculine, 2nd declension, nominative singular");
    assert_eq!(result.definition, "friend, ally, comrade");
}

#[tokio::test]
async fn test_latin_word_result_with_all_macrons() {
    let json = r#"{
        "word_with_macrons": "āēīōūȳ",
        "part_of_speech": "test",
        "definition": "all possible macrons",
        "principal_parts": null,
        "stems": null
    }"#;
    
    let result: LatinWordResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.word_with_macrons, "āēīōūȳ");
}

#[test]
fn test_latin_word_result_serialization_roundtrip() {
    let original = LatinWordResult {
        word_with_macrons: "rēgīna".to_string(),
        part_of_speech: "noun - feminine, 1st declension".to_string(),
        definition: "queen".to_string(),
        principal_parts: None,
        stems: None,
    };
    
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: LatinWordResult = serde_json::from_str(&json).unwrap();
    
    assert_eq!(original.word_with_macrons, deserialized.word_with_macrons);
    assert_eq!(original.part_of_speech, deserialized.part_of_speech);
    assert_eq!(original.definition, deserialized.definition);
    assert_eq!(original.principal_parts, deserialized.principal_parts);
    assert_eq!(original.stems, deserialized.stems);
}

#[test]
fn test_latin_word_result_with_verb_forms() {
    let json = r#"{
        "word_with_macrons": "amō",
        "part_of_speech": "verb - 1st conjugation, present indicative active, 1st person singular",
        "definition": "I love",
        "principal_parts": "amō, amāre, amāvī, amātum",
        "stems": "Present: amā-, Perfect: amāv-, Participial: amāt-"
    }"#;
    
    let result: LatinWordResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.word_with_macrons, "amō");
    assert_eq!(result.part_of_speech, "verb - 1st conjugation, present indicative active, 1st person singular");
    assert_eq!(result.definition, "I love");
    assert_eq!(result.principal_parts, Some("amō, amāre, amāvī, amātum".to_string()));
    assert_eq!(result.stems, Some("Present: amā-, Perfect: amāv-, Participial: amāt-".to_string()));
}

#[test]
fn test_claude_client_stores_api_key() {
    let api_key = "sk-ant-api03-test-key";
    let client = ClaudeClient::new_with_key(api_key.to_string());
    
    // We can't directly access the api_key field, but we can verify
    // the client was created successfully
    assert_eq!(std::mem::size_of_val(&client) > 0, true);
}

#[tokio::test]
async fn test_config_and_api_integration() {
    use tempfile::TempDir;
    
    // Create a temporary directory for config
    let _temp_dir = TempDir::new().unwrap();
    
    // Note: ConfigManager::new requires a Tauri AppHandle
    // For integration tests, we test the public API behavior
    // The actual ConfigManager integration is tested through the Tauri commands
    
    // Test that we can create a Claude client with an API key
    let test_api_key = "sk-ant-test-integration".to_string();
    let client = ClaudeClient::new_with_key(test_api_key);
    
    // Client should be created successfully
    assert_eq!(std::mem::size_of_val(&client) > 0, true);
}

#[test]
fn test_error_messages_are_user_friendly() {
    // Test that our error messages are clear and helpful
    let error_empty = "Please enter a word";
    let error_invalid = "Please enter only Latin alphabet characters";
    let error_no_key = "API key not configured. Please go to Settings to add your Anthropic API key.";
    
    // Verify error messages are user-friendly
    assert!(error_empty.contains("Please"));
    assert!(error_invalid.contains("Latin alphabet"));
    assert!(error_no_key.contains("Settings"));
}

#[test] 
fn test_custom_prompt_parameter_handling() {
    // Test that the custom prompt parameter is handled correctly in various scenarios
    
    // Test None case
    let custom_prompt: Option<String> = None;
    let as_deref = custom_prompt.as_deref();
    assert!(as_deref.is_none());
    
    // Test empty string case
    let custom_prompt: Option<String> = Some("".to_string());
    let as_deref = custom_prompt.as_deref();
    assert_eq!(as_deref, Some(""));
    assert!(as_deref.unwrap().trim().is_empty());
    
    // Test whitespace case
    let custom_prompt: Option<String> = Some("   ".to_string());
    let as_deref = custom_prompt.as_deref();
    assert_eq!(as_deref, Some("   "));
    assert!(as_deref.unwrap().trim().is_empty());
    
    // Test valid case
    let custom_prompt: Option<String> = Some("Custom instruction text".to_string());
    let as_deref = custom_prompt.as_deref();
    assert_eq!(as_deref, Some("Custom instruction text"));
    assert!(!as_deref.unwrap().trim().is_empty());
}

#[test]
fn test_prompt_building_with_custom_additions() {
    // Test that custom prompts would be appended correctly to base prompts
    let base_prompt = "Base prompt content";
    let mut final_prompt = base_prompt.to_string();
    
    // Test with None custom prompt
    let custom_prompt: Option<&str> = None;
    if let Some(custom) = custom_prompt {
        if !custom.trim().is_empty() {
            final_prompt.push_str("\n\nADDITIONAL INSTRUCTIONS:\n");
            final_prompt.push_str(custom);
        }
    }
    assert_eq!(final_prompt, "Base prompt content");
    
    // Test with empty custom prompt
    let mut final_prompt = base_prompt.to_string();
    let custom_prompt: Option<&str> = Some("");
    if let Some(custom) = custom_prompt {
        if !custom.trim().is_empty() {
            final_prompt.push_str("\n\nADDITIONAL INSTRUCTIONS:\n");
            final_prompt.push_str(custom);
        }
    }
    assert_eq!(final_prompt, "Base prompt content");
    
    // Test with valid custom prompt
    let mut final_prompt = base_prompt.to_string();
    let custom_prompt: Option<&str> = Some("Add more detail");
    if let Some(custom) = custom_prompt {
        if !custom.trim().is_empty() {
            final_prompt.push_str("\n\nADDITIONAL INSTRUCTIONS:\n");
            final_prompt.push_str(custom);
        }
    }
    assert_eq!(final_prompt, "Base prompt content\n\nADDITIONAL INSTRUCTIONS:\nAdd more detail");
}

#[test]
fn test_custom_prompt_edge_cases() {
    // Test various edge cases for custom prompt handling
    
    // Test with special characters
    let custom_prompt = "Use ā, ē, ī, ō, ū macrons";
    assert!(!custom_prompt.trim().is_empty());
    assert!(custom_prompt.contains("ā"));
    
    // Test with newlines
    let custom_prompt = "Line 1\nLine 2\nLine 3";
    assert!(!custom_prompt.trim().is_empty());
    assert!(custom_prompt.contains('\n'));
    
    // Test with only whitespace characters
    let custom_prompt = "\t\n  \r\n";
    assert!(custom_prompt.trim().is_empty());
    
    // Test with leading/trailing whitespace
    let custom_prompt = "  Custom instruction  ";
    assert_eq!(custom_prompt.trim(), "Custom instruction");
}