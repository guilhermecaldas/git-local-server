macos_aarch64="aarch64-apple-darwin"
windows_x86_64="x86_64-pc-windows-gnu"

build_macos_aarch64:
	rustup target install ${macos_aarch64}
	cargo build --target=${macos_aarch64} --release

build_linux_aarch64:
	podman pull ghcr.io/rust-cross/rust-musl-cross:aarch64-musl
	podman run --rm -it -v "$(PWD)":/home/rust/src \
	   ghcr.io/rust-cross/rust-musl-cross:aarch64-musl \
	   cargo build --release

build_linux_x86_64:
	podman pull ghcr.io/rust-cross/rust-musl-cross:x86_64-musl
	podman run --rm -it -v "$(PWD)":/home/rust/src \
	   ghcr.io/rust-cross/rust-musl-cross:x86_64-musl \
	   cargo build --release

build_windows_x86_64:
	rustup target install ${windows_x86_64}
	cargo build --target=${windows_x86_64} --release

bundle_macos_pkg:
	codesign --force -s "$(DEVELOPER_ID_APPLICATION)" \
	-v target/${macos_aarch64}/release/git-local-server \
	--strict --options=runtime --timestamp

	cp -p target/${macos_aarch64}/release/git-local-server /tmp/gls

	VERSION := $(shell cargo pkgid | cut -d# -f2)

	pkgbuild --root "/tmp/gls" \
	--identifier "tech.chemis.gitlocalserver" \
	--version "${VERSION}" \
	--install-location "/usr/local/bin" \
	--sign "$(DEVELOPER_ID_INSTALLER)" git-local-server-unsigned.pkg

	productbuild --package git-local-server-unsigned.pkg \
	--sign "$(DEVELOPER_ID_INSTALLER)" git-local-server.pkg

	xcrun notarytool submit git-local-server.pkg \
	--keychain-profile "$(KEYCHAIN_PROFILE)" --wait

	xcrun stapler staple git-local-server.pkg
	mv git-local-server.pkg target/aarch64-apple-darwin/release
	rm git-local-server-unsigned.pkg
