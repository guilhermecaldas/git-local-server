macos_aarch64="aarch64-apple-darwin"
linux_x86_64="x86_64-unknown-linux-gnu"
windows_x86_64="x86_64-pc-windows-gnu"

all: build_macos_aarch64 build_linux_linux_x86_64 build_windows_x86_64

build_macos_aarch64:
	rustup target install ${macos_aarch64}
	cargo build --target=${macos_aarch64} --release

build_linux_x86_64:
	rustup target install ${linux_x86_64}
	cargo build --target=${linux_x86_64} --release

build_windows_x86_64:
	rustup target install ${windows_x86_64}
	cargo build --target=${windows_x86_64} --release
