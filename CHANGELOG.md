# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -
## [v0.6.3](https://github.com/wahl-dev/wrpt/compare/v0.6.2..v0.6.3) - 2026-01-14
#### Bug Fixes
- **(Dockerfile)** add curl to debian image and rename docker-compose binary to docker-compose - ([ca59ddd](https://github.com/wahl-dev/wrpt/commit/ca59dddb7506369782b54b33260009a2da59a19e)) - [@Wahib-L](https://github.com/Wahib-L)

- - -

## [v0.6.2](https://github.com/wahl-dev/wrpt/compare/v0.6.1..v0.6.2) - 2026-01-14
#### Bug Fixes
- **(Dockerfile)** update docker-compose version and add openssl dependency - ([fcf7d9d](https://github.com/wahl-dev/wrpt/commit/fcf7d9d00abee6a1bf916c7f44eff7f814c4ec65)) - [@Wahib-L](https://github.com/Wahib-L)

- - -

## [v0.6.1](https://github.com/wahl-dev/wrpt/compare/v0.6.0..v0.6.1) - 2025-01-14
#### Bug Fixes
- **(docker)** switch base image from distroless to debian:12-slim for better compatibility and add docker-compose binary to the image - ([93138a1](https://github.com/wahl-dev/wrpt/commit/93138a1317e2d318f040f09142b4e0eedfdc756a)) - [@Wahib-L](https://github.com/Wahib-L)

- - -

## [v0.6.0](https://github.com/wahl-dev/wrpt/compare/v0.5.2..v0.6.0) - 2025-01-06
#### Features
- add --insecure flag to skip SSL certificate verification for API requests - ([62fe521](https://github.com/wahl-dev/wrpt/commit/62fe521ab7039a54f0decfd905e41e4981f895f0)) - [@Wahib-L](https://github.com/Wahib-L)

- - -

## [v0.5.2](https://github.com/wahl-dev/wrpt/compare/v0.5.1..v0.5.2) - 2025-01-05
#### Bug Fixes
- move trigger docker workflow step to a new position - ([1e8f96c](https://github.com/wahl-dev/wrpt/commit/1e8f96ca9d5bf8dab7da7ee91a74f7102578683c)) - [@Wahib-L](https://github.com/Wahib-L)
- move trigger docker workflow step to a new position - ([553829d](https://github.com/wahl-dev/wrpt/commit/553829d19b78be1e7e821b7107936aecc7cf8d59)) - [@Wahib-L](https://github.com/Wahib-L)

- - -

## [v0.5.1](https://github.com/wahl-dev/wrpt/compare/v0.5.0..v0.5.1) - 2025-01-05
#### Bug Fixes
- update repository URLs in workflows, changelog, and Cargo.toml to reflect new organization name - ([bb5af04](https://github.com/wahl-dev/wrpt/commit/bb5af046282ad50e39abcf57653cde5ddedc3310)) - [@Wahib-L](https://github.com/Wahib-L)

- - -

## [v0.5.0](https://github.com/wahl-dev/wrpt/compare/v0.4.0..v0.5.0) - 2025-01-05
#### Documentation
- **(README.md)** add documentation for stack start and stack stop commands - ([2bf4a73](https://github.com/wahl-dev/wrpt/commit/2bf4a73340a24dca0003cb3645a9e4b9a17508b4)) - [@Wahib-L](https://github.com/Wahib-L)
- update GitHub Actions badge link to reflect new repository name - ([e5854f1](https://github.com/wahl-dev/wrpt/commit/e5854f1f300debd40a30d6c1b0e898b019bc4a2d)) - [@Wahib-L](https://github.com/Wahib-L)
#### Features
- **(stacks)** add start command - ([4b4f1f1](https://github.com/wahl-dev/wrpt/commit/4b4f1f11c241419dad21e4be80b36d3476b29438)) - [@Wahib-L](https://github.com/Wahib-L)
- **(stacks)** add stop command - ([5248667](https://github.com/wahl-dev/wrpt/commit/52486675bb22652725c22c8da6565a3dfcb31acc)) - [@Wahib-L](https://github.com/Wahib-L)

- - -

## [v0.4.0](https://github.com/wahl-dev/wrpt/compare/v0.3.0..v0.4.0) - 2025-01-01
#### Features
- **(stacks)** implement `resource_control` command to display ResourceControl details of a specific stack - ([0111228](https://github.com/wahl-dev/wrpt/commit/01112287fde95e34009fc6fe4164a83c015d646a)) - [@Wahib-L](https://github.com/Wahib-L)
#### Miscellaneous Chores
- **(workflows)** update Docker and test workflows to trigger on release events and remove branch restrictions for better flexibility in CI/CD process - ([f4d408b](https://github.com/wahl-dev/wrpt/commit/f4d408b77f5d9f48e168a508427bf071f96c921e)) - [@Wahib-L](https://github.com/Wahib-L)

- - -

## [v0.3.0](https://github.com/wahl-dev/wrpt/compare/v0.2.1..v0.3.0) - 2025-01-01
#### Documentation
- **(README.md)** update command list in README to include team and user commands - ([9b948a7](https://github.com/wahl-dev/wrpt/commit/9b948a79ef703190cc91ab37ba6c85ed39afe7dd)) - [@Wahib-L](https://github.com/Wahib-L)
- add badges to README for crate version, Docker image, and CI status to enhance visibility and provide quick access to project information - ([65226fe](https://github.com/wahl-dev/wrpt/commit/65226fee7e8f81d0452481f1a1d92fc26204d5f8)) - [@Wahib-L](https://github.com/Wahib-L)
#### Features
- **(users)** add users command with list subcommand - ([bf098d7](https://github.com/wahl-dev/wrpt/commit/bf098d702e3fabb6404a80f40b6652ffa98a5b89)) - [@Wahib-L](https://github.com/Wahib-L)
- **(teams)** add teams command with list subcommand - ([cedb20b](https://github.com/wahl-dev/wrpt/commit/cedb20bb66de6ebc54390d96031216ec562b6097)) - [@Wahib-L](https://github.com/Wahib-L)
#### Miscellaneous Chores
- **(docs)** update README to include a link to the changelog - ([de8a499](https://github.com/wahl-dev/wrpt/commit/de8a499529b293a7504bc52c2048d52595f3d569)) - [@Wahib-L](https://github.com/Wahib-L)
#### Refactoring
- **(table)** display struct Id if available and factorize value processing - ([7257877](https://github.com/wahl-dev/wrpt/commit/725787766a8cb5f9644858eba0c678dc01a2ecde)) - [@Wahib-L](https://github.com/Wahib-L)

- - -

## [v0.2.1](https://github.com/wahl-dev/wrpt/compare/v0.2.0..v0.2.1) - 2024-12-30
#### Bug Fixes
- clean up CHANGELOG by removing redundant entries - ([6f2155d](https://github.com/wahl-dev/wrpt/commit/6f2155d8eaea208d425ef37a2300b4e9ef5d64ed)) - [@Wahib-L](https://github.com/Wahib-L)

- - -

## [v0.2.0](https://github.com/wahl-dev/wrpt/compare/v0.1.0..v0.2.0) - 2024-12-30
#### Bug Fixes
- **(workflows)** fix cargo fmt test - ([5af5a14](https://github.com/wahl-dev/wrpt/commit/5af5a142de53f2edc6c701f1ba19705d2cd516cd)) - [@Wahib-L](https://github.com/Wahib-L)
- **(workflows,rust.yml)** add clippy and fmt checks for code quality - ([e6ddec2](https://github.com/wahl-dev/wrpt/commit/e6ddec25e7b6dce0f565fc9661b515e5f183d051)) - [@Wahib-L](https://github.com/Wahib-L)
#### Features
- **(cog.toml)** add configuration file for cog to manage versioning and changelog generation effectively - ([3845b97](https://github.com/wahl-dev/wrpt/commit/3845b97273b03169c4140f48b5b78648111dbcf7)) - [@Wahib-L](https://github.com/Wahib-L)
- **(release)** add GitHub Actions workflow for automated SemVer releases and changelog generation - ([4074fd4](https://github.com/wahl-dev/wrpt/commit/4074fd477161bb7981fad83fc80e7c41be0684d8)) - [@Wahib-L](https://github.com/Wahib-L)
- **(stacks)** add support for creating standalone stacks alongside swarm stacks - ([8e0abfa](https://github.com/wahl-dev/wrpt/commit/8e0abfafc76806ff0f40355dd5724b0b3958c2ca)) - [@Wahib-L](https://github.com/Wahib-L)
#### Miscellaneous Chores
- **(release.yml)** add permissions for write access to contents - ([5c2c372](https://github.com/wahl-dev/wrpt/commit/5c2c372f9f6eed79be7757b721ff07540c9b4006)) - [@Wahib-L](https://github.com/Wahib-L)
- **(tests.yml)** remove duplicate test execution step - ([3446b4c](https://github.com/wahl-dev/wrpt/commit/3446b4c7dfefb028a8732f0827767bc55577fa47)) - [@Wahib-L](https://github.com/Wahib-L)
- **(version)** v0.2.0 - ([4009bb3](https://github.com/wahl-dev/wrpt/commit/4009bb366d6e3773161a29311a9bd3906369e17a)) - github-actions
#### Style
- format code for improved readability and consistency across files - ([d29f19d](https://github.com/wahl-dev/wrpt/commit/d29f19d64709997246ab4034330c460656624849)) - [@Wahib-L](https://github.com/Wahib-L)
