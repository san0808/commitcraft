use async_trait::async_trait;
use reqwest::Client;
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::json;

use super::{AIProvider, GeneratedCommit};

#[derive(Debug, serde::Deserialize, JsonSchema)]
struct Commit {
    /// The title of the commit message (max 50 chars).
    title: String,
    /// A detailed, exhaustive description of the changes.
    description: String,
}

pub struct AnthropicProvider {
    client: Client,
    api_key: String,
    model: String,
}

impl AnthropicProvider {
    pub fn new(api_key: String, model: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            model,
        }
    }
}

#[derive(Deserialize)]
struct AnthropicResponse {
    content: Vec<ContentBlock>,
    #[allow(dead_code)]
    stop_reason: Option<String>,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
enum ContentBlock {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "tool_use")]
    ToolUse {
        #[allow(dead_code)]
        id: String,
        name: String,
        input: serde_json::Value,
    },
}

#[async_trait]
impl AIProvider for AnthropicProvider {
    async fn generate_commit_message(&self, diff: &str) -> Result<GeneratedCommit, String> {
        let url = "https://api.anthropic.com/v1/messages";

        let system_prompt = "You are an expert programmer who writes git commit messages following the Conventional Commits specification (https://www.conventionalcommits.org/en/v1.0.0/).

For the title field:
- MUST follow this exact format: <type>[optional scope]: <description>
- Common types: feat (new feature), fix (bug fix), docs (documentation), style (formatting), refactor (code restructuring), test (adding tests), chore (maintenance)
- CRITICAL: Keep title under 50 characters total (including type and colon)
- Use lowercase for type
- Be specific but concise
- Examples: \"feat(auth): add OAuth2 login\", \"fix: resolve memory leak\"

For the description field:
- Provide detailed explanation of what changed and why
- Use imperative mood (\"add\" not \"added\")
- Explain the impact and context
- Include breaking changes if any

Analyze the git diff carefully and generate an appropriate conventional commit message using the generate_commit tool. The title MUST be 50 characters or less.";

        // Create the tool schema
        let parameters_schema = serde_json::to_value(schemars::schema_for!(Commit))
            .map_err(|e| format!("Failed to create schema: {}", e))?;

        let body = json!({
            "model": self.model,
            "max_tokens": 1024,
            "temperature": 0.2,
            "system": system_prompt,
            "messages": [
                {
                    "role": "user",
                    "content": format!("Here is the git diff to analyze:\n```diff\n{}\n```", diff)
                }
            ],
            "tools": [
                {
                    "name": "generate_commit",
                    "description": "Generate a conventional commit message with title and description",
                    "input_schema": parameters_schema
                }
            ],
            "tool_choice": {
                "type": "tool",
                "name": "generate_commit"
            }
        });

        let response = self
            .client
            .post(url)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Anthropic API request failed: {}", e))?;

        if !response.status().is_success() {
            let error_body = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(format!("Anthropic API returned an error: {}", error_body));
        }

        let anthropic_response: AnthropicResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse Anthropic response: {}", e))?;

        // Look for tool use in the content blocks
        for content_block in &anthropic_response.content {
            if let ContentBlock::ToolUse { name, input, .. } = content_block {
                if name == "generate_commit" {
                    let commit: Commit = serde_json::from_value(input.clone())
                        .map_err(|e| format!("Failed to parse tool input: {}", e))?;

                    return Ok(GeneratedCommit {
                        title: commit.title,
                        description: commit.description,
                    });
                }
            }
        }

        // Fallback: if no tool use found, check for text response
        for content_block in &anthropic_response.content {
            if let ContentBlock::Text { text } = content_block {
                // Try to parse as JSON in case Anthropic returns JSON without tool calling
                if let Ok(commit) = serde_json::from_str::<Commit>(text) {
                    return Ok(GeneratedCommit {
                        title: commit.title,
                        description: commit.description,
                    });
                }
            }
        }

        Err("No valid tool use or parseable JSON found in Anthropic response".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_anthropic_provider_new() {
        let provider = AnthropicProvider::new("key".to_string(), "model".to_string());
        assert_eq!(provider.api_key, "key");
        assert_eq!(provider.model, "model");
    }

    #[test]
    fn test_anthropic_response_deserialize() {
        let json = r#"{
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
        let resp: AnthropicResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.content.len(), 1);

        // Test tool use parsing
        if let ContentBlock::ToolUse { name, input, .. } = &resp.content[0] {
            assert_eq!(name, "generate_commit");
            assert!(input.get("title").is_some());
        } else {
            panic!("Expected tool use content block");
        }
    }

    #[test]
    fn test_anthropic_text_response_deserialize() {
        let json = r#"{
            "content": [
                {
                    "type": "text",
                    "text": "{\"title\": \"feat: add\", \"description\": \"desc\"}"
                }
            ]
        }"#;
        let resp: AnthropicResponse = serde_json::from_str(json).unwrap();
        if let ContentBlock::Text { text } = &resp.content[0] {
            assert!(text.contains("title"));
        } else {
            panic!("Expected text content block");
        }
    }
}
