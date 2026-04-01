---
name: remote-tests
description: How to run tests using remote executor.
---

Some brocode integration tests support a running against a remote executor.
This means that when BROCODE_TEST_REMOTE_ENV environment variable is set they will attempt to start an executor process in a docker container BROCODE_TEST_REMOTE_ENV points to and use it in tests.

Docker container is built and initialized via ./scripts/test-remote-env.sh

Currently running remote tests is only supported on Linux, so you need to use a devbox to run them

You can list devboxes via `applied_devbox ls`, pick the one with `brocode` in the name.
Connect to devbox via `ssh <devbox_name>`.
Reuse the same checkout of brocode in `~/code/brocode`. Reset files if needed. Multiple checkouts take longer to build and take up more space.
Check whether the SHA and modified files are in sync between remote and local.
