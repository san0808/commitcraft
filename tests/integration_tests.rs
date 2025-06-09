#[cfg(test)]
mod integration_tests {
    use commitcraft::providers::{openai::OpenAIProvider, gemini::GeminiProvider, anthropic::AnthropicProvider, AIProvider};
    use std::env;

    const TEST_DIFF: &str = r#"diff --git a/src/main.rs b/src/main.rs
index 1234567..abcdefg 100644
--- a/src/main.rs
+++ b/src/main.rs
@@ -1,3 +1,10 @@
 fn main() {
-    println!("Hello, world!");
+    println!("Hello, CommitCraft!");
+    
+    // Add new function to process files
+    process_files();
+}
+
+fn process_files() {
+    println!("Processing files...");
 }"#;

    #[tokio::test]
    #[ignore = "requires OpenAI API key"]
    async fn test_openai_integration() {
        let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set for integration tests");
        let provider = OpenAIProvider::new(api_key, "gpt-4o-mini".to_string());
        
        let result = provider.generate_commit_message(TEST_DIFF).await;
        
        match result {
            Ok(commit) => {
                println!("OpenAI commit title: {}", commit.title);
                println!("OpenAI commit description: {}", commit.description);
                
                // Verify the commit follows conventional commits format
                assert!(commit.title.len() > 0, "Title should not be empty");
                assert!(commit.title.len() <= 50, "Title should be 50 chars or less");
                assert!(commit.title.contains(":"), "Title should contain colon for conventional commits");
                
                // Check that title starts with a type
                let valid_types = ["feat", "fix", "docs", "style", "refactor", "test", "chore"];
                let starts_with_valid_type = valid_types.iter().any(|&t| commit.title.starts_with(t));
                assert!(starts_with_valid_type, "Title should start with a conventional commit type");
                
                assert!(commit.description.len() > 0, "Description should not be empty");
            }
            Err(e) => {
                eprintln!("OpenAI integration test failed: {}", e);
                panic!("OpenAI integration test failed: {}", e);
            }
        }
    }

    #[tokio::test]
    #[ignore = "requires Gemini API key"]
    async fn test_gemini_integration() {
        let api_key = env::var("GOOGLE_AI_API_KEY").expect("GOOGLE_AI_API_KEY must be set for integration tests");
        let provider = GeminiProvider::new(api_key, "gemini-1.5-flash".to_string());
        
        let result = provider.generate_commit_message(TEST_DIFF).await;
        
        match result {
            Ok(commit) => {
                println!("Gemini commit title: {}", commit.title);
                println!("Gemini commit description: {}", commit.description);
                
                // Verify the commit follows conventional commits format
                assert!(commit.title.len() > 0, "Title should not be empty");
                assert!(commit.title.len() <= 50, "Title should be 50 chars or less");
                assert!(commit.title.contains(":"), "Title should contain colon for conventional commits");
                
                // Check that title starts with a type
                let valid_types = ["feat", "fix", "docs", "style", "refactor", "test", "chore"];
                let starts_with_valid_type = valid_types.iter().any(|&t| commit.title.starts_with(t));
                assert!(starts_with_valid_type, "Title should start with a conventional commit type");
                
                assert!(commit.description.len() > 0, "Description should not be empty");
            }
            Err(e) => {
                eprintln!("Gemini integration test failed: {}", e);
                panic!("Gemini integration test failed: {}", e);
            }
        }
    }

    #[tokio::test]
    #[ignore = "requires Anthropic API key"]
    async fn test_anthropic_integration() {
        let api_key = env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY must be set for integration tests");
        let provider = AnthropicProvider::new(api_key, "claude-3-5-haiku-20241022".to_string());
        
        let result = provider.generate_commit_message(TEST_DIFF).await;
        
        match result {
            Ok(commit) => {
                println!("Anthropic commit title: {}", commit.title);
                println!("Anthropic commit description: {}", commit.description);
                
                // Verify the commit follows conventional commits format
                assert!(commit.title.len() > 0, "Title should not be empty");
                assert!(commit.title.len() <= 50, "Title should be 50 chars or less");
                assert!(commit.title.contains(":"), "Title should contain colon for conventional commits");
                
                // Check that title starts with a type
                let valid_types = ["feat", "fix", "docs", "style", "refactor", "test", "chore"];
                let starts_with_valid_type = valid_types.iter().any(|&t| commit.title.starts_with(t));
                assert!(starts_with_valid_type, "Title should start with a conventional commit type");
                
                assert!(commit.description.len() > 0, "Description should not be empty");
            }
            Err(e) => {
                eprintln!("Anthropic integration test failed: {}", e);
                panic!("Anthropic integration test failed: {}", e);
            }
        }
    }

    #[tokio::test]
    #[ignore = "requires all API keys"]
    async fn test_all_providers_consistency() {
        let openai_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
        let gemini_key = env::var("GOOGLE_AI_API_KEY").expect("GOOGLE_AI_API_KEY must be set");
        let anthropic_key = env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY must be set");
        
        let openai = OpenAIProvider::new(openai_key, "gpt-4.1-nano".to_string());
        let gemini = GeminiProvider::new(gemini_key, "gemini-1.5-flash".to_string());
        let anthropic = AnthropicProvider::new(anthropic_key, "claude-3-5-haiku-20241022".to_string());
        
        let providers: Vec<(&str, Box<dyn AIProvider>)> = vec![
            ("OpenAI", Box::new(openai)),
            ("Gemini", Box::new(gemini)), 
            ("Anthropic", Box::new(anthropic)),
        ];

        for (name, provider) in providers {
            println!("\nTesting {} provider...", name);
            let result = provider.generate_commit_message(TEST_DIFF).await;
            
            match result {
                Ok(commit) => {
                    println!("{} - Title: {}", name, commit.title);
                    println!("{} - Description: {}", name, commit.description);
                    
                    // All providers should return valid conventional commits
                    assert!(commit.title.contains(":"), "{} should return conventional commit format", name);
                    assert!(commit.title.len() <= 50, "{} title should be <= 50 chars", name);
                    assert!(!commit.description.is_empty(), "{} description should not be empty", name);
                }
                Err(e) => {
                    eprintln!("{} failed: {}", name, e);
                    panic!("{} provider failed: {}", name, e);
                }
            }
        }
    }
} 