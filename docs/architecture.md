# Architecture of i3status-again

This document aims to describe the architecture of the project, making it
easier for a new contributor to start working on the project.

## Architecture description

The `src/` directory looks like this :

```
.
|-- bar.rs
|-- blocks
|   |-- battery.rs
|   |-- cpu_freq.rs
|   |-- cpu_temp.rs
|   |-- datetime.rs
|   `-- mod.rs
|-- common.rs
|-- config.rs
|-- lib.rs
|-- main.rs
|-- os
|   |-- battery
|   |   |-- mod.rs
|   |   `-- openbsd.rs
|   |-- cpu_freq
|   |   |-- mod.rs
|   |   `-- openbsd.rs
|   |-- cpu_temp
|   |   |-- mod.rs
|   |   `-- openbsd.rs
|   `-- mod.rs
`-- sensors
    |-- mod.rs
    `-- sysctl
        |-- mod.rs
        `-- openbsd
            |-- headers
            |   |-- mod.rs
            |   |-- sensors.rs
            |   `-- sysctl.rs
            `-- mod.rs
```

### `bar`

The `bar` module handles the i3bar protocol, it defines a `BlockOutput` struct
that is returned by the blocks. Then it is encoded to JSON, so the blocks don't
have to bother with it.

### `common`

The `common` module defines some structs that are common to several modules. For
example, the struct TempUnit is defined here and is used in `config`, `blocks`
and `os` to let the user decide which temperature unit to use. This module must
remain as small as possible.

### `config`

The `config` module reads the TOML configuration and defines structs to give the
configuration to the blocks.

### `sensors`

The `sensors` module defines ways to access information on each OS. For example
on OpenBSD, [sysctl(2)][openbsd-sysctl] can be used to query several information
like the battery's remaining capacity, last full capacity, rate,
but also the CPU temperature and more.

One sensors' submodule, like `sensors::sysctl`, has OS-specific implementation.  
While most Unix-like systems have sysctl (Linux removed it in version 5.5), their
usage is different. The Linux sysctl was accepting a struct instead of flat
parameters, OpenBSD sysctl can be used to get a `Sensor` as defined in
"[sys/sys/sensors.h][github-openbsd-sensor]", which could be different from
another BSD.

Since several information can be read from each sensors' submodule, implementing
one can be useful for several blocks. Sysctl is a good example of this.

### `os`

The `os` module uses what's defined in `sensors` to expose a common API for all
operating systems, this way the `blocks` module can be independent of the OS.

For example the `os::battery` module provides functions like
`get_battery_level()` that returns the current level of the battery.
In `os::battery::openbsd` it uses `sensors::sysctl::openbsd` to get the
information, in `os::battery::linux` it could use `sensors::power_supply::linux`,
but both expose the same functions.

For the `blocks` module to be OS-independent, each submodule of `os` re-exports
the functions defined for the right OS. It's done this way:

```rust
// src/os/battery/mod.rs
#[cfg(target_os = "openbsd")]
pub mod openbsd;
#[cfg(target_os = "openbsd")]
pub use crate::os::battery::openbsd::*;
```

It means that a consumer of this module only has to `use crate::os::battery`
without wondering which OS submodule to import. At compile time, only the
implementation needed for the operating system is made available in the
submodule.

### `blocks`

The `blocks` module implements the logic of a block, without the complexity of
fetching the information from the OS. A block is common between all
operating systems.

For example the `blocks::battery` module defines a `BatteryBlock` struct that
takes a `BatteryConfig` (defined in `config`) to let the user format the output
using placeholders like `{rem_percent}`, calls functions like
`get_battery_level()` to get the information, and returns a `BlockOutput` that
will be displayed to the user.


## Why this architecture

This architecture is supposed to make it easier to port the project to a new
operating system without breaking it for another one.

To port the `battery` block to Linux, a module `sensors::power_supply::linux`
will be implemented, independently of the existing code.  
Then a `os::battery::linux` module will implement the required functions,
without affecting the OpenBSD code, and re-export them in `os::battery`.  
That's all.

With such a modular architecture, it should be easy to implement OS security like
[pledge(2)][openbsd-pledge] and [unveil(2)][openbsd-unveil] in the program,
without the need for package maintainers to apply patches for this.
  

[openbsd-sysctl]: https://man.openbsd.org/sysctl.2
[github-openbsd-sensor]: https://github.com/openbsd/src/blob/master/sys/sys/sensors.h#L112
[openbsd-pledge]: https://man.openbsd.org/pledge.2
[openbsd-unveil]: https://man.openbsd.org/unveil.2
