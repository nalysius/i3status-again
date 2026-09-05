# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/2.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- An `interval` option in the configuration to let the user decide how often the
  bar updates its information. Default to 1 second.
- A block "cpu_freq" for OpenBSD, that displays the frequency of the CPU in MHz
  or GHz. If several CPUs are combined in one block, it's possible to display the
  average or maximum frequency.
- A block "cpu_temp" for OpenBSD, that displays the temperature of the CPU in
  Celsius or Fahrenheit.
- Documentation in the [./docs](./docs) directory to explain the choices that
  have been made, the project architecture, and describe how the blocks work.

### Changed

- The ports of the OpenBSD C headers sensors.h, sysctl.h and uvmexp.g to Rust
  have been improved to follow the libc guidelines.

## [0.1.0] - 2026-09-03

### Added

- A block "battery" for OpenBSD, that displays the battery level, state and
  remaining time. Works with sysctl.
- A block "datetime" for all OS, that displays date and/or time using
  [chrono][chrono].
- An architecture that should make easy to port i3status-again to another
  OS (modules "blocks", "os" and "sensors").



[chrono]: https://docs.rs/chrono/latest/chrono/
