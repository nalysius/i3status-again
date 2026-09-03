# Decisions

This document describes the choices that have been made in this project.

## A portable program

As an OpenBSD user, it's annoying to see software supporting only Linux.
An i3 status bar should work everywhere i3 works. I understand the difficulty
to access hardware information on every operating system, especially in Rust
where we can't just "#include <sys/sensors.h>" but have to port some C code
to Rust.  
However, I think the program should at least be easily portable. The
[architecture](./architecture.md) should help to have a portable program.

## apm vs sysctl on OpenBSD

At first I implemented [apm][openbsd-apm] to read battery information on OpenBSD.
But since apm is available only for x86 and amd64 architectures, someone using
OpenBSD on a Raspberry Pi wouldn't be able to use the battery block. So I decided
to remove apm and use sysctl instead.  
Now the battery block works on all architectures where OpenBSD works.


[openbsd-apm]: https://man.openbsd.org/man4/amd64/apm.4
