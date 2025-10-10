macos_aarch64="aarch64-apple-darwin"
macos_x86_64="x86_64-apple-darwin"
windows_x86_64="x86_64-pc-windows-gnu"
linux_aarch64="aarch64-unknown-linux-musl"
linux_x86_64="x86_64-unknown-linux-musl"

assets_dir="target/assets"
bin_name="git-local-server"

target_macos_aarch64="target/$(macos_aarch64)/release/$(bin_name)"
target_macos_x86_64="target/$(macos_x86_64)/release/$(bin_name)"
target_linux_aarch64="target/$(linux_aarch64)/release/$(bin_name)"
target_linux_x86_64="target/$(linux_x86_64)/release/$(bin_name)"
target_windows_x86_64="target/$(windows_x86_64)/release/$(bin_name).exe"

assets_macos_aarch64=$(assets_dir)/$(bin_name)-macos-aarch64
assets_macos_x86_64=$(assets_dir)/$(bin_name)-macos-x86_64
assets_linux_aarch64=$(assets_dir)/$(bin_name)-linux-aarch64
assets_linux_x86_64=$(assets_dir)/$(bin_name)-linux-x86_64
assets_windows_x86_64=$(assets_dir)/$(bin_name)-windows-x86_64.exe

macos_bundle_id="tech.chemis.gitlocalserver"
macos_aarch64_unsigned_pkg_path=$(assets_dir)/$(bin_name)-macos-aarch64-unsigned.pkg
macos_aarch64_pkg_path=$(assets_dir)/$(bin_name)-macos-aarch64.pkg
macos_x86_64_unsigned_pkg_path=$(assets_dir)/$(bin_name)-macos-x86_64-unsigned.pkg
macos_x86_64_pkg_path=$(assets_dir)/$(bin_name)-macos-x86_64.pkg

# Build all target architectures and create the assets directory.
all: build_linux_aarch64 build_linux_x86_64 build_windows_x86_64 bundle_macos_aarch64_pkg bundle_macos_x86_64_pkg
	@echo "Moving binaries and packages to target/assets..."
	@cp $(target_macos_aarch64) $(assets_macos_aarch64)
	@cp $(target_macos_x86_64) $(assets_macos_x86_64)
	@cp $(target_linux_aarch64) $(assets_linux_aarch64)
	@cp $(target_linux_x86_64) $(assets_linux_x86_64)
	@cp $(target_windows_x86_64) $(assets_windows_x86_64)

# Create the assets directory.
make_assets_dir:
	@mkdir -p target/assets

# Build for macOS aarch64.
build_macos_aarch64:
	rustup target install $(macos_aarch64)
	cargo build --target=$(macos_aarch64) --release

# Build for macOS x86_64.
build_macos_x86_64:
	rustup target install $(macos_x86_64)
	cargo build --target=$(macos_x86_64) --release

# Build for Linux aarch64 using Podman.
build_linux_aarch64:
	podman pull ghcr.io/rust-cross/rust-musl-cross:aarch64-musl
	podman run --rm -it -v "$(PWD)":/home/rust/src \
		ghcr.io/rust-cross/rust-musl-cross:aarch64-musl \
		cargo build --release

# Build for Linux x86_64 using Podman.
build_linux_x86_64:
	podman pull ghcr.io/rust-cross/rust-musl-cross:x86_64-musl
	podman run --rm -it -v "$(PWD)":/home/rust/src \
		ghcr.io/rust-cross/rust-musl-cross:x86_64-musl \
		cargo build --release

# Build for Windows x86_64.
build_windows_x86_64:
	rustup target install $(windows_x86_64)
	cargo build --target=$(windows_x86_64) --release

# Bundle the macOS aarch64 build into a signed pkg.
bundle_macos_aarch64_pkg: build_macos_aarch64 make_assets_dir
	@echo "Signing macOS binary..."
	codesign --force -s "$(DEVELOPER_ID_APPLICATION)" \
	-v target/$(macos_aarch64)/release/$(bin_name) --strict --options=runtime --timestamp

	@echo "Creating unsigned macOS package..."
	mkdir -p "/tmp/gls/usr/local/bin"
	cp target/$(macos_aarch64)/release/$(bin_name) "/tmp/gls/usr/local/bin/"
	pkgbuild --root "/tmp/gls" \
	--identifier $(macos_bundle_id) \
	--version "$(shell cargo pkgid | cut -d# -f2)" \
	--install-location "/" \
	--sign "$(DEVELOPER_ID_INSTALLER)" $(macos_aarch64_unsigned_pkg_path)

	@echo "Signing macOS package..."
	productbuild --package $(macos_aarch64_unsigned_pkg_path) \
	--sign "$(DEVELOPER_ID_INSTALLER)" $(macos_aarch64_pkg_path)

	@echo "Submitting macOS package for notarization..."
	xcrun notarytool submit $(macos_aarch64_pkg_path) \
	--keychain-profile "$(KEYCHAIN_PROFILE)" --wait

	@echo "Stapling macOS package..."
	xcrun stapler staple $(macos_aarch64_pkg_path)

	rm $(macos_aarch64_unsigned_pkg_path)
	rm -rf "/tmp/gls"

# Bundle the macOS aarch64 build into a signed pkg.
bundle_macos_x86_64_pkg: build_macos_x86_64 make_assets_dir
	@echo "Signing macOS binary..."
	codesign --force -s "$(DEVELOPER_ID_APPLICATION)" \
	-v target/$(macos_x86_64)/release/$(bin_name) --strict --options=runtime --timestamp

	@echo "Creating unsigned macOS package..."
	mkdir -p "/tmp/gls/usr/local/bin"
	cp target/$(macos_x86_64)/release/$(bin_name) "/tmp/gls/usr/local/bin/"
	pkgbuild --root "/tmp/gls" \
	--identifier $(macos_bundle_id) \
	--version "$(shell cargo pkgid | cut -d# -f2)" \
	--install-location "/" \
	--sign "$(DEVELOPER_ID_INSTALLER)" $(macos_x86_64_unsigned_pkg_path)

	@echo "Signing macOS package..."
	productbuild --package $(macos_x86_64_unsigned_pkg_path) \
	--sign "$(DEVELOPER_ID_INSTALLER)" $(macos_x86_64_pkg_path)

	@echo "Submitting macOS package for notarization..."
	xcrun notarytool submit $(macos_x86_64_pkg_path) \
	--keychain-profile "$(KEYCHAIN_PROFILE)" --wait

	@echo "Stapling macOS package..."
	xcrun stapler staple $(macos_x86_64_pkg_path)

	rm $(macos_x86_64_unsigned_pkg_path)
	rm -rf "/tmp/gls"

# Clean the build artifacts.
clean:
	@echo "Cleaning Rust build artifacts..."
	cargo clean
