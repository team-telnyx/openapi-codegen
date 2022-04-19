#!/usr/bin/env bash

set -euo pipefail

color() {
    if [[ "${2:-}" == "last" ]]; then
        tput setaf 2
    elif [[ "${2:-}" == "error" ]]; then
        tput setaf 1
    else
        tput setaf "$1"
    fi
}

h_line() {
    color 4 "${1:-}"
    printf '%*s\n' "${COLUMNS:-$(tput cols)}" '' | sed 's/ /═/g'
    tput sgr0
}

banner() {
    if [[ "${2:-}" != "first" && "${2:-}" != "last" ]]; then echo; fi
    h_line "${2:-}"
    printf \
        '%*s' \
        "$(( ( ${COLUMNS:-$(tput cols)} - ${#1} ) / 2))" \
        ''
    color 6 "${2:-}"
    echo "$1"
    tput sgr0
    h_line "${2:-}"
    if [[ "${2:-}" != "last" && "${2:-}" != "error" ]]; then echo; fi
}

failure() {
    if [[ $? -ne 0 ]]; then
        banner "Failure" error
    fi
}

trap failure EXIT

banner "Versions" first
rustc --version
cargo --version
rustfmt --version
rustdoc --version
cargo clippy -- --version
nixpkgs-fmt --version 2>&1 | head -n1 || true
cargo outdated -R

banner "Linting"
nixpkgs-fmt --check .
cargo fmt -- --check
cargo clippy -- -D warnings

banner "Testing"
cargo test

banner "Success" last
