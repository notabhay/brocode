# npm releases

Use the staging helper in the repo root to generate npm tarballs for a release. For
example, to stage the CLI, responses proxy, and SDK packages for version `0.6.0`:

```bash
./scripts/stage_npm_packages.py \
  --release-version 0.6.0 \
  --package brocode \
  --package brocode-responses-api-proxy \
  --package brocode-sdk
```

This downloads the native artifacts once, hydrates `vendor/` for each package, and writes
tarballs to `dist/npm/`.

When `--package brocode` is provided, the staging helper builds the lightweight
`@openai/brocode` meta package plus all platform-native `@openai/brocode` variants
that are later published under platform-specific dist-tags.

If you need to invoke `build_npm_package.py` directly, run
`brocode-cli/scripts/install_native_deps.py` first and pass `--vendor-src` pointing to the
directory that contains the populated `vendor/` tree.
