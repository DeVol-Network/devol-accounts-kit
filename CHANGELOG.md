# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.4] - 2024-05-06

### Changed
- Start next pool params constructor now receive f32 array as pool bounds params instead of i32
- DvlClient receive Pubkey instead of &str

## [0.1.3] - 2024-05-03

### Fixed

- Incorrect hex error codes

## [0.1.2] - 2024-05-03

### Updated

- Minimal serde version

## [0.1.1] - 2024-05-03

### Added

- Serialization for some common structures

## [0.1.0] - 2024-04-30

### Added

- CI pipeline

## [0.0.14] - 2024-04-29

### Added

- Payoff Instruction

### Changed

- Added instructions versions checking (on-chain)

## [0.0.13] - 2024-04-29

### Changed

- User don't need to pass his private key anymore, just sign function

