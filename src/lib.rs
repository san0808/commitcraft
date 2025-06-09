pub mod cli;
pub mod config;
pub mod git;
pub mod providers;

// Re-export commonly used types for convenience
pub use providers::{AIProvider, GeneratedCommit}; 