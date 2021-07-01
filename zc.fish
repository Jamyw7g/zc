# change pwd hook
function __zc_add --on-variable PWD
    status --is-command-substitution; and return
    zc --add (pwd) > /dev/null &
end


# default zc command
function z
    switch "$argv"
        case '-*' '--*'
            zc $argv
        case '*'
            set -l output (zc $argv)
            # Check for . and attempt a regular cd
            if [ $output = "." ]
                cd $argv
            else
                set_color red
                echo $output
                set_color normal
                cd $output
            end
    end
end