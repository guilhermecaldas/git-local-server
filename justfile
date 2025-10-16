version := `cargo pkgid | cut -d# -f2`
assets_dir := "target/assets"
bin_name := "git-local-server"
macos_bundle_id := "tech.chemis.gitlocalserver"
bundle_temp_dir := "/tmp/gls"
bundle_temp_bin_dir := "/tmp/gls/usr/local/bin"

# List available recipes
_default:
    @just --list

# Run all build and bundle recipes
all: build-all bundle-all generate-assets

# Build all targets available
[group("build")]
build-all: build-macos-aarch64 build-macos-x86_64 build-windows-x86_64 build-linux-aarch64 build-linux-x86_64

[group("build")]
build-macos-aarch64: (_build "aarch64-apple-darwin")

[group("build")]
build-macos-x86_64: (_build "x86_64-apple-darwin")

[group("build")]
build-windows-x86_64: (_build "x86_64-pc-windows-gnu")

[group("build")]
build-linux-aarch64: (_build-linux "aarch64-unknown-linux-musl" "aarch64-musl")

[group("build")]
build-linux-x86_64: (_build-linux "x86_64-unknown-linux-musl" "x86_64-musl")

# Bundle all MacOS targets
[group("bundle")]
[macos]
bundle-all: bundle-macos-aarch64 bundle-macos-x86_64

# Bundle MacOS aarch64 .pkg
[group("bundle")]
[macos]
bundle-macos-aarch64: (_bundle-macos "aarch64-apple-darwin" "-macos-aarch64")

# Bundle MacOS x86_64 .pkg
[group("bundle")]
[macos]
bundle-macos-x86_64: (_bundle-macos "x86_64-apple-darwin" "-macos-x86_64")

# Bundle the macOS build into a signed pkg.
[group("bundle")]
[macos]
_bundle-macos target suffix:
    # sign binary
    codesign --force -s "$DEVELOPER_ID_APPLICATION" \
    -v target/{{ target }}/release/{{ bin_name }} --strict --options=runtime --timestamp

    # move signed binary to temporary directory
    mkdir -p {{ bundle_temp_bin_dir }}
    cp target/{{ target }}/release/{{ bin_name }} {{ bundle_temp_bin_dir }}

    # create unsigned package
    mkdir -p {{ assets_dir }}
    pkgbuild --root {{ bundle_temp_dir }} \
    --identifier {{ macos_bundle_id }} \
    --version {{ version }} \
    --install-location "/" \
    --sign "$DEVELOPER_ID_INSTALLER" {{ assets_dir }}/{{ target }}.unsigned.pkg

    # sign package
    productbuild --package {{ assets_dir }}/{{ target }}.unsigned.pkg \
    --sign "$DEVELOPER_ID_INSTALLER" {{ assets_dir }}/{{ bin_name }}{{ suffix }}.pkg

    # submit package for notarization
    xcrun notarytool submit {{ assets_dir }}/{{ bin_name }}{{ suffix }}.pkg \
    --keychain-profile "$KEYCHAIN_PROFILE" --wait

    # staple package
    xcrun stapler staple {{ assets_dir }}/{{ bin_name }}{{ suffix }}.pkg

    # clean workspace
    rm {{ assets_dir }}/{{ target }}.unsigned.pkg
    rm -rf {{ bundle_temp_dir }}

# Copy binary to assets_dir with publish name
[group("tools")]
_copy-asset target bin_suffix extension="":
    cp target/{{ target }}/release/{{ bin_name }}{{ extension }} \
    {{ assets_dir }}/{{ bin_name }}{{ bin_suffix }}{{ extension }}

# Generate named assets binary files
[group("tools")]
generate-assets:
    mkdir -p {{ assets_dir }}
    @just _copy-asset "aarch64-apple-darwin" "-macos-aarch64"
    @just _copy-asset "x86_64-apple-darwin" "-macos-x86_64"
    @just _copy-asset "aarch64-unknown-linux-musl" "-linux-aarch64"
    @just _copy-asset "x86_64-unknown-linux-musl" "-linux-x86_64"
    @just _copy-asset "x86_64-pc-windows-gnu" "-windows-x86_64" ".exe"

# Build target on host machine
[group("tools")]
_build target:
    rustup target install {{ target }}
    cargo build --target={{ target }} --release

# Build Linux target on Podman image
[group("tools")]
_build-linux target arch:
    podman pull ghcr.io/rust-cross/rust-musl-cross:{{ arch }}
    podman run --rm -it -v "$(PWD)":/home/rust/src \
        ghcr.io/rust-cross/rust-musl-cross:{{ arch }} \
        cargo build --release --target={{ target }}

# Clean the build artifacts.
[group("tools")]
clean:
    cargo clean
