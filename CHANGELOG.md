# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [v2.2.2] - 2025-02-13

### Fixed

- New repositories not initializing correctly

## [v2.2.1] - 2025-02-10

### Added

- Build optimization settings. Binaries are now up to 70% smaller.

### Fixed

- Broken MacOS build due to OpenSSL lib not statically linking
- MacOS binaries not verified displaying warning

## [2.2.0] - 2025-02-05

### Added

- New command `set-head` which sets HEAD reposititory reference

### Fixed

- New repositories now use "develop" as HEAD branch by default, to avoid errors
  when trying to delete branches in repositories without "main". This default
  choise is based on ocurrence of "develop" branches compared to "main" in our
  projects local workflow

## [2.1.0] - 2025-01-17

### Changed

- Split code into `serve` and `init` commands
- Improved repository configuration handling
- Removed unused code and dependencies

## [2.0.0] - 2025-01-13

### Changed

- Migrated server to Rust using static git2, dav_server and warp
- Releases are static, so don't depend on Git or any platform dependency

### Fixed

- User had to `push force` in order to create new branch

## [1.0.0] - 2024-12-19

### Added

- First release (Shell Script and Rclone based)
