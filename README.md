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

## How It Works

The tool uses fish's built-in `history` command to get a complete list of all commands, including adjacent duplicates. This provides more accurate frequency counting than reading the history file directly, as the history file (`~/.local/share/fish/fish_history`) eliminates adjacent identical commands.

The process:
1. Executes `fish -c "history"` to get the complete command history
2. Normalizes each command (removes extra spaces, quotes, etc.)
3. Counts the frequency of each normalized command
4. Sorts commands by frequency

## Debugging

The tool includes a debug mode that can help diagnose issues with command frequency counting. To enable debug output, use the `-d` or `--debug` flag:

```bash
most-frequent-commands analyze --top 10 --debug
```

The debug output will show:

```
=== DEBUG OUTPUT ===
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

1. **Total Commands Processed**: The total number of commands found in your history. This includes all commands, even adjacent duplicates.

2. **Unique Commands Found**: The number of distinct commands after normalization. If this is much lower than total commands, it means many commands are duplicates.

3. **Top 20 Commands**: Shows the most frequently used commands with their counts. This can help identify:
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
   - Verify that `fish -c "history"` works in your shell
   - Ensure you have command history in fish
   - Check if there are any permission issues

## Troubleshooting

If the commands are not showing up:

1. Make sure you have some command history in fish shell
2. Try running `fish -c "history"` to verify you can access the history
3. Try running `most-frequent-commands analyze --top 10` to see if it can read your history 