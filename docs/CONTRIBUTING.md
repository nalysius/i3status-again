# Contributing to i3status-again

Thank you for investing your time in this project.  
Any contribution can be useful, whether it's fixing a typo the documentation,
creating a new block or port one to a new operating system.

## New contributor

## Contributing to the documentation

The documentation is as important as the source code, so if you spot any error
or something is missing, feel free to add it.  
If you don't feel up to creating a pull request, you can just create an issue,
describe the issue and the correction, the fix will be applied for you.

## Contributing to the code.

If it's your first time in the project, I suggest that you read the
[documentation](./) first, especially about the
[architecture of the project](./architecture.md).

Some general instructions about the source code:
1. Avoid adding crates when not necessary. It's tempting to use a crate to
   read information from sysctl or power_supply, but I prefer to reduce
   the dependency to external libraries.
2. The project must be compatible with a Rust version that is a few months old.
   It doesn't mean it cannot be compatible with the latest one, but please don't
   depend on very new features. As an example, on 2026-09-04 the latest version
   of Rust is 1.98.1, and OpenBSD 7.9 packages include 1.94.1.
3. If you add something you don't have to add it for every operating systems,
   but don't block them. By working in the right modules it should be fine.
4. The functions and structures must be commented, so at least we know what
   they do.
5. Don't forget to update [features.md](./features.md) if you add or update
   a block.

Below I'll describe the two main situations other than a bug fix that could
push to contribute to the project.

### Creating a new block

Creating a new block means having to touch a bit to everything. Here are the
steps:

1. Implementing the `src/sensors/` submodule, if not done already. Most of
   the work is there. Sometimes (e.g.: sysctl), C headers must be ported to Rust
   to define constants and structures. While it's better to contribute to the
   libc crate for this, it's usually fine to put them in i3status-again first and
   contribute to libc later. Please reduce the unsafe code to the minimum.
2. Creating the `src/os/` submodule abstraction for your block. Implement it for
   your operating system, expose the needed functions. It's important to return a
   Result in case reading the sensors fails. Each `os` submodule defines its
   error enumeration, so it's isolated from the `sensors` submodule and can be
   used in the block. Don't forget to re-export the functions with "pub use" in
   `mod.rs`, so the API is OS-independent.
3. Preparing the configuration of the new block. In `src/config.rs`, add a
   variant in the `BlockConfig` enum and create a struct for your block's
   configuration. Then in the `Config`'s `to_blocks()` method, add a match
   branch to convert your `BlockConfig` variant to a `BlockType`.
4. Preparing the ground for the new block. `BlockType` is defined in
   `src/blocks/mod.rs`, add a variant to store your future block.
5. Creating the block under `src/blocks/`. The logic is similar between most
   blocks: defining the struct, implement the `from_config()` method, implement
   `get_output()` from the `Block` trait, and that's all.
6. Document the new block and its documentation in [features.md](./features.md)
   and add it in the [README.md](../README.md) table.

### Porting a block to a new operating system

Porting a block means the configuration and block are already there, so it's
less work than creating a new one. The steps are as follow:

1. Implementing the `src/sensors/` submodule, if not done already. As an example
   if sysctl isn't implemented for NetBSD and you want to port a module to
   NetBSD, most of the work will be to implement sysctl. If you need to port
   C headers to Rust you can do it in the project, but the best would be to
   do it in the libc crate directly. Please reduce the unsafe code to the
   minimum.
2. Creating the `src/os/` submodule abstraction for your block / operating
   system pair. Implement the same functions that other operating systems for
   the block you're interested in, and don't forget to re-export them using
   "pub use" in `mod.rs`. This way the existing block will be able to use them.
3. Updating the [README.md](../README.md)'s table to show that the block now
   supports the new operating system.



