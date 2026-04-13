#!/usr/bin/bash

(
logging_script_full_path="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)/logging.sh"

[ -f "${logging_script_full_path}" ] && . "${logging_script_full_path}"

__no_cargo || exit 1
)
