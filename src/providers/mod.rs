use async_trait::async_trait;
use serde::Deserialize;

pub mod anthropic;
pub mod gemini;
pub mod openai;

#[derive(Debug, Deserialize)]
pub struct GeneratedCommit {
    pub title: String,
    pub description: String,
}

impl GeneratedCommit {
    /// Validates if the commit follows conventional commits format
    pub fn validate(&self) -> Result<(), String> {
        // Check title length
        if self.title.len() > 72 {
            return Err("Commit title is too long (max 72 characters)".to_string());
        }

        // Check for conventional commits format
        let conventional_types = [
            "feat", "fix", "docs", "style", "refactor", "test", "chore", "perf", "ci", "build",
            "revert",
        ];

        // Parse the title to check format
        if let Some(colon_pos) = self.title.find(':') {
            let prefix = &self.title[..colon_pos];

            // Check for scope format: type(scope) or just type
            let type_part = if let Some(paren_pos) = prefix.find('(') {
                &prefix[..paren_pos]
            } else {
                prefix
            };

            // Remove trailing ! for breaking changes
            let clean_type = type_part.trim_end_matches('!');

            if !conventional_types.contains(&clean_type) {
                return Err(format!(
                    "Invalid commit type '{}'. Valid types: {}",
                    clean_type,
                    conventional_types.join(", ")
                ));
            }

            // Check description part after colon
            let description = &self.title[colon_pos + 1..].trim();
            if description.is_empty() {
                return Err("Commit description after colon cannot be empty".to_string());
            }

            // Check that description doesn't start with capital letter (conventional commits style)
            if description.chars().next().unwrap_or(' ').is_uppercase() {
                return Err("Commit description should start with lowercase letter".to_string());
            }
        } else {
            return Err("Commit title must follow format: type(scope): description".to_string());
        }

        Ok(())
    }

    /// Gets a summary of the commit for display
    pub fn summary(&self) -> String {
        format!(
            "Type: {}, Files: {}",
            self.get_type(),
            self.description.lines().count()
        )
    }

    /// Extracts the commit type from the title
    pub fn get_type(&self) -> String {
        if let Some(colon_pos) = self.title.find(':') {
            let prefix = &self.title[..colon_pos];
            if let Some(paren_pos) = prefix.find('(') {
                prefix[..paren_pos].trim_end_matches('!').to_string()
            } else {
                prefix.trim_end_matches('!').to_string()
            }
        } else {
            "unknown".to_string()
        }
    }
}

impl ToString for GeneratedCommit {
    fn to_string(&self) -> String {
        format!("{}\n\n{}", self.title, self.description)
    }
}

#[async_trait]
pub trait AIProvider {
    async fn generate_commit_message(&self, diff: &str) -> Result<GeneratedCommit, String>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generated_commit_to_string() {
        let commit = GeneratedCommit {
            title: "feat: add new feature".to_string(),
            description: "This adds a new feature to the project.".to_string(),
        };
        let s = commit.to_string();
        assert!(s.contains("feat: add new feature"));
        assert!(s.contains("This adds a new feature to the project."));
    }
}
