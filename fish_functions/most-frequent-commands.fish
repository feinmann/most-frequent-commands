function __most_frequent_command
    set -l index $argv[1]
    set -l cmd (most-frequent-commands get --index $index)
    if test -n "$cmd"
        commandline -r $cmd
    end
end

function __cycle_most_frequent
    # Get the current command line
    set -l cmdline (commandline)
    
    # If command line is empty, start with index 0
    if test -z "$cmdline"
        set -g __most_frequent_index 0
    else
        # Increment index or reset to 0
        set -q __most_frequent_index; or set -g __most_frequent_index 0
        set __most_frequent_index (math $__most_frequent_index + 1)
    end
    
    # Get the command at current index
    set -l cmd (most-frequent-commands get --index $__most_frequent_index | string trim)
    
    # If no command found, reset index to 0 and try again
    if test -z "$cmd"
        set -g __most_frequent_index 0
        set cmd (most-frequent-commands get --index 0 | string trim)
    end
    
    # Update command line
    if test -n "$cmd"
        commandline -r $cmd
    end
end

# Bind Alt+j to cycle through most frequent commands
bind \ej __cycle_most_frequent 