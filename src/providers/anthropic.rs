use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;

use super::{AIProvider, GeneratedCommit};

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
}

#[derive(Deserialize)]
struct ContentBlock {
    text: String,
}

#[async_trait]
impl AIProvider for AnthropicProvider {
    async fn generate_commit_message(&self, diff: &str) -> Result<GeneratedCommit, String> {
        let url = "https://api.anthropic.com/v1/messages";

        let system_prompt = "You are an expert programmer who writes git commit messages following the Conventional Commits specification (https://www.conventionalcommits.org/en/v1.0.0/).

IMPORTANT: Your response MUST be ONLY a single, valid JSON object in the format {\"title\": \"string\", \"description\": \"string\"}. Do not include any other text, explanations, or markdown formatting.

For the title field:
- MUST follow this exact format: <type>[optional scope]: <description>
- Common types: feat (new feature), fix (bug fix), docs (documentation), style (formatting), refactor (code restructuring), test (adding tests), chore (maintenance)
- Keep under 50 characters
- Use lowercase for type
- Be specific and actionable
- Examples: \"feat(auth): add OAuth2 login support\", \"fix: resolve memory leak in parser\"

For the description field:
- Provide detailed explanation of what changed and why
- Use imperative mood (\"add\" not \"added\")
- Explain the impact and context
- Include breaking changes if any

Analyze the git diff carefully and generate an appropriate conventional commit message.";

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
            ]
        });

        let response = self.client.post(url)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Anthropic API request failed: {}", e))?;

        if !response.status().is_success() {
             let error_body = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
             return Err(format!("Anthropic API returned an error: {}", error_body));
        }

        let anthropic_response: AnthropicResponse = response.json().await
            .map_err(|e| format!("Failed to parse Anthropic response: {}", e))?;

        let text = anthropic_response.content.get(0)
            .map(|c| &c.text)
            .ok_or("Invalid response structure from Anthropic".to_string())?;

        serde_json::from_str(text)
            .map_err(|e| format!("Failed to parse JSON from Anthropic response text: {}. Raw text: '{}'", e, text))
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
                { "text": "{\"title\": \"feat: add\", \"description\": \"desc\"}" }
            ]
        }"#;
        let resp: AnthropicResponse = serde_json::from_str(json).unwrap();
        let text = &resp.content[0].text;
        assert!(text.contains("title"));
    }
}
