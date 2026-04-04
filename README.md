# Git Local Server

Serves a Git bare repository over local LAN using an HTTP server

> [!WARNING]
> Don't use as a public solution. The intention of this script is to serve files
> locally in a private environment to private machines

## Getting started

Install Rust by downloading from https://www.rust-lang.org

## Run

After installing Rust, run by executing:

```sh
cargo run -- init <repo_name.git>
# or
cargo run -- serve <root_dir> -p <port> -a <ipv4_addr>
# or
cargo run -- set-head <repo_path> <branch>
```

## Commands

- `serve [PATH]` - Serves Git repositories inside of a specified directory (defaults to current directory)
- `init <REPO_NAME>` - Initializes a Git repository in the specified path
- `set-head <REPOSITORY> <BRANCH>` - Sets the HEAD branch for a repository

## Options for `serve`

- `-p, --port <PORT>` - Port number (default: 5005)
- `-a, --addr <ADDR>` - IPv4 address (default: 0.0.0.0)
- `--no-timeout` - Disable server timeout (not recommended)

## Build

To build, use `just`:

```sh
just build <target> # e.g. just build x86_64-unknown-linux-musl
```

Example of rustup build targets:
- `aarch64-apple-darwin` (macOS Apple Silicon)
- `x86_64-apple-darwin` (macOS Intel)
- `aarch64-unknown-linux-musl` (Linux ARM64)
- `x86_64-unknown-linux-musl` (Linux x86_64)
- `x86_64-unknown-freebsd` (FreeBSD x86_64)
- `x86_64-pc-windows-gnu` (Windows x86_64)

You can also build all default targets at once:

```sh
just build-all
```

The release targets will be statically built using `cargo-zigbuild` and output
files will be generated in `target/<target>/release/git-local-server`.
Name-mapped artifacts can be generated in `target/artifacts` using:

```sh
just generate-assets
```

## Install

To install as a crate, just run:

```sh
cargo install git-local-server
```

Alternatively, to install from source:

```sh
cargo install --path .
```

And then, it will be available on your path:

```sh
git-local-server serve
```
