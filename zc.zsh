# change pwd hook
zc_chpwd() {
    zc --add "$(pwd)" >/dev/null
}

typeset -gaU chpwd_functions
chpwd_functions+=(zc_chpwd)

# default zc command
z() {
    if [[ ${1} == -* ]] && [[ ${1} != "--" ]]; then
        zc ${@}
        return
    fi

    setopt localoptions noautonamedirs
    local output="$(zc ${@})"
    if [ -t 1 ]; then  # if stdout is a terminal, use colors
        echo -e "\\033[31m${output}\\033[0m"
    else
        echo -e "${output}"
    fi
    cd "${output}"
}