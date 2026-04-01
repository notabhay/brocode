set working-directory := "brocode-rs"
set positional-arguments

# Display help
help:
    just -l

# `brocode`
alias c := brocode
brocode *args:
    cargo run --bin brocode -- "$@"

# `brocode exec`
exec *args:
    cargo run --bin brocode -- exec "$@"

# Start brocode-exec-server and run brocode-tui.
[no-cd]
tui-with-exec-server *args:
    ./scripts/run_tui_with_exec_server.sh "$@"

# Run the CLI version of the file-search crate.
file-search *args:
    cargo run --bin brocode-file-search -- "$@"

# Build the CLI and run the app-server test client
app-server-test-client *args:
    cargo build -p brocode-cli
    cargo run -p brocode-app-server-test-client -- --brocode-bin ./target/debug/brocode "$@"

# format code
fmt:
    cargo fmt -- --config imports_granularity=Item 2>/dev/null

fix *args:
    cargo clippy --fix --tests --allow-dirty "$@"

clippy *args:
    cargo clippy --tests "$@"

install:
    rustup show active-toolchain
    cargo fetch

# Run `cargo nextest` since it's faster than `cargo test`, though including
# --no-fail-fast is important to ensure all tests are run.
#
# Run `cargo install cargo-nextest` if you don't have it installed.
# Prefer this for routine local runs; use explicit `cargo test --all-features`
# only when you specifically need full feature coverage.
test:
    cargo nextest run --no-fail-fast

# Build and run Brocode from source using Bazel.
# Note we have to use the combination of `[no-cd]` and `--run_under="cd $PWD &&"`
# to ensure that Bazel runs the command in the current working directory.
[no-cd]
bazel-brocode *args:
    bazel run //brocode-rs/cli:brocode --run_under="cd $PWD &&" -- "$@"

[no-cd]
bazel-lock-update:
    bazel mod deps --lockfile_mode=update

[no-cd]
bazel-lock-check:
    ./scripts/check-module-bazel-lock.sh

bazel-test:
    bazel test --test_tag_filters=-argument-comment-lint //... --keep_going

bazel-clippy:
    bazel build --config=clippy -- //brocode-rs/... -//brocode-rs/v8-poc:all

[no-cd]
bazel-argument-comment-lint:
    bazel build --config=argument-comment-lint -- $(./tools/argument-comment-lint/list-bazel-targets.sh)

bazel-remote-test:
    bazel test --test_tag_filters=-argument-comment-lint //... --config=remote --platforms=//:rbe --keep_going

build-for-release:
    bazel build //brocode-rs/cli:release_binaries --config=remote

# Run the MCP server
mcp-server-run *args:
    cargo run -p brocode-mcp-server -- "$@"

# Regenerate the json schema for config.toml from the current config types.
write-config-schema:
    cargo run -p brocode-core --bin brocode-write-config-schema

# Regenerate vendored app-server protocol schema artifacts.
write-app-server-schema *args:
    cargo run -p brocode-app-server-protocol --bin write_schema_fixtures -- "$@"

[no-cd]
write-hooks-schema:
    cargo run --manifest-path ./brocode-rs/Cargo.toml -p brocode-hooks --bin write_hooks_schema_fixtures

# Run the argument-comment Dylint checks across brocode-rs.
[no-cd]
argument-comment-lint *args:
    if [ "$#" -eq 0 ]; then \
      bazel build --config=argument-comment-lint -- $(./tools/argument-comment-lint/list-bazel-targets.sh); \
    else \
      ./tools/argument-comment-lint/run-prebuilt-linter.py "$@"; \
    fi

[no-cd]
argument-comment-lint-from-source *args:
    ./tools/argument-comment-lint/run.py "$@"

# Tail logs from the state SQLite database
log *args:
    if [ "${1:-}" = "--" ]; then shift; fi; cargo run -p brocode-state --bin logs_client -- "$@"
