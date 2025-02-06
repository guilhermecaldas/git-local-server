# Git Local Server

Serves a Git bare repository over local LAN using an HTTP server

## Getting started

Install Rust by downloading from https://www.rust-lang.org

## Run

After installing Rust, run by executing:

```sh
cargo run -- init repo_name.git
# or
cargo run -- serve -p <port> -d <root_dir> -a <ipv4_addr>
```

## Build

To build, use `make`:

```sh
make all # to build all targets
make <target_name> # eg. make build_linux_x86_64 to build a Linux x86_64 target
```

The release targets will be statically built and output files will be generated
in `target/<platform>/release/git-local-server` (for Unix) or
`target/<platform>/release/git-local-server.exe` (for Windows)

## Install

To install as a crate, just run:

```sh
cargo install --path .
```

And then, it will be available on your path:

```sh
git-local-server serve
```

## Notes

Don't use as a public solution. The intention of this script is to serve files
locally in a private environment to private machines
