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
    #[serde(rename = "finishReason")]
    #[allow(dead_code)]
    finish_reason: Option<String>,
}

#[derive(Deserialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum Part {
    Text { text: String },
}

#[async_trait]
impl AIProvider for GeminiProvider {
    async fn generate_commit_message(&self, diff: &str) -> Result<GeneratedCommit, String> {
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            self.model, self.api_key
        );

        let system_instruction = "You are an expert programmer who writes git commit messages following the Conventional Commits specification (https://www.conventionalcommits.org/en/v1.0.0/).

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

Analyze the git diff carefully and respond with a JSON object containing the title and description fields. The title MUST be 50 characters or less.";

        // Create the response schema using the new structured output approach
        let mut response_schema = serde_json::to_value(schemars::schema_for!(Commit))
            .map_err(|e| format!("Failed to create schema: {}", e))?;

        // Remove $schema and other metadata that Gemini doesn't accept
        if let Some(obj) = response_schema.as_object_mut() {
            obj.remove("$schema");
            obj.remove("title");
        }

        let body = json!({
            "system_instruction": {
                "parts": [
                    { "text": system_instruction }
                ]
            },
            "contents": [{
                "parts": [
                    { "text": format!("Here is the git diff to analyze:\n```diff\n{}\n```", diff) }
                ]
            }],
            "generation_config": {
                "temperature": 0.2,
                "candidate_count": 1,
                "response_mime_type": "application/json",
                "response_schema": response_schema
            }
        });

        let response = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Gemini API request failed: {}", e))?;

        if !response.status().is_success() {
            let error_body = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(format!("Gemini API returned an error: {}", error_body));
        }

        let gemini_response: GeminiResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse Gemini response: {}", e))?;

        let candidate = gemini_response
            .candidates
            .first()
            .ok_or("No candidates in Gemini response".to_string())?;

        // With structured output, Gemini returns JSON directly in text parts
        for part in &candidate.content.parts {
            let Part::Text { text } = part;
            let commit: Commit = serde_json::from_str(text)
                .map_err(|e| format!("Failed to parse structured JSON response: {}", e))?;

            return Ok(GeneratedCommit {
                title: commit.title,
                description: commit.description,
            });
        }

        Err("No text content found in Gemini response".to_string())
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
                { 
                    "content": { 
                        "parts": [ 
                            { 
                                "text": "{\"title\": \"feat: add new feature\", \"description\": \"Added a new feature to improve functionality\"}"
                            } 
                        ] 
                    },
                    "finishReason": "STOP"
                }
            ]
        }"#;
        let resp: GeminiResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.candidates.len(), 1);

        // Test structured output parsing
        if let Part::Text { text } = &resp.candidates[0].content.parts[0] {
            let commit: Commit = serde_json::from_str(text).unwrap();
            assert_eq!(commit.title, "feat: add new feature");
            assert_eq!(
                commit.description,
                "Added a new feature to improve functionality"
            );
        } else {
            panic!("Expected text part");
        }
    }

    #[test]
    fn test_gemini_text_response_deserialize() {
        let json = r#"{
            "candidates": [
                { 
                    "content": { 
                        "parts": [ 
                            { "text": "{\"title\": \"feat: add\", \"description\": \"desc\"}" } 
                        ] 
                    } 
                }
            ]
        }"#;
        let resp: GeminiResponse = serde_json::from_str(json).unwrap();
        if let Part::Text { text } = &resp.candidates[0].content.parts[0] {
            assert!(text.contains("title"));
        } else {
            panic!("Expected text part");
        }
    }
}
