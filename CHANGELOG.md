# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.1]

### Added

- API documentation: autogenerate builder function descriptions.

### Fixed

- Upgrade Azure SDK dependencies:
  - azure_core: 0.3
  - azure_identity: 0.4

## [0.3.0]

### Breaking change

- Move `Credential` definition from `auth` module to root, to separate it from all the
  feature modules. `auth` module is now private.
  - To migrate change `use azure_devops_rust_api::auth::Credential` to `use azure_devops_rust_api::Credential`

### Added

- API documentation: autogenerate function description and parameter descriptions from fields
  in the OpenAPI spec.

## [0.2.0]

### Breaking change

- Credentials now need to be provided via a new type `auth::Credential` that supports
  both PAT and TokenCredential types. For more details see examples and docs.rs.

### Added

- Support for PAT authentication

### Fixed

- Remove use of `unwrap()` from examples.

## [0.1.3] - 2022-07-29

### Fixed

- Remove `--no-deps` flag for docs.rs documentation generation.

## [0.1.2] - 2022-07-29

### Fixed

- Set `--no-deps` flag for docs.rs documentation generation.

## [0.1.1] - 2022-07-29

### Fixed

- Enable `all-features` flag for docs.rs documentation generation.

## [0.1.0] - 2022-07-29

- Initial release.

[Unreleased]: https://github.com/microsoft/azure-devops-rust-api/compare/0.3.1...HEAD
[0.3.1]: https://github.com/microsoft/azure-devops-rust-api/compare/0.3.0...0.3.1
[0.3.0]: https://github.com/microsoft/azure-devops-rust-api/compare/0.2.0...0.3.0
[0.2.0]: https://github.com/microsoft/azure-devops-rust-api/compare/0.1.3...0.2.0
[0.1.3]: https://github.com/microsoft/azure-devops-rust-api/compare/0.1.2...0.1.3
[0.1.2]: https://github.com/microsoft/azure-devops-rust-api/compare/0.1.1...0.1.2
[0.1.1]: https://github.com/microsoft/azure-devops-rust-api/compare/0.1.0...0.1.1
[0.1.0]: https://github.com/microsoft/azure-devops-rust-api/tree/0.1.0
