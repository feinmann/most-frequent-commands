# Most Frequent Commands

A tool to display and use your most frequently used shell commands in fish shell.

## Installation

1. Build the Rust program:
```bash
cargo build --release
```

2. Install the binary:
```bash
cp target/release/most-frequent-commands ~/.local/bin/
```

3. Install the fish shell function:
```bash
mkdir -p ~/.config/fish/functions
cp fish_functions/most-frequent-commands.fish ~/.config/fish/functions/
```

4. Restart your fish shell or run:
```bash
source ~/.config/fish/functions/most-frequent-commands.fish
```

## Usage

### Command Line

- To see your most frequent commands:
```bash
most-frequent-commands analyze --top 10
```

- To get a specific command by index:
```bash
most-frequent-commands get --index 0
```

### Shell Integration

The tool integrates with fish shell to provide quick access to your most frequent commands:

- `↑` (Up Arrow): Shows your most frequent command
- `Ctrl+↑`: Shows your most frequent command
- `Shift+↑`: Shows your second most frequent command
- `Alt+↑`: Shows your third most frequent command

## How It Works

The tool reads your fish shell history file (`~/.local/share/fish/fish_history`), counts the frequency of each command, and allows you to quickly access them through keyboard shortcuts. 