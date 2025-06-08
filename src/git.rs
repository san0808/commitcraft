use colored::*;
use std::process::{Command, Stdio};
use std::str;

/// Checks if the current directory is a Git repository.
pub fn is_git_repository() -> bool {
    Command::new("git")
        .arg("rev-parse")
        .arg("--is-inside-work-tree")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map_or(false, |status| status.success())
}

/// Gets the diff of staged files.
pub fn get_staged_diff() -> Result<String, String> {
    let output = Command::new("git")
        .arg("diff")
        .arg("--staged")
        .output()
        .map_err(|e| format!("Failed to execute 'git diff --staged': {}", e))?;

    if !output.status.success() {
        let stderr = str::from_utf8(&output.stderr).unwrap_or("Unknown git error");
        return Err(format!("'git diff --staged' failed: {}", stderr));
    }

    let diff = str::from_utf8(&output.stdout)
        .map_err(|e| format!("Failed to parse git diff output: {}", e))?
        .trim();

    if diff.is_empty() {
        return Err("There are no staged files to commit. Try running 'git add'.".to_string());
    }

    Ok(diff.to_string())
}

/// Gets the names of staged files.
pub fn get_staged_files() -> Result<Vec<String>, String> {
    let output = Command::new("git")
        .arg("diff")
        .arg("--staged")
        .arg("--name-only")
        .output()
        .map_err(|e| format!("Failed to execute 'git diff --staged --name-only': {}", e))?;

    if !output.status.success() {
        let stderr = str::from_utf8(&output.stderr).unwrap_or("Unknown git error");
        return Err(format!(
            "'git diff --staged --name-only' failed: {}",
            stderr
        ));
    }

    let files = str::from_utf8(&output.stdout)
        .map_err(|e| format!("Failed to parse git diff output: {}", e))?
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.to_string())
        .collect();

    Ok(files)
}

/// Gets basic repository information.
pub fn get_repo_info() -> Result<(String, String), String> {
    // Get current branch
    let branch_output = Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .output()
        .map_err(|e| format!("Failed to get current branch: {}", e))?;

    let branch = str::from_utf8(&branch_output.stdout)
        .map_err(|e| format!("Failed to parse branch name: {}", e))?
        .trim()
        .to_string();

    // Get repository root directory name
    let root_output = Command::new("git")
        .arg("rev-parse")
        .arg("--show-toplevel")
        .output()
        .map_err(|e| format!("Failed to get repository root: {}", e))?;

    let root_path = str::from_utf8(&root_output.stdout)
        .map_err(|e| format!("Failed to parse repository root: {}", e))?
        .trim();

    let repo_name = std::path::Path::new(root_path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("unknown")
        .to_string();

    Ok((repo_name, branch))
}

/// Commits the generated message.
pub fn commit(message: &str, review: bool) -> Result<(), String> {
    let mut command = Command::new("git");
    command.arg("commit");

    if review {
        command.arg("-e"); // Open editor
    }

    let mut child = command
        .arg("-F")
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn git commit process: {}", e))?;

    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    let message_clone = message.to_string();
    std::thread::spawn(move || {
        use std::io::Write;
        stdin
            .write_all(message_clone.as_bytes())
            .expect("Failed to write to stdin");
    });

    let output = child
        .wait_with_output()
        .map_err(|e| format!("Failed to wait for git commit process: {}", e))?;

    if output.status.success() {
        println!("{}", "Commit successful!".green());
        println!("{}", String::from_utf8_lossy(&output.stdout));
        Ok(())
    } else {
        Err(format!(
            "Git commit failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_git_repository_false() {
        // Should return false if not in a git repo (simulate by running in a temp dir)
        // This test is limited since it depends on the environment, but we can at least check it doesn't panic
        let _ = is_git_repository();
    }

    #[test]
    fn test_get_staged_diff_no_git() {
        // This will likely fail if not in a git repo, but should return an error string
        let result = get_staged_diff();
        assert!(result.is_err(), "Expected error, got: {:?}", result);
    }

    #[test]
    fn test_commit_error() {
        // Should error if git is not available or not in a repo
        let result = commit("test message", false);
        assert!(result.is_err());
    }
}
