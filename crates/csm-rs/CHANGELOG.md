# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.44.0](https://github.com/aescoubas/manta-project/compare/csm-rs-v0.43.23...csm-rs-v0.44.0) - 2025-08-22

### Added

- [**breaking**] function to create a BOS session was not showing the BOS session name in logs::info, however, we do show this information when creating a BOS sessiontemplate, this behaviour make command 'apply sat-file' counter intuitive and this patch address this by making sure we log the BOS session name upon creation
- improve error handling
- move interactive code in functionality to delete and cancel CFS sessions to higher levels
- filter BOS by group name
- overwrite cfs configuration

### Fixed

- improve error management
- enum name
- BOS boot_set.rootfs_provider value was hardcoded to 'cpss3' and this is incompatible with iSCSI. This fix sets boot_set.rootfs_provider in bos to the same value user specifies in the SAT file
- BOS boot_set.rootfs_provider value was hardcoded to 'cpss3' and this is incompatible with iSCSI. This fix sets boot_set.rootfs_provider in bos to the same value user specifies in the SAT file
- remove interactive features in function apply_session
- manta apply sat-file is not validating the CFS configuration properly and it overwrites existing CFS configurations, this is not dessired since we lose the link between the CFS configuration and the image it built

### Other

- test
- test pipeline
- Release csm-rs version 0.8.0
- migrate from json Value to struct
- Release csm-rs version 0.7.1
- command 'apply sat-file --dryrun' was missrepresenting the image data when creating a session template, this patch fixes this by making sure we reuse the relevant information in the sat file so we command shows a better representation of what is going to be created
- Release csm-rs version 0.7.0
- clean code
- Release csm-rs version 0.6.1
- update Cargo.toml
- Release csm-rs version 0.6.0
- clean code
- refactor code to reduce the number of memory allocations
- clean code
- format code
- clean code
- Release csm-rs version 0.5.0-beta.19
- update Cargo.toml
- clean code
- clean code
- clean code
- Release csm-rs version 0.5.0-beta.18
- Release csm-rs version 0.5.0-beta.17
- Release csm-rs version 0.5.0-beta.16
- update Cargo.toml
- Release csm-rs version 0.5.0-beta.15
- update Cargo.toml
- Release csm-rs version 0.5.0-beta.14
- Release csm-rs version 0.5.0-beta.13
- update Cargo.toml
- Release csm-rs version 0.5.0-beta.12
