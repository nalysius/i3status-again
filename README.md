#  i3status again

i3status-again is a try to create a replacement for i3status in Rust,
that is portable on several operating systems.

i3status-rust already exists and is nice, but unfortunately it's
designed only for Linux and doesn't work on OpenBSD.

The first target is OpenBSD, but it's designed to be easy to port on
other operating systems. I'll probably port it myself on Linux and other
BSDs.

## Install

For the moment there is no package ready to use, it can only be installed
manually. That is:

1. Downloading or cloning the repository, like
   `git clone https://github.com/nalysius/i3status-again`.
2. Compiling the programme with `cargo build --release` (you need Rust
   and Cargo installed).
3. Updating your i3 configuration file so `status_command` contains
   `path/to/i3status-again/target/release/i3status-again path/to/i3status-again-config.toml`.

## Configuration

An example configuration file lives in `docs/config.toml`, copy and edit it.

## License

This project is licensed under the ISC.  
You can find the full license text in LICENSE.txt
