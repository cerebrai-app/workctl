# workctl

Command line interface (Rust).

## Install

### Prebuilt binary (no Rust needed)

Tagged releases publish binaries on the
[Releases page](https://github.com/cerebrai-app/workctl/releases) for:

| Platform | Target |
| --- | --- |
| Linux x86_64 | `x86_64-unknown-linux-gnu` |
| Linux arm64 | `aarch64-unknown-linux-gnu` |
| macOS Apple Silicon | `aarch64-apple-darwin` |
| macOS Intel | `x86_64-apple-darwin` |
| Windows x86_64 | `x86_64-pc-windows-msvc` |

With the [GitHub CLI](https://cli.github.com) (also works if the repo is private):

```sh
gh release download --repo cerebrai-app/workctl --pattern '*x86_64-unknown-linux-gnu*'   # pick your target
shasum -a 256 -c workctl-*.tar.gz.sha256
tar xzf workctl-*.tar.gz
install -m 755 workctl-*/workctl ~/.local/bin/    # any directory on your PATH
```

On Windows, download the `.zip`, extract it, and put `workctl.exe` in a directory on your `PATH`.

### From source, without cloning

Requires a [Rust toolchain](https://rustup.rs) (stable).

```sh
cargo install --git https://github.com/cerebrai-app/workctl --locked
# or a specific release:
cargo install --git https://github.com/cerebrai-app/workctl --tag v0.1.0 --locked
```

This builds and installs to `~/.cargo/bin` (`%USERPROFILE%\.cargo\bin` on Windows), which rustup
normally puts on your `PATH`. Re-run the command with `--force` to upgrade.

### From a local clone

```sh
git clone https://github.com/cerebrai-app/workctl
cd workctl
cargo install --path . --locked
```

Check the install with `workctl version`. To remove it, run `cargo uninstall workctl`.

## Usage

```sh
cargo run -- version
cargo run -- init
```

### `init`

Downloads a [python-build-standalone](https://github.com/astral-sh/python-build-standalone)
distribution to `<data dir>/python-build-standalone/<version>+<build options>/`
(for example `3.14.10+freethreaded+pgo+lto`). The data dir is `$XDG_DATA_HOME` if set to an absolute path, otherwise `%LOCALAPPDATA%` on Windows
and `~/.local/share` elsewhere. It does nothing if the install directory already exists.

`init` looks up the latest release on every run, so it needs network access even when everything is
already installed. A newer Python patch version in a newer release installs into a new directory next
to the old one; pass `--release <tag>` to stay on a specific release.

Defaults to the newest Python 3.14 `freethreaded+pgo+lto` build. If that build isn't published for
your platform (e.g. Windows has no LTO builds) it falls back to the next best build and says so.
Downloads are verified against the release's `SHA256SUMS`.

```sh
workctl init                              # python 3.14, freethreaded+pgo+lto
workctl init --python-version 3.13.9      # exact version
workctl init --variant pgo+lto            # exact build options, no fallback
workctl init --release 20261026           # pin a python-build-standalone release
```

## Development

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo build --release
```

## Releasing

1. Bump `version` in `Cargo.toml` (and commit the updated `Cargo.lock`).
2. Tag and push: `git tag v<version> && git push origin v<version>`.

The `Release` workflow checks that the tag matches `Cargo.toml`, builds every target, and publishes
a GitHub Release with the archives and `.sha256` files.
