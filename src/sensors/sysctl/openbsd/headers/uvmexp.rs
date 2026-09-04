//! The sensors::sysctl::openbsd::uvmexp module contains the constants and
//! structures used in OpenBSD used to access vm.* in sysctl.
//! See /usr/include/uvm/uvmexp.h

use crate::sensors::sysctl::openbsd::{CTLTYPE_INT, CTLTYPE_NODE, CTLTYPE_STRING, CTLTYPE_STRUCT};
use libc::{c_int, c_void, size_t};

// CTL_VM identifiers

pub const VM_METER: c_int = 1;
/// Struct loadavg
pub const VM_LOADAVG: c_int = 2;
/// PSSTRINGS
pub const VM_PSSTRINGS: c_int = 3;
/// Struct uvmexp
pub const VM_UVMEXP: c_int = 4;
/// int
pub const VM_SWAPENCRYPT: c_int = 5;
/// int: number of kmem_map pages
pub const VM_NKMEMPAGES: c_int = 6;
pub const VM_ANONMIN: c_int = 7;
pub const VM_VTEXTMIN: c_int = 8;
pub const VM_VNODEMIN: c_int = 9;
pub const VM_MAXSLP: c_int = 10;
pub const VM_USPACE: c_int = 11;
/// Config for userland malloc
pub const VM_MALLOC_CONF: c_int = 12;
/// Number of valid vm ids
pub const VM_MAXID: c_int = 13;

pub const CTL_VM_NAMES: [(&str, c_int); VM_MAXID as size_t] = [
    ("", 0),
    ("vmmeter", CTLTYPE_STRUCT),
    ("loadavg", CTLTYPE_STRUCT),
    ("psstrings", CTLTYPE_STRUCT),
    ("uvmexp", CTLTYPE_STRUCT),
    ("swapencrypt", CTLTYPE_NODE),
    ("nkmempages", CTLTYPE_INT),
    ("anonmin", CTLTYPE_INT),
    ("vtextmin", CTLTYPE_INT),
    ("vnodemin", CTLTYPE_INT),
    ("maxslp", CTLTYPE_INT),
    ("uspace", CTLTYPE_INT),
    ("malloc_conf", CTLTYPE_STRING),
];

/// Uvmemp: global data tructures that are exported to parts of the kernel
/// other than the vm system.
///
/// Locks used to protect struct members in this file:
///   a    atomic operations (signed int, so use atomic_load_sing)
///   I    immutable after creation
///   K    kernel lock
///   F    uvm_lock_fpageq
///   L    uvm_locks_pageq
///   S    uvm_swap_data_lock
///   p    copy of per-CPU counters, used only by userland.
///   o    updated only by page daemon
#[derive(Clone, Copy)]
#[repr(C)]
pub struct uvmexp {
    // vm_page constants
    /// Size of a page (PAGE_SIZE): must be a power of 2
    pub pagesize: c_int,
    /// Page mask
    pub pagemask: c_int,
    /// Page shift
    pub pageshift: c_int,

    // vm_page counters
    /// [I] number of pages we manage
    pub npages: c_int,
    /// [aF] number of free pages
    pub free: c_int,
    /// [aL] number of active pages
    pub active: c_int,
    /// [aL] number of pages we free'd but may want back
    pub inactive: c_int,
    /// [a] number of pages in the process of being paged out
    pub paging: c_int,
    /// [a] number of wired pages
    pub wired: c_int,
    /// [aF] number of zero's pages
    pub zeropages: c_int,
    /// [I] number of pages reserved for pagedaemon
    pub reserve_pagedaemon: c_int,
    /// [I] number of pages reserved for kernel
    pub reserve_kernel: c_int,
    /// [a] number of pages used by vnode page cache
    pub percpucaches: c_int,
    /// Number of pages used by vnode page cache
    pub vnodepages: c_int,
    /// Number of pages used by vtext vnodes
    pub vtextpages: c_int,

    // pageout params
    /// [I] min number of free pages
    pub freemin: c_int,
    /// [I] target number of free pages
    pub freetarg: c_int,
    /// Target number of inactive pages
    pub inactart: c_int,
    /// [I] max number of wired pages
    pub wiredmax: c_int,
    /// Min treeshold for anon pages
    pub anonmin: c_int,
    /// Min threshold for vtext pages
    pub vtextmin: c_int,
    /// Min threshold for vnode pages
    pub vnodemin: c_int,
    /// Min percent anon pages
    pub anonminpct: c_int,
    /// Min percent vtext pages
    pub vtextminpct: c_int,
    /// Min percent vnode pages
    pub vnodeminpct: c_int,

    // swap
    /// [aS] number of configured swap devices in system
    pub nswapdev: c_int,
    /// [aS] number of PAGE_SIZE'd swap pages
    pub swpages: c_int,
    ///[aS] number os swap pages in use
    pub swpginuse: c_int,
    /// [a] number of swap page in use, not also in RAM
    pub swpgonly: c_int,
    /// [a] number of swap pages moved from disk to RAM
    pub nswget: c_int,
    /// number total of anon's in system
    pub nanon: c_int,
    /// Formerly nanonneeded
    pub unused05: c_int,
    /// Formerly nfreeanon
    pub unused06: c_int,

    // stat counters
    /// [p] page fault count
    pub faults: c_int,
    /// [a] trap count
    pub traps: c_int,
    /// [a] interrupt count
    pub intrs: c_int,
    /// context switch count
    pub swtch: c_int,
    /// [a] software interrupt count
    pub softs: c_int,
    /// [a] system calls
    pub syscalls: c_int,
    /// [p] pagein operatin count, pageouts are in pdpageouts below
    pub pageins: c_int,
    /// [a] number of pagealloc from per-CPU cache
    pub pcphit: c_int,
    /// [a] number of times a per-CPU cache was empty
    pub pcpmiss: c_int,
    /// Pages swapped in
    pub pgswapin: c_int,
    /// [a] pages swapped out
    pub pgswapout: c_int,
    /// Forks
    pub forks: c_int,
    /// Forks where parent waits
    pub forks_ppwait: c_int,
    /// Forks where vmspace is shared
    pub forks_sharevm: c_int,
    /// [a] pagealloc where zero wanted and zero was available
    pub pga_zerohit: c_int,
    /// [a] pagealloc where zero zanted and zero not available
    pub pga_zeromiss: c_int,
    /// Formerly zeroaborts
    pub unused09: c_int,

    // fault subcounters
    /// [p] number of times fault was out of RAM
    pub fltnoram: c_int,
    /// [p] number of times fault was out of anons
    pub fltnoanon: c_int,
    /// [p] number of times fault was out of amap chunks
    pub fltnoamap: c_int,
    /// [p] number of times fault had to wait on a page
    pub fltpgwait: c_int,
    /// [p] number of times fault found a released page
    pub fltpgrele: c_int,
    /// [p] number of times fault relock is a success
    pub fltrelck: c_int,
    /// [p] number of times fault relock failed
    pub fltnorelck: c_int,
    /// [p] number of times fault gets anon page
    pub fltanget: c_int,
    /// [p] number of times fault retries an anon get
    pub fltanretry: c_int,
    /// [p] number of times fault clears "needs copy"
    pub fltamcopy: c_int,
    /// [p] number of times fault maps a neighbor anon page
    pub fltnamap: c_int,
    /// [p] number of times fault pams a neighbor obj page
    pub fltnomap: c_int,
    /// [p] number of times fault does a locked pgo_get
    pub fltlget: c_int,
    /// [p] number of times fault does an unlocked get
    pub fltget: c_int,
    /// [p] number of times fault anon (case 1a)
    pub flt_anon: c_int,
    /// [p] number of times fault anon cow (case 1b)
    pub flt_acow: c_int,
    /// [p] number of times fault is on object page (2a)
    pub flt_obj: c_int,
    /// [p] number of times fault promotes with copy (2b)
    pub flt_prcopy: c_int,
    /// [p] number of times fault promotes with zerofill (2b)
    pub flt_przero: c_int,
    /// [p] number of times fault upgrade is a success
    pub fltup: c_int,
    /// [p] number of times fault upgrade failed
    pub fltnoup: c_int,

    // daemon counters
    /// [ao] number of times daemon woke up
    pub pdwoke: c_int,
    /// [ao] number of times daemon scanned for free pages
    pub pdrevs: c_int,
    /// [o] number of times daemon called for swapout
    pub pdswout: c_int,
    /// [ao] number of times daemon freed since boot
    pub pdfreed: c_int,
    /// [ao] number of pages daemon scanned since boot
    pub pdscans: c_int,
    /// [ao] number of anonymous pages canned by daemon
    pub pdanscan: c_int,
    /// [ao] number of object pages scanned by daemon
    pub pdobscan: c_int,
    /// [ao] number of pages daemon reactivated since boot
    pub pdreact: c_int,
    /// [ao] number of times daemon found a busy pages
    pub pdbusy: c_int,
    /// [ao] number of times started a pageout
    pub pdpageouts: c_int,
    /// [ao] number of daemon got a pending pageout
    pub pdpending: c_int,
    /// [ao] number of pages daemon deactivated
    pub pddeact: c_int,
    /// [ao] number of pages delaye because swap crypt busy
    pub swpskip: c_int,

    /// [a] FPU context switches
    pub fpswtch: c_int,
    /// [a] number of kernel map entries
    pub kmapent: c_int,
}

#[repr(C)]
pub struct ps_strings {
    pub val: *mut c_void,
}

// Match the uvm_exp_counters enum
pub const UVM_EXP_COUNTERS_FAULTS: c_int = 0;
pub const UVM_EXP_COUNTERS_PAGEINS: c_int = 1;
pub const UVM_EXP_COUNTERS_NORAM: c_int = 2;
pub const UVM_EXP_COUNTERS_NOANON: c_int = 3;
pub const UVM_EXP_COUNTERS_NOAMAP: c_int = 4;
pub const UVM_EXP_COUNTERS_PGWAIT: c_int = 5;
pub const UVM_EXP_COUNTERS_RELE: c_int = 6;
pub const UVM_EXP_COUNTERS_RELCK: c_int = 7;
pub const UVM_EXP_COUNTERS_NORELCK: c_int = 8;
pub const UVM_EXP_COUNTERS_ANGET: c_int = 9;
pub const UVM_EXP_COUNTERS_ANRETRY: c_int = 10;
pub const UVM_EXP_COUNTERS_AMCOPY: c_int = 11;
pub const UVM_EXP_COUNTERS_NAMAP: c_int = 12;
pub const UVM_EXP_COUNTERS_NOMAP: c_int = 13;
pub const UVM_EXP_COUNTERS_LGET: c_int = 14;
pub const UVM_EXP_COUNTERS_GET: c_int = 15;
pub const UVM_EXP_COUNTERS_ANON: c_int = 16;
pub const UVM_EXP_COUNTERS_ACOW: c_int = 17;
pub const UVM_EXP_COUNTERS_OBJ: c_int = 18;
pub const UVM_EXP_COUNTERS_PRCOPY: c_int = 19;
pub const UVM_EXP_COUNTERS_PRZERO: c_int = 20;
pub const UVM_EXP_COUNTERS_UP: c_int = 21;
pub const UVM_EXP_COUNTERS_NOUP: c_int = 22;

