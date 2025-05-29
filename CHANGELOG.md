# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Comprehensive CI/CD pipeline with security checks
- Dependabot configuration for automated dependency updates
- Release automation workflow
- Documentation checks
- MSRV (Minimum Supported Rust Version) testing
- Security scanning with cargo-audit and OSSF Scorecard
- PR labeling automation
- Changelog enforcement

### Changed
- Updated GitHub Actions to use latest versions
- Enhanced CI workflow with better caching and performance optimizations
- Added concurrency controls to cancel outdated builds

### Security
- Added permissions restrictions to all workflows
- Implemented dependency review for pull requests
- Added supply chain security checks with cargo-deny

## [0.1.2] - 2025-01-29

### Added
- Static analysis tools integration (cargo-audit, cargo-deny, cargo-outdated)
- Enhanced clippy linting in CI pipeline

### Fixed
- Resolved all clippy warnings
- Fixed duplicate tauri.conf.json files
- Corrected package name inconsistencies

### Changed
- Updated to semantic versioning
- Improved error handling

## [0.1.1] - 2025-01-29

### Added
- Screenshot and demo GIF in README
- CI/CD badges
- Detailed cost breakdown for API usage
- Contributing guidelines
- API key privacy information

### Changed
- Updated copyright year to 2025
- Enhanced README with better documentation

## [0.1.0] - 2024-12-15

### Added
- Initial release
- Latin word lookup functionality using Claude AI
- Cross-platform desktop application (Windows, macOS, Linux)
- Custom prompt support
- API key management
- Real-time Latin word analysis with macrons
- Principal parts and stems for verbs

[Unreleased]: https://github.com/warddave/latin-word-lookup/compare/v0.1.2...HEAD
[0.1.2]: https://github.com/warddave/latin-word-lookup/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/warddave/latin-word-lookup/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/warddave/latin-word-lookup/releases/tag/v0.1.0