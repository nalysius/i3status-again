#  i3status-again

i3status-again is a portable replacement for i3status, written in Rust.

i3status-rust already exists and is nice, but unfortunately it's
designed only for Linux and doesn't work on OpenBSD.

The first target is OpenBSD, but i3status-again is designed to be easy to port
on other operating systems.

![Screenshot of i3status-again](docs/imgs/screenshot.webp)

## Support

The table below shows which blocks are supported on which operating systems.

|            | OpenBSD | FreeBSD | NetBSD | DragonFly BSD | Linux |
|------------|---------|---------|--------|---------------|-------|
| battery    |   ✅    |   ❌    |   ❌   |      ❌       |   ❌  |
| cpu_freq   |   ✅    |   ❌    |   ❌   |      ❌       |   ❌  |
| cpu_temp   |   ✅    |   ❌    |   ❌   |      ❌       |   ❌  |
| datetime   |   ✅    |   ✅    |   ✅   |      ✅       |   ✅  |

✅ = supported  
❌ = not yet supported

## Features

The features, blocks and how they can be used are described in
[./docs/features.md](./docs/features.md).

## Install

### Pre-built release

Compiled versions are distributed with each [release][releases], you can just
download the one for your operating system.  
Then update your i3 configuration file so `status_command` contains
`path/to/i3status-again/target/release/i3status-again path/to/i3status-again-config.toml`.

### Build from source

You can also build the program from source.

1. Download or clone the repository, like
   `git clone https://github.com/nalysius/i3status-again`.
2. Compile the program with `cargo build --release` (you need Rust
   and Cargo installed).
3. Update your i3 configuration file so `status_command` contains
   `path/to/i3status-again/target/release/i3status-again path/to/i3status-again-config.toml`.

## Documentation

The documentation can be found in the [./docs/](docs/) directory.

### Configuration

An example configuration file lives in [./docs/config.toml](docs/config.toml),
copy and edit it.

## License

This project is licensed under the ISC license.  
You can find the full license text in LICENSE.txt


[releases]: https://github.com/nalysius/i3status-again/releases
