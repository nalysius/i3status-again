# Decisions

This document describes the choices made in this project.

## A portable program

As an OpenBSD user, it's annoying to see software supporting only Linux.
An i3 status bar should work everywhere i3 works. I understand the difficulty
to access hardware information on every operating system, especially in Rust
where we can't just "#include <sys/sensors.h>" but have to port some C code
to Rust.  
However, I think the program should at least be easily portable. The
[architecture](./architecture.md) should help to have a portable program.

## OpenBSD: apm vs sysctl

At first I implemented [apm][openbsd-apm] to read battery information on OpenBSD.
But since apm is available only for x86 and amd64 architectures, someone using
OpenBSD on a Raspberry Pi wouldn't be able to use the battery block. So I decided
to remove apm and use sysctl instead.  
Now the battery block works on all architectures where OpenBSD works.

## OpenBSD: what is used or available memory

It's not straightforward to say exactly how much memory is used. Firstly, one
has to decide between measuring the total memory (hw.physmem) or the
user-available memory (hw.usermem) only. I consider the memory used by the kernel
as mine too, I want the global picture so I choose to measure the total memory.

To access memory information on OpenBSD, sysctl reads a
[uvmexp][openbsd-uvm-struct] struct, which has fields like:

- `pagesize`: the size of a page in bytes
- `npages`: the total number of pages
- `free`: the number of free pages
- `active`: the number of active pages
- `inactive`: the number of inactive pages
- `wired`: the number of wired pages (reserved by the kernel or `mlock(2)`ed)
- `vnodepages`: the number of pages caching file contents
- `vtextpages`: the number of pages caching executable text (subset of
  vnodepages)

What is certain is that the _active_ and _wired_ pages are not available, so they
can be counted as used memory. Likewise, _free_ and _inactive_ are available or
claimable, so they can be counted as available memory.  

What about _vnodepages_ and _vtextpages_? They count cached pages, but a part of
the cache is used, whereas another part isn't.  
My understanding is that when I open a file in Emacs:

1. The Emacs binary is stored in memory and counted in _vtextpages_ (and thus
   in _vnodepages_).
2. The file content is stored twice and counted both in _vnodepages_ (because
   the operating system read it) and in _active_ (because it's in Emacs' buffer).
3. Additional memory used by Emacs during its execution is counted in _active_.

While the file is open, the pages stay in cache because I use them.
When I close the file and quit Emacs, the _active_ pages used by Emacs are freed,
while the _vnodepages_ and _vtextpages_ stay there, in case
I run Emacs or read the file again. The difference is that now, these cached
pages are no longer used.  
The issue is that the uvmexp struct doesn't tell us how much of the cache is
used. So there are only two choices: consider the whole cache as free or as
used, and neither is perfect.

If we consider the cache as used memory, most files that have been opened since
the machine's boot are in cache, even if they have been closed. On a 32GiB
machine using OpenBSD and i3 with just a Web browser and a few terminal emulators
open, 22GiB were used by the cache. In practice most of these 22GiB were
probably not in active use, so it wouldn't make sense to say to a user that only
10GiB of RAM are available in this situation.

On the other hand if we consider the cache as available, when the user opens a
2GiB file in Emacs, 2GiB will be stored in _vnodepages_ (in addition to the
_active_) and will effectively be used, but will be counted as available
anyway, which isn't true either.

I consider the second option closer to the truth, so I count the cache
as available. Here are the calculations:

```
total_memory = npages  * pagesize
used_memory = (active + wired) * pagesize
```

It means that _active_ and _wired_ are counted as used memory, and everything
else is counted as available.

## Why implement C headers in the project?

Since i3status-again doesn't call external programs, it needs to communicate
with the operating system using libc. It's done in the `sensors` module.
In Rust, the libc crate contains many FFI definitions to use the libc of any
operating systems. It doesn't contain everything for every OS, so some FFI
definitions must be declared manually.  
Implementing them manually is good to understand how the operating system works,
and it's faster than sending everything to the libc crate and wait until it's
merged and published. Sending pull requests to the libc crate is a long-term
goal, but implementing the missing headers in the project lets me move faster.

The C headers ported to Rust try to follow the crate's conventions. Structures
and constants keep their original names and C types are used as much as
possible. We don't have the crate's `s!` and `c_enum!` macros, so the structures
derive useful traits manually and the C enumerations are ported as constants
whose names are the name of the variants.  
The pull requests to the crate have started in September 2026. When a structure
or an enumeration is added in Rust's libc, switching to it should be as simple
as updating the `use` path.

## OpenBSD: pledge and unveil

OpenBSD provides two nice features to programs, pledge(2) and unveil(2). Pledge
lets a program restrict the system operations it can do, while unveil restricts
the parts of the filesystem it can access.  

i3status-again uses pledge with the following promises: stdio rpath cpath vminfo
unveil unix.  
- `stdio` is required because the program writes to stdout.
- `rpath` is required to read the configuration file and a few other files
  related to sndio.
- `cpath` is required for sndio, which creates a file ~/.sndio/cookie.
- `vminfo` is required for the `memory` block which reads memory information.
- `unveil` is temporarily required to call unveil(2), and removed then.
- `unix` is required for sndio, to connect to a unix socket.

The following paths are allowed in unveil:
- `~/.sndio/cookie rwc` is the cookie created when connecting to sndio the first
  time.
- `/var/run/sndio rw` and `/tmp/sndio rw` are directories where the sndio socket
  can be created.
- `/etc/localtime r` is the file (symlink) containing the user's timezone, used
  for the `datetime` block.

## Notes about CPU usage

Some decisions had to be made for the module `cpu_usage`, I'll describe them here.

1. The block only provides the percentage of usage, mixing user, system and so on.
   Not all OS have the same details, for example spin isn't available everywhere.
   To keep it easily portable, only the busy time is computed. It should be enough
   for a status line, top(1) can be used if more details are needed.
2. Everything except idle is considered busy. On OpenBSD the following times are
   measured: user, nice, sys, spin, intr and idle. We could debate about whether
   spin (waiting for a lock) should count as busy, since it's not actually doing
   something, but it's not available either. To keep it simple, I decided that
   only idle counted as free, everything else counted as busy.
3. The block computes percentage of time, nothing more. If a CPU core spends
   50% if its _ticks_ in idle state, then it's 50% busy, no matter how often the
   bar refreshes, only the ticks between two refreshes are compared. This way, no
   need to know the clock rate. The kind of CPU core doesn't impact the
   computation either. A Performant core being at 90% doesn't have the same
   importance than a Lethargic core being at 90%, but this isn't taken into
   account here since the block only computes time, not load.
4. Only the online CPU cores are computed. If some cores are offline, they are
   just ignored. For example on OpenBSD SMT and Lethargic cores are disabled by
   default, so if a CPU has 3 physical cores with SMT support but SMT is disabled,
   only 3 cores will be available, while 6 would be if SMT was enabled.
5. Unless a CPU index is provided in the block configuration, all the CPUs are
   re-discovered at each refresh. Offline cores can be re-activated while the
   host is running and the core indexes aren't necessarily contiguous. So it's
   easier to test all the CPU each time.

[openbsd-apm]: https://man.openbsd.org/man4/amd64/apm.4
[openbsd-uvm-struct]: https://github.com/openbsd/src/blob/master/sys/uvm/uvmexp.h#L53
