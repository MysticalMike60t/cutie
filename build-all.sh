#!/bin/sh
set -e

FEATURES="default ansi16 ansi24bit"
PROFILES="dev release release-optimizations"
BIN_NAME="cutie"
DIST="./dist"

mkdir -p "$DIST"

for profile in $PROFILES; do
    printf '\n--- %s profile builds ---\n' "$profile"

    # map profile to cargo's output dir name
    case "$profile" in
        dev) target_dir="debug" ;;
        *) target_dir="$profile" ;;
    esac

    for feature in $FEATURES; do
        if [ "$feature" = "default" ]; then
            cargo build --timings --profile "$profile"
            suffix="${BIN_NAME}-${profile}"
        else
            cargo build --timings --profile "$profile" --features "$feature"
            suffix="${BIN_NAME}-${profile}-${feature}"
        fi

        cp "target/${target_dir}/${BIN_NAME}" "${DIST}/${suffix}"
        printf 'Copied -> %s/%s\n' "$DIST" "$suffix"
    done
done

printf '\n--- Finished. Binaries in %s ---\n' "$DIST"
ls -lh "$DIST"
