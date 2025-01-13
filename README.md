# Git Local Server

Serves a Git bare repository over local LAN using http server

## Getting started

Install Rust by downloading from https://www.rust-lang.org

## Build

After installing Rust, execute by running:

```sh
cargo run ##
```

or

```sh
cargo run -- --repo <path/to/repository.gir> --port <port_number>
```

To build, use `make`:

```sh
make # to build all targets
make <target_name> # eg. make build_linux_x86_64 to build a linux x86_64 target
```

The release targets will be statically built and output files generated onto
`target/<platform>/release/git-local-server` (for Unix) or
`target/<platform>/release/git-local-server.exe` (for Windows)

## Notes

- Don't use as a public solution. The intention of this script is to serve
  locally in a private environment to private machines
