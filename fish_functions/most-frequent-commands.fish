function __most_frequent_command
    set -l index $argv[1]
    set -l cmd (most-frequent-commands get --index $index)
    if test -n "$cmd"
        commandline -r $cmd
    end
end

function fish_user_key_bindings
    # Ctrl+Shift+Up for most frequent command
    bind \e\[1\;6A '__most_frequent_command 0'
    # Ctrl+Shift+Right for second most frequent
    bind \e\[1\;6C '__most_frequent_command 1'
    # Ctrl+Shift+Left for third most frequent
    bind \e\[1\;6D '__most_frequent_command 2'
end 