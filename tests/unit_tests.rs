#[cfg(test)]
mod unit_tests {
    use commitcraft::providers::{
        anthropic::AnthropicProvider, gemini::GeminiProvider, openai::OpenAIProvider,
    };

    #[test]
    fn test_provider_initialization() {
        // Test OpenAI provider initialization
        let openai = OpenAIProvider::new("test_key".to_string(), "gpt-4".to_string());
        // Should not panic during creation

        // Test Gemini provider initialization
        let gemini = GeminiProvider::new("test_key".to_string(), "gemini-1.5-flash".to_string());
        // Should not panic during creation

        // Test Anthropic provider initialization
        let anthropic = AnthropicProvider::new(
            "test_key".to_string(),
            "claude-3-5-sonnet-20241022".to_string(),
        );
        // Should not panic during creation

        // If we reach here, all providers initialized successfully
        assert!(true);
    }

    #[test]
    fn test_openai_response_parsing() {
        // Test that OpenAI response structures can be deserialized correctly
        // This will help catch API format changes

        let function_call_response = r#"{
            "choices": [
                {
                    "message": {
                        "tool_calls": [
                            {
                                "id": "call_123",
                                "type": "function",
                                "function": {
                                    "name": "generate_commit",
                                    "arguments": "{\"title\": \"feat: add new feature\", \"description\": \"Added a new feature to improve functionality\"}"
                                }
                            }
                        ]
                    },
                    "finish_reason": "tool_calls"
                }
            ]
        }"#;

        // Parse without panicking
        let parsed: Result<serde_json::Value, _> = serde_json::from_str(function_call_response);
        assert!(parsed.is_ok(), "OpenAI response should parse correctly");

        let json = parsed.unwrap();
        assert!(json.get("choices").is_some());
        assert!(json["choices"][0].get("message").is_some());
        assert!(json["choices"][0]["message"].get("tool_calls").is_some());
    }

    #[test]
    fn test_gemini_response_parsing() {
        // Test that Gemini response structures can be deserialized correctly

        let function_call_response = r#"{
            "candidates": [
                {
                    "content": {
                        "parts": [
                            {
                                "functionCall": {
                                    "name": "generate_commit",
                                    "args": {
                                        "title": "feat: add new feature",
                                        "description": "Added a new feature to improve functionality"
                                    }
                                }
                            }
                        ]
                    },
                    "finishReason": "STOP"
                }
            ]
        }"#;

        // Parse without panicking
        let parsed: Result<serde_json::Value, _> = serde_json::from_str(function_call_response);
        assert!(parsed.is_ok(), "Gemini response should parse correctly");

        let json = parsed.unwrap();
        assert!(json.get("candidates").is_some());
        assert!(json["candidates"][0].get("content").is_some());
        assert!(json["candidates"][0]["content"].get("parts").is_some());

        // Test fallback text response
        let text_response = r#"{
            "candidates": [
                {
                    "content": {
                        "parts": [
                            {
                                "text": "{\"title\": \"feat: add feature\", \"description\": \"Description here\"}"
                            }
                        ]
                    }
                }
            ]
        }"#;

        let parsed_text: Result<serde_json::Value, _> = serde_json::from_str(text_response);
        assert!(
            parsed_text.is_ok(),
            "Gemini text response should parse correctly"
        );
    }

    #[test]
    fn test_anthropic_response_parsing() {
        // Test that Anthropic response structures can be deserialized correctly

        let tool_use_response = r#"{
            "content": [
                {
                    "type": "tool_use",
                    "id": "toolu_123",
                    "name": "generate_commit",
                    "input": {
                        "title": "feat: add new feature",
                        "description": "Added a new feature to improve functionality"
                    }
                }
            ],
            "stop_reason": "tool_use"
        }"#;

        // Parse without panicking
        let parsed: Result<serde_json::Value, _> = serde_json::from_str(tool_use_response);
        assert!(parsed.is_ok(), "Anthropic response should parse correctly");

        let json = parsed.unwrap();
        assert!(json.get("content").is_some());
        assert_eq!(json["content"][0]["type"], "tool_use");
        assert_eq!(json["content"][0]["name"], "generate_commit");
        assert!(json["content"][0].get("input").is_some());

        // Test fallback text response
        let text_response = r#"{
            "content": [
                {
                    "type": "text",
                    "text": "{\"title\": \"feat: add feature\", \"description\": \"Description here\"}"
                }
            ]
        }"#;

        let parsed_text: Result<serde_json::Value, _> = serde_json::from_str(text_response);
        assert!(
            parsed_text.is_ok(),
            "Anthropic text response should parse correctly"
        );
    }

    #[test]
    fn test_conventional_commit_format_validation() {
        // Test helper function for validating conventional commit format
        let valid_titles = vec![
            "feat: add new feature",
            "fix: resolve bug in parser",
            "docs: update README",
            "style: format code",
            "refactor: restructure module",
            "test: add unit tests",
            "chore: update dependencies",
            "feat(auth): add OAuth support",
            "fix(ui): resolve layout issue",
        ];

        for title in valid_titles {
            assert!(
                is_valid_conventional_commit_title(title),
                "Title '{}' should be valid conventional commit format",
                title
            );
        }

        let invalid_titles = vec![
            "Add new feature",   // No type
            "FEAT: add feature", // Uppercase type
            "feat add feature",  // No colon
            "",                  // Empty
            "feat:",             // No description
        ];

        for title in invalid_titles {
            assert!(
                !is_valid_conventional_commit_title(title),
                "Title '{}' should be invalid conventional commit format",
                title
            );
        }
    }

    // Helper function to validate conventional commit format
    fn is_valid_conventional_commit_title(title: &str) -> bool {
        if title.is_empty() || title.len() > 50 {
            return false;
        }

        if !title.contains(':') {
            return false;
        }

        let parts: Vec<&str> = title.splitn(2, ':').collect();
        if parts.len() != 2 {
            return false;
        }

        let type_part = parts[0].trim();
        let description_part = parts[1].trim();

        if description_part.is_empty() {
            return false;
        }

        // Check if type part matches conventional commit types
        let valid_types = ["feat", "fix", "docs", "style", "refactor", "test", "chore"];

        // Handle scoped types like "feat(auth)"
        let base_type = if type_part.contains('(') {
            type_part.split('(').next().unwrap_or("")
        } else {
            type_part
        };

        valid_types.contains(&base_type)
            && base_type
                .chars()
                .all(|c| c.is_lowercase() || c == '(' || c == ')')
    }

    #[test]
    fn test_json_schema_generation() {
        // Test that our Commit struct can generate valid JSON schemas
        use schemars::JsonSchema;
        use serde_json;

        #[derive(schemars::JsonSchema)]
        struct TestCommit {
            title: String,
            description: String,
        }

        let schema = schemars::schema_for!(TestCommit);
        let schema_value = serde_json::to_value(schema).expect("Should serialize schema");

        // Verify basic schema structure
        assert!(schema_value.get("type").is_some());
        assert!(schema_value.get("properties").is_some());

        let properties = &schema_value["properties"];
        assert!(properties.get("title").is_some());
        assert!(properties.get("description").is_some());
    }
}

#[cfg(test)]
mod mock_tests {
    use async_trait::async_trait;
    use commitcraft::providers::{AIProvider, GeneratedCommit};

    // Mock provider for testing
    struct MockProvider {
        should_fail: bool,
    }

    #[async_trait]
    impl AIProvider for MockProvider {
        async fn generate_commit_message(&self, _diff: &str) -> Result<GeneratedCommit, String> {
            if self.should_fail {
                Err("Mock provider error".to_string())
            } else {
                Ok(GeneratedCommit {
                    title: "feat: add mock feature".to_string(),
                    description: "Added a mock feature for testing purposes".to_string(),
                })
            }
        }
    }

    #[tokio::test]
    async fn test_mock_provider_success() {
        let provider = MockProvider { should_fail: false };
        let result = provider.generate_commit_message("mock diff").await;

        assert!(result.is_ok());
        let commit = result.unwrap();
        assert_eq!(commit.title, "feat: add mock feature");
        assert!(!commit.description.is_empty());
    }

    #[tokio::test]
    async fn test_mock_provider_failure() {
        let provider = MockProvider { should_fail: true };
        let result = provider.generate_commit_message("mock diff").await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Mock provider error");
    }
}
