# Brocode CLI Runtime for Python SDK

Platform-specific runtime package consumed by the published `brocode-app-server-sdk`.

This package is staged during release so the SDK can pin an exact Brocode CLI
version without checking platform binaries into the repo.

`brocode-cli-bin` is intentionally wheel-only. Do not build or publish an sdist
for this package.
