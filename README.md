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

The Linux binaries are built on Ubuntu 22.04, so they need glibc 2.35 or newer (Ubuntu 22.04 and
later, Debian 12 and later). On older systems, build from source; static musl binaries, which would
run anywhere, are a [stretch goal](#stretch-goals).

### From source, without cloning

Requires a [Rust toolchain](https://rustup.rs), version 1.85 or newer. That is the oldest release the
dependencies support (`rust-version` in `Cargo.toml`), and CI builds and tests with exactly that
version. Rust has no long-term-support channel, so the floor is raised only when a dependency or our
code needs a newer release.

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
to the old one. Older directories are never removed (each is a few hundred MB): delete the ones you no
longer need by hand.

A later release that rebuilds the *same* Python version is not picked up: `init` finds the existing
directory and does nothing, so delete it if you want the newer build.

`--python-version` takes a Python version (`3`, `3.14` or `3.14.15`), never a python-build-standalone
release date.

Defaults to the newest Python 3.14 `freethreaded+pgo+lto` build. If that build isn't published for
your platform (e.g. Windows has no LTO builds) it falls back to the next best build and says so.
Downloads are checked against the release's `SHA256SUMS`, which catches corrupt or truncated downloads.
The checksums come from the same GitHub release as the archive, so this does not protect against a
compromised release.

```sh
workctl init                              # python 3.14, freethreaded+pgo+lto
workctl init --python-version 3           # newest Python 3.x
workctl init --python-version 3.13.9      # exact version
workctl init --variant pgo+lto            # exact build options, no fallback
```

## Development

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo build --release
```

CI also runs the unit tests and a `version` smoke test for **arm64 Linux under QEMU**, since there is
no arm64 runner. To try it on an x86_64 Linux machine:

```sh
sudo apt-get install gcc-aarch64-linux-gnu qemu-user
rustup target add aarch64-unknown-linux-gnu
export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc \
       CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc \
       CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_RUNNER="qemu-aarch64 -L /usr/aarch64-linux-gnu"
cargo test --target aarch64-unknown-linux-gnu --bins
cargo run --target aarch64-unknown-linux-gnu -- version
```

## Stretch goals

- **Static musl builds** (`x86_64-unknown-linux-musl` and `aarch64-unknown-linux-musl`): one binary
  that runs on any Linux, including distributions older than glibc 2.35 and Alpine. The arm64 build
  would be tested under QEMU like the gnu one. The C dependencies (`zstd`, `ring`) need `musl-tools`,
  and arm64 needs a musl cross compiler, which is why this is not done yet.

## Releasing

1. Bump `version` in `Cargo.toml` (and commit the updated `Cargo.lock`).
2. Tag and push: `git tag v<version> && git push origin v<version>`.

The `Release` workflow checks that the tag matches `Cargo.toml`, builds every target, and publishes
a GitHub Release with the archives and `.sha256` files.
