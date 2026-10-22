#!/bin/bash
# Run a command with this checkout's optional, isolated macOS Rust toolchain.
# This script does not install software or modify the parent shell's environment.
set -euo pipefail

if [[ "$(uname -s)" != Darwin ]]; then
  echo 'mac-dev.sh requires macOS.' >&2
  exit 1
fi

tokendaily_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
if [[ -x "$tokendaily_root/.local-toolchain/mac-cargo/bin/cargo" ]]; then
  export CARGO_HOME="$tokendaily_root/.local-toolchain/mac-cargo"
  export RUSTUP_HOME="$tokendaily_root/.local-toolchain/mac-rustup"
  export PATH="$CARGO_HOME/bin:$PATH"
fi

if [[ $# -eq 0 ]]; then
  echo 'Usage: bash scripts/mac-dev.sh <command> [arguments...]' >&2
  exit 2
fi

cd "$tokendaily_root"
exec "$@"
