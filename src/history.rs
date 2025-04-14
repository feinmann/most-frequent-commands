use anyhow::{Context, Result};
use directories::BaseDirs;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub struct CommandFrequency {
    commands: HashMap<String, usize>,
}

impl CommandFrequency {
    pub fn new(debug: bool) -> Result<Self> {
        let mut commands = HashMap::new();
        let mut total_commands = 0;

        // Get history from fish command
        let output = Command::new("fish")
            .arg("-c")
            .arg("history")
            .output()
            .context("Failed to execute fish history command")?;

        if !output.status.success() {
            return Err(anyhow::anyhow!(
                "Fish history command failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        let history = String::from_utf8(output.stdout)
            .context("Failed to parse fish history output")?;

        // Parse each line of history
        for line in history.lines() {
            if let Some(cmd) = line.splitn(2, ' ').nth(1) {
                let normalized_cmd = Self::normalize_command(cmd);
                *commands.entry(normalized_cmd).or_insert(0) += 1;
                total_commands += 1;
            }
        }

        if debug {
            println!("\n=== DEBUG OUTPUT ===");
            println!("Total commands processed: {}", total_commands);
            println!("Unique commands found: {}", commands.len());
            
            // Print top 20 commands for debugging
            let mut debug_entries: Vec<_> = commands.iter().collect();
            debug_entries.sort_by(|a, b| b.1.cmp(a.1));
            println!("\nTop 20 commands:");
            for (i, (cmd, count)) in debug_entries.iter().take(20).enumerate() {
                println!("{}. {} ({} times)", i + 1, cmd, count);
            }
            println!("=== END DEBUG ===\n");
        }

        Ok(Self { commands })
    }

    fn normalize_command(cmd: &str) -> String {
        // Remove leading/trailing whitespace
        let cmd = cmd.trim();
        // Remove quotes if present
        let cmd = cmd.trim_matches(|c| c == '"' || c == '\'');
        // Normalize multiple spaces to single space
        let cmd = cmd.split_whitespace().collect::<Vec<_>>().join(" ");
        cmd.to_string()
    }

    pub fn get_most_frequent(&self, count: usize) -> Vec<(&String, &usize)> {
        let mut entries: Vec<_> = self.commands.iter().collect();
        entries.sort_by(|a, b| b.1.cmp(a.1));
        entries.into_iter().take(count).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_command() {
        assert_eq!(CommandFrequency::normalize_command("  ls  "), "ls");
        assert_eq!(CommandFrequency::normalize_command("ls -la"), "ls -la");
        assert_eq!(CommandFrequency::normalize_command("  ls   -la  "), "ls -la");
        assert_eq!(CommandFrequency::normalize_command("\"ls -la\""), "ls -la");
        assert_eq!(CommandFrequency::normalize_command("'ls -la'"), "ls -la");
    }

    #[test]
    fn test_command_frequency() {
        // Create a test history string with more ls commands to ensure it's the most frequent
        let test_history = r#"1 ls
2 ls -la
3 cd
4 ls
5 git status
6 ls
7 cd
8 cd
9 ls
10 ls"#;

        // Create a mock CommandFrequency
        let mut commands = HashMap::new();
        for line in test_history.lines() {
            if let Some(cmd) = line.splitn(2, ' ').nth(1) {
                let normalized_cmd = CommandFrequency::normalize_command(cmd);
                *commands.entry(normalized_cmd).or_insert(0) += 1;
            }
        }
        let frequency = CommandFrequency { commands };

        // Test get_most_frequent
        let most_frequent = frequency.get_most_frequent(3);
        assert_eq!(most_frequent.len(), 3);
        
        // Verify the order and counts
        assert_eq!(most_frequent[0].0, "ls");
        assert_eq!(*most_frequent[0].1, 5);  // ls appears 5 times
        
        assert_eq!(most_frequent[1].0, "cd");
        assert_eq!(*most_frequent[1].1, 2);  // cd appears 2 times
        
        assert_eq!(most_frequent[2].0, "ls -la");
        assert_eq!(*most_frequent[2].1, 1);  // ls -la appears 1 time
    }
} 