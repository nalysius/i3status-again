# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- A block "cpu_temp" for OpenBSD, that displays the temperature of the CPU in
  Celsius or Fahrenheit.
- Documentation in the [./docs](./docs) directory to explain the choices that
  have been made, the project architecture, and describe how the blocks work.

## [0.1.0] - 2026-09-03

### Added

- A block "battery" for OpenBSD, that displays the battery level, state and
  remaining time. Works with sysctl.
- A block "datetime" for all OS, that displays date and/or time using
  [chrono][chrono].
- An architecture that should make easy to port i3status-again to another
  OS (modules "blocks", "os" and "sensors").



[chrono]: https://docs.rs/chrono/latest/chrono/
