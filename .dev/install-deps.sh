#!/usr/bin/bash

[ -f "./logging.sh" ] && . "./logging.sh"

if command -v cargo >"dev/null"; then
    cargo install cargo-msrv || exit 1
else
    __no_cargo || exit 1
    exit 1
fi

exit 0
