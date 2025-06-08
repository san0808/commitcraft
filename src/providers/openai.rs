use async_openai::{
    config::OpenAIConfig,
    types::{
        // Remove deprecated types
        // ChatCompletionFunctions, ChatCompletionFunctionCall,
        // Add new types
        ChatCompletionRequestMessage, // This is now an enum
        ChatCompletionRequestSystemMessage,
        ChatCompletionRequestUserMessage,
        ChatCompletionRequestUserMessageContent,
        CreateChatCompletionRequestArgs,
        Role, // Role might not be directly used for message construction anymore
    },
    Client,
};
use async_trait::async_trait;
use schemars::JsonSchema;

use super::{AIProvider, GeneratedCommit};

#[derive(Debug, serde::Deserialize, JsonSchema)]
struct Commit {
    /// The title of the commit message (max 50 chars).
    title: String,
    /// A detailed, exhaustive description of the changes.
    description: String,
}

pub struct OpenAIProvider {
    client: Client<OpenAIConfig>,
    model: String,
}

impl OpenAIProvider {
    pub fn new(api_key: String, model: String) -> Self {
        let config = OpenAIConfig::new().with_api_key(api_key);
        Self {
            client: Client::with_config(config),
            model,
        }
    }
}

#[async_trait]
impl AIProvider for OpenAIProvider {
    async fn generate_commit_message(&self, diff: &str) -> Result<GeneratedCommit, String> {
        let parameters_schema = serde_json::to_value(schemars::schema_for!(Commit))
            .map_err(|e| format!("Failed to create schema: {}", e))?;

        let system_prompt = "You are an expert programmer who writes git commit messages following the Conventional Commits specification (https://www.conventionalcommits.org/en/v1.0.0/).

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

        let messages = vec![
            ChatCompletionRequestMessage::System(ChatCompletionRequestSystemMessage {
                role: Role::System,
                content: system_prompt.to_string(),
                name: None,
            }),
            ChatCompletionRequestMessage::User(ChatCompletionRequestUserMessage {
                role: Role::User,
                content: ChatCompletionRequestUserMessageContent::Text(format!(
                    "Here is the git diff:\n\n```diff\n{}\n```",
                    diff
                )),
                name: None,
            }),
        ];

        let request = CreateChatCompletionRequestArgs::default()
            .model(&self.model)
            .messages(messages)
            .temperature(0.2)
            .build()
            .map_err(|e| format!("Failed to build OpenAI request: {}", e))?;

        let response = self
            .client
            .chat()
            .create(request)
            .await
            .map_err(|e| format!("OpenAI API call failed: {}", e))?;

        let choice = response
            .choices
            .get(0)
            .ok_or("No response choice from OpenAI".to_string())?;

        let function_details = &choice
            .message
            .tool_calls
            .as_ref()
            .ok_or("Expected tool calls from OpenAI".to_string())?[0]
            .function;

        let commit: Commit = serde_json::from_str(&function_details.arguments).map_err(|e| {
            format!(
                "Failed to parse OpenAI tool call arguments: {}\nArguments: {}",
                e, function_details.arguments
            )
        })?;

        Ok(GeneratedCommit {
            title: commit.title,
            description: commit.description,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_openai_provider_new() {
        let provider = OpenAIProvider::new("key".to_string(), "model".to_string());
        assert_eq!(provider.model, "model");
    }
}
