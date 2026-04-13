#!/usr/bin/sh

__pcolor() {
    ___style_color_favcolor="240;120;160"
    ___style_color_red="240;50;50"
    ___style_color_orange="240;160;50"
    ___style_color_yellow="240;220;50"
    ___style_color_green="70;240;50"
    ___style_color_cyan="50;240;220"
    ___style_color_blue="50;100;240"
    ___style_color_purple="160;50;240"
    ___style_color_pink="240;50;240"
    ___style_color_process="160;100;240"
    ___style_color_rust="240;160;80"

    ___print_text_color_rgb() {
        ___rgb="${1:-}"
        shift 1
        printf '\033[38;2;%sm%s\033[0m' "${___rgb}" "${*:-}" || return 1
        return 0
    }

    ___color="${1:-}"
    shift 1
    ___text="${*:-}"
    case "${___color}" in
        favcolor)
            ___print_text_color_rgb "${___style_color_favcolor}" "${___text}"
            ;;
        red)
            ___print_text_color_rgb "${___style_color_red}" "${___text}"
            ;;
        orange)
            ___print_text_color_rgb "${___style_color_orange}" "${___text}"
            ;;
        yellow)
            ___print_text_color_rgb "${___style_color_yellow}" "${___text}"
            ;;
        green)
            ___print_text_color_rgb "${___style_color_green}" "${___text}"
            ;;
        cyan)
            ___print_text_color_rgb "${___style_color_cyan}" "${___text}"
            ;;
        blue)
            ___print_text_color_rgb "${___style_color_blue}" "${___text}"
            ;;
        purple)
            ___print_text_color_rgb "${___style_color_purple}" "${___text}"
            ;;
        pink)
            ___print_text_color_rgb "${___style_color_pink}" "${___text}"
            ;;
        process)
            ___print_text_color_rgb "${___style_color_process}" "${___text}"
            ;;
        rust | cargo)
            ___print_text_color_rgb "${___style_color_rust}" "${___text}"
            ;;
        *)
            return 1
            ;;
    esac
    return 0
}

__no_cargo() {
    ___butter_user_prefix="$(printf '\033[38;2;100;100;100m<\033[38;2;240;120;160mbutter\033[38;2;100;100;100m>\033[0m    ')"
    printf "%sYou don't have $(__pcolor rust "cargo")????\n" "${___butter_user_prefix}" || return 1
    sleep 0.25 || return 1
    printf "%s\033[1;3mWTF????\033[0m\n" "${___butter_user_prefix}" || return 1
    sleep 0.25 || return 1
    printf '%sFine\033[31m.\033[0m' "${___butter_user_prefix}" || return 1
    sleep 0.25 || return 1
    __pcolor red "." || return 1
    sleep 0.25 || return 1
    __pcolor orange "." || return 1
    sleep 0.25 || return 1
    __pcolor yellow "." || return 1
    sleep 0.25 || return 1
    __pcolor green "." || return 1
    sleep 0.25 || return 1
    __pcolor cyan "." || return 1
    sleep 0.25 || return 1
    __pcolor blue "." || return 1
    sleep 0.25 || return 1
    __pcolor purple "." || return 1
    sleep 0.25 || return 1
    __pcolor pink "." || return 1
    sleep 0.25 || return 1
    printf "\n%sI'll find it for you $(__pcolor favcolor ":3")\n" "${___butter_user_prefix}" || return 1
    sleep 0.25 || return 1
    printf "%s$(__pcolor process "Which"): %s\n" "${___butter_user_prefix}" "$(__pcolor blue "$(which cargo)")" || return 1
    return 0
}
