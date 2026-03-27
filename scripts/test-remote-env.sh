#!/usr/bin/env bash

# Remote-env setup script for brocode-rs integration tests.
#
# Usage (source-only):
#   source scripts/test-remote-env.sh
#   cd brocode-rs
#   cargo test -p brocode-core --test all remote_env_connects_creates_temp_dir_and_runs_sample_script
#   brocode_remote_env_cleanup

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

is_sourced() {
  [[ "${BASH_SOURCE[0]}" != "$0" ]]
}

setup_remote_env() {
  local container_name
  local brocode_exec_server_binary_path

  container_name="${BROCODE_TEST_REMOTE_ENV_CONTAINER_NAME:-brocode-remote-test-env-local-$(date +%s)-${RANDOM}}"
  brocode_exec_server_binary_path="${REPO_ROOT}/brocode-rs/target/debug/brocode-exec-server"

  if ! command -v docker >/dev/null 2>&1; then
    echo "docker is required (Colima or Docker Desktop)" >&2
    return 1
  fi

  if ! docker info >/dev/null 2>&1; then
    echo "docker daemon is not reachable; for Colima run: colima start" >&2
    return 1
  fi

  if ! command -v cargo >/dev/null 2>&1; then
    echo "cargo is required to build brocode-exec-server" >&2
    return 1
  fi

  (
    cd "${REPO_ROOT}/brocode-rs"
    cargo build -p brocode-exec-server --bin brocode-exec-server
  )

  if [[ ! -f "${brocode_exec_server_binary_path}" ]]; then
    echo "brocode-exec-server binary not found at ${brocode_exec_server_binary_path}" >&2
    return 1
  fi

  docker rm -f "${container_name}" >/dev/null 2>&1 || true
  docker run -d --name "${container_name}" ubuntu:24.04 sleep infinity >/dev/null

  export BROCODE_TEST_REMOTE_ENV="${container_name}"
}

brocode_remote_env_cleanup() {
  if [[ -n "${BROCODE_TEST_REMOTE_ENV:-}" ]]; then
    docker rm -f "${BROCODE_TEST_REMOTE_ENV}" >/dev/null 2>&1 || true
    unset BROCODE_TEST_REMOTE_ENV
  fi
}

if ! is_sourced; then
  echo "source this script instead of executing it: source scripts/test-remote-env.sh" >&2
  exit 1
fi

old_shell_options="$(set +o)"
set -euo pipefail
if setup_remote_env; then
  status=0
  echo "BROCODE_TEST_REMOTE_ENV=${BROCODE_TEST_REMOTE_ENV}"
  echo "Remote env ready. Run your command, then call: brocode_remote_env_cleanup"
else
  status=$?
fi
eval "${old_shell_options}"
return "${status}"
