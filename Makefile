macos_aarch64="aarch64-apple-darwin"
linux_x86_64="x86_64-unknown-linux-musl"
windows_x86_64="x86_64-pc-windows-gnu"

build_macos_aarch64:
	rustup target install ${macos_aarch64}
	cargo build --target=${macos_aarch64} --release

build_linux_x86_64:
	rustup target install ${linux_x86_64}
	cargo build --target=${linux_x86_64} --release

build_linux_x86_64_cross:
	podman run -tiv "$(PWD)":/usr/src/build \
        -v cargo-git:/home/rust/.cargo/git \
        -v cargo-registry:/home/rust/.cargo/registry \
        rust-builder

build_windows_x86_64:
	rustup target install ${windows_x86_64}
	cargo build --target=${windows_x86_64} --release
