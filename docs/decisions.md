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

## OpenBSD: apm vs sysctl

At first I implemented [apm][openbsd-apm] to read battery information on OpenBSD.
But since apm is available only for x86 and amd64 architectures, someone using
OpenBSD on a Raspberry Pi wouldn't be able to use the battery block. So I decided
to remove apm and use sysctl instead.  
Now the battery block works on all architectures where OpenBSD works.

## OpenBSD: what is used or available memory

It's not straightforward to say exactly how much memory is used.
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





[openbsd-apm]: https://man.openbsd.org/man4/amd64/apm.4
[openbsd-uvm-struct]: https://github.com/openbsd/src/blob/master/sys/uvm/uvmexp.h#L53
