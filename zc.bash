# change pwd hook
zc_add() {
    zc --add "$(pwd)" > /dev/null &
}

case $PROMPT_COMMAND in
    *autojump*)
        ;;
    *)
        PROMPT_COMMAND="${PROMPT_COMMAND:+$(echo "${PROMPT_COMMAND}" | awk '{gsub(/; *$/,"")}1') ; }zc_add"
        ;;
esac


# default zc command
z() {
    if [[ ${1} == -* ]] && [[ ${1} != "--" ]]; then
        zc ${@}
        return
    fi

    output="$(zc ${@})"
    if [ -t 1 ]; then  # if stdout is a terminal, use colors
        echo -e "\\033[31m${output}\\033[0m"
    else
        echo -e "${output}"
    fi
    cd "${output}"
}