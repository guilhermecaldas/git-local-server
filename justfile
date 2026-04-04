version := `cargo pkgid | cut -d# -f2`
artifacts_dir := "target/artifacts"
bin_name := "git-local-server"
macos_bundle_id := "tech.chemis.gitlocalserver"
bundle_temp_dir := "/tmp/gls"
bundle_temp_bin_dir := "/tmp/gls/usr/local/bin"

# targets and platforms must be in sync

targets := "aarch64-apple-darwin \
            x86_64-apple-darwin \
            aarch64-unknown-linux-musl \
            x86_64-unknown-linux-musl \
            x86_64-unknown-freebsd \
            x86_64-pc-windows-gnu"
platforms := "macos-aarch64 \
              macos-x86_64 \
              linux-aarch64 \
              linux-x86_64 \
              freebsd-x86_64 \
              windows-x86_64"

# List available recipes
_default:
    @just --list

# Run all build and artifacts generation recipes
all: build-all generate-artifacts

# Build all specified targets or try building default ones set on `targets`
[group("build")]
build-all +targets=targets:
    #!/usr/bin/env bash
    set -euo pipefail
    for target in {{ targets }}; do
        echo ""
        just build ${target};
    done

# Build target on host machine
[group("build")]
build target:
    cargo zigbuild --target {{ target }} --release

# Bundle all MacOS targets
[group("bundle")]
[macos]
bundle-all-macos: bundle-macos-aarch64 bundle-macos-x86_64

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
    mkdir -p {{ artifacts_dir }}
    pkgbuild --root {{ bundle_temp_dir }} \
    --identifier {{ macos_bundle_id }} \
    --version {{ version }} \
    --install-location "/" \
    --sign "$DEVELOPER_ID_INSTALLER" {{ artifacts_dir }}/{{ target }}.unsigned.pkg

    # sign package
    productbuild --package {{ artifacts_dir }}/{{ target }}.unsigned.pkg \
    --sign "$DEVELOPER_ID_INSTALLER" {{ artifacts_dir }}/{{ bin_name }}{{ suffix }}.pkg

    # submit package for notarization
    xcrun notarytool submit {{ artifacts_dir }}/{{ bin_name }}{{ suffix }}.pkg \
    --keychain-profile "$KEYCHAIN_PROFILE" --wait

    # staple package
    xcrun stapler staple {{ artifacts_dir }}/{{ bin_name }}{{ suffix }}.pkg

    # clean workspace
    rm {{ artifacts_dir }}/{{ target }}.unsigned.pkg
    rm -rf {{ bundle_temp_dir }}

# Generate named binary file artifacts
[group("tools")]
generate-artifacts:
    #!/usr/bin/env bash
    set -euo pipefail

    rm -rf {{ artifacts_dir }}
    mkdir -p {{ artifacts_dir }}

    targets=({{ targets }})
    platforms=({{ platforms }})

    for i in "${!targets[@]}"; do
        case "${platforms[$i]}" in
            *windows*) ext=".exe" ;;
            *)         ext=""     ;;
        esac

        cp "target/${targets[$i]}/release/{{ bin_name }}${ext}" \
        "{{ artifacts_dir }}/{{ bin_name }}-{{ version }}-${platforms[$i]}${ext}"
    done
    echo -e "\nArtifacts available at {{ artifacts_dir }}"

# List installed rustup targets
[group("tools")]
list-installed-targets:
    rustup target list | grep installed

# Clean the build artifacts.
[group("tools")]
clean:
    cargo clean
