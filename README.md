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

3. Add this function to your `~/.config/fish/config.fish`:
(source: https://github.com/fish-shell/fish-shell/issues/5938)
```fish
function my_hist --on-event fish_preexec --description "Track fish history in file"
    echo $argv >> ~/.local/share/fish/custom_history
end
```

4. Restart your fish shell or run:
```bash
source ~/.config/fish/config.fish
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

## How It Works

The tool reads from a custom history file (`~/.local/share/fish/custom_history`) that maintains a complete record of all commands. This provides more accurate frequency counting than fish's built-in history.

The process:
1. Each command is logged to the custom history file
2. The tool reads and normalizes each command from the history file
3. Commands are counted and sorted by frequency
4. The most frequent commands are displayed or returned

## Debugging

The tool includes a debug mode that can help diagnose issues with command frequency counting. To enable debug output, use the `-d` or `--debug` flag:

```bash
most-frequent-commands analyze --top 10 --debug
```

The debug output will show:

```
=== DEBUG OUTPUT ===
History file: /home/user/.local/share/fish/custom_history
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

1. **History File Location**: Shows the path of the custom history file being read.

2. **Total Commands Processed**: The total number of commands found in your history.

3. **Unique Commands Found**: The number of distinct commands after normalization.

4. **Top 20 Commands**: Shows the most frequently used commands with their counts.

### Common Issues and Solutions

1. **Missing Commands**: If a command you expect to see is missing:
   - Check if it's in a different format in the history
   - Verify the command appears in your custom history file
   - Try running the command again to ensure it's recorded

2. **Unexpected Counts**: If command counts seem incorrect:
   - Check if the command appears in different formats
   - Verify if the command is being normalized correctly
   - Look for similar commands that might be counted separately

3. **Empty Output**: If no commands are shown:
   - Verify that the custom history file exists and is readable
   - Ensure you have command history in fish
   - Check if there are any permission issues

## Troubleshooting

If the commands are not showing up:

1. Make sure you have some command history in fish shell
2. Check if the custom history file exists at `~/.local/share/fish/custom_history`
3. Try running `most-frequent-commands analyze --top 10` to see if it can read your history 