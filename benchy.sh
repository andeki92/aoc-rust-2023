#!/bin/bash

# exit on the first error, see: http://stackoverflow.com/a/185900/432509
error() {
    local parent_lineno="$1"
    local message="$2"
    local code="${3:-1}"
    if [[ -n "$message" ]]; then
        echo "Error on or near line ${parent_lineno}: ${message}; exiting with status ${code}"
    else
        echo "Error on or near line ${parent_lineno}; exiting with status ${code}"
    fi
    exit "${code}"
}
trap 'error ${LINENO}' ERR
# done with trap

# Support cargo command override.
if [[ -z $CARGO_BIN ]]; then
    CARGO_BIN=cargo
fi

# toplevel git repo
ROOT=$(git rev-parse --show-toplevel)

for cargo_dir in $(find "$ROOT" -name Cargo.toml | xargs -n1 -I {} dirname {} | sort -t/); do
    project=$(basename $cargo_dir | sed 's/-/ /g' | awk '{ print toupper(substr($0, 1, 1)) substr($0, 2) }')
    pushd "$cargo_dir" >/dev/null

    if [ -f "src/main.rs" ]; then
        echo "=============== ${project} ==============="
        RUST_BACKTRACE=0 $CARGO_BIN test --quiet
        RUST_BACKTRACE=0 $CARGO_BIN run --quiet
        echo ""
        RUST_BACKTRACE=0 $CARGO_BIN build --release >/dev/null
        hyperfine -N ./target/release/$(basename $cargo_dir) 2>/dev/null
    fi

    popd >/dev/null
done
