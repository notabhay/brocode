#!/bin/bash

# Set "chatgpt.cliExecutable": "/Users/<USERNAME>/code/brocode/scripts/debug-brocode.sh" in VSCode settings to always get the 
# latest brocode-rs binary when debugging Brocode Extension.


set -euo pipefail

BROCODE_RS_DIR=$(realpath "$(dirname "$0")/../brocode-rs")
(cd "$BROCODE_RS_DIR" && cargo run --quiet --bin brocode -- "$@")