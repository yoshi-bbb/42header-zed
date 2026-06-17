# Publishing

This project can be published in two practical ways:

1. GitHub repository: users install with `cargo install --git`.
2. crates.io package: users install with `cargo install zed-42header`.

The Zed extension registry is a separate path. Zed registry submissions currently require an `extension.toml` Zed extension repository and a PR to `zed-industries/extensions`. This project is currently an external formatter CLI, not a registry extension.

## GitHub

Create a public repository, then push:

```sh
git init
git add .
git commit -m "Initial release"
git branch -M main
git remote add origin https://github.com/yoshi-bbb/42header-zed.git
git push -u origin main
```

After publishing, users can install with:

```sh
cargo install --git https://github.com/yoshi-bbb/42header-zed
```

## crates.io

Before publishing:

```sh
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
cargo package
```

Then publish:

```sh
cargo login
cargo publish
```

After publishing, users can install with:

```sh
cargo install zed-42header
```

## Zed Extension Registry

If this project later grows a real Zed extension wrapper:

1. Add `extension.toml` at the extension root.
2. Test it locally with Zed's dev extension flow.
3. Open a PR to `zed-industries/extensions`.
4. Add the repository as a public HTTPS submodule under `extensions/`.
5. Add an entry to `extensions.toml`.
6. Run `pnpm sort-extensions` in the Zed extensions repository.
