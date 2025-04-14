use anyhow::{Context, Result};
use directories::BaseDirs;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub struct CommandFrequency {
    commands: HashMap<String, usize>,
}

impl CommandFrequency {
    pub fn new(debug: bool) -> Result<Self> {
        let mut commands = HashMap::new();
        let history_file = Self::get_history_file()?;

        if history_file.exists() {
            let content = fs::read_to_string(&history_file)
                .with_context(|| format!("Failed to read history file: {:?}", history_file))?;

            let mut current_cmd: Option<String> = None;
            let mut total_commands = 0;
            
            for line in content.lines() {
                if line.starts_with("- cmd: ") {
                    if let Some(cmd) = current_cmd {
                        let normalized_cmd = Self::normalize_command(&cmd);
                        *commands.entry(normalized_cmd).or_insert(0) += 1;
                        total_commands += 1;
                    }
                    current_cmd = Some(line[7..].to_string());
                }
            }
            
            // Don't forget the last command
            if let Some(cmd) = current_cmd {
                let normalized_cmd = Self::normalize_command(&cmd);
                *commands.entry(normalized_cmd).or_insert(0) += 1;
                total_commands += 1;
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
        let base_dirs = BaseDirs::new().context("Failed to get base directories")?;
        let mut path = PathBuf::from(base_dirs.home_dir());
        path.push(".local/share/fish/fish_history");
        Ok(path)
    }

    pub fn get_most_frequent(&self, count: usize) -> Vec<(&String, &usize)> {
        let mut entries: Vec<_> = self.commands.iter().collect();
        entries.sort_by(|a, b| b.1.cmp(a.1));
        entries.into_iter().take(count).collect()
    }
} 