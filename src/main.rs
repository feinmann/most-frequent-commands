use clap::{Parser, Subcommand};
use anyhow::Result;
mod history;

/// A command-line tool to analyze and display frequently used commands
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Analyze command history and show most frequent commands
    Analyze {
        /// Number of top commands to show
        #[arg(short, long, default_value_t = 10)]
        top: usize,
        
        /// Enable debug output
        #[arg(short, long)]
        debug: bool,
    },
    
    /// Get a specific command by its index in the frequency list
    Get {
        /// Index of the command to get (0-based)
        #[arg(short, long)]
        index: usize,
        
        /// Enable debug output
        #[arg(short, long)]
        debug: bool,
    },
}

fn main() -> Result<()> {
    let args = Cli::parse();

    match args.command {
        Commands::Analyze { top, debug } => {
            let frequency = history::CommandFrequency::new(debug)?;
            let most_frequent = frequency.get_most_frequent(top);
            
            println!("Top {} most frequent commands:", top);
            for (i, (cmd, count)) in most_frequent.iter().enumerate() {
                println!("{}. {} (used {} times)", i + 1, cmd, count);
            }
        }
        Commands::Get { index, debug } => {
            let frequency = history::CommandFrequency::new(debug)?;
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
