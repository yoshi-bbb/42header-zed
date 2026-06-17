# Contributing

Thanks for improving `zed-42header`.

## Development

Install Rust, then run:

```sh
cargo test
```

Before opening a pull request, run:

```sh
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
```

## Project Scope

`zed-42header` is a CLI formatter designed to be called from Zed's external formatter settings. Keep changes focused on:

- generating 42 headers;
- updating existing headers without changing `Created` metadata;
- supporting additional Zed languages or file extensions;
- improving cross-platform behavior.

Zed extension-registry support may be added later if Zed exposes an extension capability that can safely edit the active buffer or manage this CLI.

## Pull Requests

Please include:

- the behavior change;
- a minimal input/output example when formatting behavior changes;
- tests for new comment styles, language mappings, or header parsing.

