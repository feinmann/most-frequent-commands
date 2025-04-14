use anyhow::{Context, Result};
use directories::BaseDirs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
struct FishHistoryEntry {
    cmd: String,
    when: i64,
}

pub struct CommandFrequency {
    commands: HashMap<String, usize>,
}

impl CommandFrequency {
    pub fn new() -> Result<Self> {
        let mut commands = HashMap::new();
        let history_file = Self::get_history_file()?;

        if history_file.exists() {
            let content = fs::read_to_string(&history_file)
                .with_context(|| format!("Failed to read history file: {:?}", history_file))?;

            for line in content.lines() {
                if let Ok(entry) = serde_json::from_str::<FishHistoryEntry>(line) {
                    *commands.entry(entry.cmd).or_insert(0) += 1;
                }
            }
        }

        Ok(Self { commands })
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