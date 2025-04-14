function __most_frequent_command
    set -l index $argv[1]
    set -l cmd (most-frequent-commands get --index $index)
    if test -n "$cmd"
        commandline -r $cmd
    end
end

function fish_user_key_bindings
    bind \e\[A '__most_frequent_command 0'
    bind \e\[1\;5A '__most_frequent_command 0'  # Ctrl+Up
    bind \e\[1\;2A '__most_frequent_command 1'  # Shift+Up
    bind \e\[1\;3A '__most_frequent_command 2'  # Alt+Up
end 