# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.17] - 2024-05-22

### Fixed
- Client pool getter unaligned offset

## [0.2.16] - 2024-05-20

### Fixed
- Client account size checking

## [0.2.15] - 2024-05-20

### Added
- Serialization for BasketData constructor

### Removed
- Unnecessary github action `tagging_process`

## [0.2.14] - 2024-05-20

### Added
- Serialization for BasketData

## [0.2.13] - 2024-05-20

### Added
- `[ERROR]` and `[INFO]` indication in `send_transaction`

## [0.2.12] - 2024-05-20

### Changed
- Public Default impls for client account structures

## [0.2.11] - 2024-05-20

### Changed
- Split vanilla_memo array to BasketData array in ClientPool

## [0.2.10] - 2024-05-17

### Changed
- Pool finalization price params will be calculated by the SDK 

## [0.2.9] - 2024-05-17

### Changed
- Use finalized blockhash

## [0.2.8] - 2024-05-17

### Changed
- Refactored readers for better readability

## [0.2.7] - 2024-05-16

### Changed
- Renamed Error: PoolActiveCannotFinalize to InactivePoolCannotFinalize
- Updated Error Message: "Cannot finalize pool as it is still active (worker is active)" to "Cannot finalize pool as it is not active"

## [0.2.6] - 2024-05-16

### Added
- Debug+Display realization for WorkerState

## [0.2.5] - 2024-05-14

### Added
- Action for publishing to crates.io

## [0.2.4] - 2024-05-14

### Changed
- Project description

## [0.2.3] - 2024-05-14

### Added
- Readme

## [0.2.2] - 2024-05-14

### Fixed
- Client reader tests

## [0.2.1] - 2024-05-07

### Added
- Serialization for Worker

### Changed
- Start pool instruction doesn't need a price distribution

## [0.2.0] - 2024-05-06

### Changed
- Switched over to async approach
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
