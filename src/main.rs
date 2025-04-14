use clap::Parser;
use anyhow::Result;
mod history;

/// A command-line tool to analyze and display frequently used commands
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
enum Args {
    /// Analyze and display most frequent commands
    Analyze {
        /// Number of top commands to show
        #[arg(short, long, default_value_t = 10)]
        top: usize,
    },
    /// Get the nth most frequent command
    Get {
        /// Index of the command to get (0-based)
        #[arg(short, long, default_value_t = 0)]
        index: usize,
    },
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args {
        Args::Analyze { top } => {
            let frequency = history::CommandFrequency::new()?;
            let most_frequent = frequency.get_most_frequent(top);
            
            println!("Top {} most frequent commands:", top);
            for (i, (cmd, count)) in most_frequent.iter().enumerate() {
                println!("{}. {} (used {} times)", i + 1, cmd, count);
            }
        }
        Args::Get { index } => {
            let frequency = history::CommandFrequency::new()?;
            let most_frequent = frequency.get_most_frequent(index + 1);
            
            if let Some((cmd, _)) = most_frequent.get(index) {
                println!("{}", cmd);
            } else {
                println!("No command found at index {}", index);
            }
        }
    }

    Ok(())
}
