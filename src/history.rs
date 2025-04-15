use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub struct CommandFrequency {
    commands: HashMap<String, usize>,
}

impl CommandFrequency {
    pub fn new(debug: bool) -> Result<Self> {
        let mut commands = HashMap::new();
        let mut total_commands = 0;

        // Get history from custom history file
        let history_file = Self::get_history_file()?;
        
        println!("Reading history file: {:?}", history_file);
        
        if history_file.exists() {
            let content = fs::read_to_string(&history_file)
                .with_context(|| format!("Failed to read history file: {:?}", history_file))?;

            println!("History file content length: {} bytes", content.len());
            
            // Parse each line of history
            for line in content.lines() {
                // Skip empty lines
                if line.trim().is_empty() {
                    continue;
                }
                
                let normalized_cmd = Self::normalize_command(line);
                *commands.entry(normalized_cmd).or_insert(0) += 1;
                total_commands += 1;
            }
        } else {
            println!("History file does not exist");
        }

        if debug {
            println!("\n=== DEBUG OUTPUT ===");
            println!("History file: {:?}", history_file);
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

    fn get_history_file() -> Result<PathBuf> {
        let mut path = dirs::home_dir().context("Failed to get home directory")?;
        path.push(".local/share/fish/custom_history");
        
        // Ensure directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).context("Failed to create history directory")?;
        }
        
        Ok(path)
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
        // Create a test history string
        let test_history = r#"1 ls
2 ls -la
3 cd
4 ls
5 git status
6 ls
7 cd
8 cd
9 ls
10 git status"#;

        // Create a mock CommandFrequency
        let mut commands = HashMap::new();
        for line in test_history.lines() {
            if let Some(cmd) = line.splitn(2, ' ').nth(1) {
                let normalized_cmd = CommandFrequency::normalize_command(cmd);
                *commands.entry(normalized_cmd).or_insert(0) += 1;
            }
        }

        // Debug output for the test
        println!("\n=== TEST DEBUG OUTPUT ===");
        println!("Commands in test history:");
        for (cmd, count) in &commands {
            println!("{}: {}", cmd, count);
        }
        println!("=== END TEST DEBUG ===\n");

        let frequency = CommandFrequency { commands };

        // Test get_most_frequent
        let most_frequent = frequency.get_most_frequent(3);
        assert_eq!(most_frequent.len(), 3);
        
        // Verify the order and counts
        assert_eq!(most_frequent[0].0, "ls");
        assert_eq!(*most_frequent[0].1, 4);  // ls appears 4 times
        
        assert_eq!(most_frequent[1].0, "cd");
        assert_eq!(*most_frequent[1].1, 3);  // cd appears 3 times
        
        assert_eq!(most_frequent[2].0, "git status");
        assert_eq!(*most_frequent[2].1, 2);  // git status appears 2 times
    }
} 