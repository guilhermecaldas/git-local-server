# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.0.0] - 2025-01-13

### Changed

- Migrated server to Rust using static git2, dav_server and warp
- Releases are static, so don't depend on Git or any platform dependency

### Fixed

- User had to `push force` in order to create new branch

## [1.0.0] - 2024-12-19

### Added

- First release (Shell Script and Rclone based)
