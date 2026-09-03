# Features

## Security

It's planned to implement security features provided by operating systems.  

i3status-again needs to read only a few files: config.toml,
/sys/class/power_supply/ on Linux, etc. It doesn't need to
create or execute files, nor to access the Internet.  
So, it makes sense to implement [pledge(2)][openbsd-pledge] and
[unveil(2)][openbsd-unveil] on OpenBSD, and similar mechanisms on other
operating systems. If it's as easy to use as pledge and unveil, there is
no reason for a package maintainer to apply custom patches, security
should be built-in.

## Blocks

An i3 status bar is built around the concept of blocks. A block to display
the date and time, another block to display the CPU temperature, and so on.  
Below are described the blocks implemented in i3status-again and how they
can be used.

See the support table in the [README.md](../README.md) to be sure a module works
on your operating system.

For a demo configuration file, see [config.toml](./config.toml).

### Battery

The battery block displays information about the battery. Here is its configuration:

```toml
[[blocks]]
block = "battery"
format = "{chr_state} {rem_percent}% {rem_time}"
#index = 0
```

- `block` contains the name of the block, here "battery".
- `format` describes how to display the information about the battery. Some people
  want only the percentage, some others want the remaining time, all can choose.
  There are three placeholders that can be used:
  - `{chr_state}` means charging state. It's "CHR" when the battery is charging,
	"BAT" otherwise.
  - `{rem_percent}` means remaining percentage. It is the remaining capacity of
	the battery, without the percent sign.
  - `{rem_time}` means remaining time. It's the time before the battery becomes
	full or empty, depending on if the battery is charging or discharging. It's
	formatted "HH:mm", like	"03:42". Note that this value depends on the power
	rate (how many watts are consumed). If the value is 0, which happens on one
	battery when a machine has several of them, the remaining time is an empty
	string.
- `index` is the index of the battery to monitor. Remove or comment this setting
  if you have only one battery or if you have several that you want to combine
  to display in one block. If you want to have one block for each of your
  batteries, define several "battery" blocks, one with index = 0, the other one
  with index = 1 and so on.

### Datetime

The datetime block shows the current date and time. Here is the configuration:

```toml
[[blocks]]
block = "datetime"
format = "%Y-%m-%d %H:%M:%S"
```

- `block` contains the name of the block, here "datetime"
- `format` describes how to display the date and time. For the full list of
  specifiers, see the [chrono documentation][chrono-format].



[openbsd-pledge]: https://man.openbsd.org/pledge.2
[openbsd-unveil]: https://man.openbsd.org/unveil.2
[chrono-format]: https://docs.rs/chrono/latest/chrono/format/strftime/index.html
