use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;

use super::{AIProvider, GeneratedCommit};

pub struct GeminiProvider {
    client: Client,
    api_key: String,
    model: String,
}

impl GeminiProvider {
    pub fn new(api_key: String, model: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            model,
        }
    }
}

#[derive(Deserialize)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
}

#[derive(Deserialize)]
struct Candidate {
    content: Content,
}

#[derive(Deserialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Deserialize)]
struct Part {
    text: String,
}

#[async_trait]
impl AIProvider for GeminiProvider {
    async fn generate_commit_message(&self, diff: &str) -> Result<GeneratedCommit, String> {
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            self.model, self.api_key
        );

        let system_prompt = "You are an expert programmer who writes git commit messages following the Conventional Commits specification (https://www.conventionalcommits.org/en/v1.0.0/).

IMPORTANT: Respond with ONLY a JSON object in the format {\"title\": \"string\", \"description\": \"string\"}.

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
            "contents": [{
                "parts": [
                    { "text": system_prompt },
                    { "text": "Git diff to analyze:\n```diff\n" },
                    { "text": diff },
                    { "text": "\n```" }
                ]
            }],
            "generationConfig": {
                "response_mime_type": "application/json",
                "temperature": 0.2,
            }
        });

        let response = self.client.post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Gemini API request failed: {}", e))?;
            
        if !response.status().is_success() {
             let error_body = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
             return Err(format!("Gemini API returned an error: {}", error_body));
        }

        let gemini_response: GeminiResponse = response.json().await
            .map_err(|e| format!("Failed to parse Gemini response: {}", e))?;

        let text = gemini_response.candidates
            .get(0)
            .and_then(|c| c.content.parts.get(0))
            .map(|p| &p.text)
            .ok_or("Invalid response structure from Gemini".to_string())?;

        serde_json::from_str(text)
            .map_err(|e| format!("Failed to parse JSON from Gemini response text: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_gemini_provider_new() {
        let provider = GeminiProvider::new("key".to_string(), "model".to_string());
        assert_eq!(provider.api_key, "key");
        assert_eq!(provider.model, "model");
    }

    #[test]
    fn test_gemini_response_deserialize() {
        let json = r#"{
            "candidates": [
                { "content": { "parts": [ { "text": "{\"title\": \"feat: add\", \"description\": \"desc\"}" } ] } }
            ]
        }"#;
        let resp: GeminiResponse = serde_json::from_str(json).unwrap();
        let text = &resp.candidates[0].content.parts[0].text;
        assert!(text.contains("title"));
    }
}
