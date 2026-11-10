# workctl

Command line interface (Rust).

## Install locally

Requires a [Rust toolchain](https://rustup.rs) (stable).

```sh
git clone https://github.com/cerebrai-app/workctl
cd workctl
cargo install --path . --locked
```

This builds a release binary and installs it to `~/.cargo/bin` (`%USERPROFILE%\.cargo\bin` on
Windows), which rustup normally puts on your `PATH`. Check it with:

```sh
workctl version
```

To upgrade, `git pull` and re-run the `cargo install` command (it replaces the old binary).
To remove it, run `cargo uninstall workctl`.

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
