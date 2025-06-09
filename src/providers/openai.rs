use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestMessage, ChatCompletionRequestSystemMessage,
        ChatCompletionRequestUserMessage, ChatCompletionRequestUserMessageContent,
        ChatCompletionTool, ChatCompletionToolType, CreateChatCompletionRequestArgs,
        FunctionObject, Role,
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

CRITICAL CONSTRAINT: The title field MUST be exactly 50 characters or less. Count every character including spaces, colons, and parentheses.

For the title field:
- Format: <type>[optional scope]: <description>
- Types: feat, fix, docs, style, refactor, test, chore
- MAX 50 characters total - this is NON-NEGOTIABLE
- Use very short, concise descriptions
- Examples (note character counts):
  * \"feat: add file processing\" (26 chars ✓)
  * \"fix: resolve memory leak\" (24 chars ✓) 
  * \"feat(auth): add OAuth\" (20 chars ✓)

For the description field:
- Provide detailed explanation of what changed and why
- Use imperative mood (\"add\" not \"added\")
- Explain the impact and context
- Include breaking changes if any

REMINDER: Title must be ≤50 characters. Prefer shorter, punchy titles over longer descriptive ones.";

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

        let tools = vec![ChatCompletionTool {
            r#type: ChatCompletionToolType::Function,
            function: FunctionObject {
                name: "generate_commit".to_string(),
                description: Some("Generate a conventional commit message".to_string()),
                parameters: Some(parameters_schema),
            },
        }];

        let request = CreateChatCompletionRequestArgs::default()
            .model(&self.model)
            .messages(messages)
            .tools(tools)
            .tool_choice("auto")
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
            .first()
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
