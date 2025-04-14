# Most Frequent Commands

A tool to display and use your most frequently used shell commands in fish shell. (Created with the help of cursor)

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

### Shell Integration #TODO not working

The tool integrates with fish shell to provide quick access to your most frequent commands:

- `Ctrl+Shift+↑`: Shows your most frequent command
- `Ctrl+Shift+→`: Shows your second most frequent command
- `Ctrl+Shift+←`: Shows your third most frequent command

## Debugging

The tool includes a debug mode that can help diagnose issues with command frequency counting. To enable debug output, use the `-d` or `--debug` flag:

```bash
most-frequent-commands analyze --top 10 --debug
```

The debug output will show:

```
=== DEBUG OUTPUT ===
History file: /home/user/.local/share/fish/fish_history
Total commands processed: 1234
Unique commands found: 567

Top 20 commands:
1. ls (45 times)
2. cd (32 times)
3. git status (28 times)
...
=== END DEBUG ===
```

### Interpreting Debug Output

1. **History File Location**: Shows the exact path of the fish history file being read. Verify this matches your system's configuration.

2. **Total Commands Processed**: The total number of commands found in your history file. If this number seems low, it might indicate:
   - The history file is not being read correctly
   - The history file is empty or truncated
   - The file format is not being parsed correctly

3. **Unique Commands Found**: The number of distinct commands after normalization. If this is much lower than total commands, it means many commands are duplicates.

4. **Top 20 Commands**: Shows the most frequently used commands with their counts. This can help identify:
   - If command normalization is working correctly
   - If certain commands are being counted multiple times
   - If expected commands are missing or have unexpected counts

### Common Issues and Solutions

1. **Missing Commands**: If a command you expect to see is missing:
   - Check if it's in a different format in the history (e.g., with quotes or different spacing)
   - Verify the command appears in your fish history
   - Try running the command again to ensure it's recorded

2. **Unexpected Counts**: If command counts seem incorrect:
   - Check if the command appears in different formats in the history
   - Verify if the command is being normalized correctly
   - Look for similar commands that might be counted separately

3. **Empty Output**: If no commands are shown:
   - Verify the history file exists and is readable
   - Check if the file format matches the expected YAML format
   - Ensure you have command history in fish

## How It Works

The tool reads your fish shell history file (`~/.local/share/fish/fish_history`), counts the frequency of each command, and allows you to quickly access them through keyboard shortcuts.

## Troubleshooting

If the commands are not showing up:

1. Make sure you have some command history in fish shell
2. Check if the history file exists at `~/.local/share/fish/fish_history`
3. Try running `most-frequent-commands analyze --top 10` to see if it can read your history 