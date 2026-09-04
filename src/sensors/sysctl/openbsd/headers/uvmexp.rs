//! The sensors::sysctl::openbsd::uvmexp module contains the constants and
//! structures used in OpenBSD used to access vm.* in sysctl.
//! See /usr/include/uvm/uvmexp.h

use crate::sensors::sysctl::openbsd::{CTLTYPE_NODE, CTLTYPE_INT, CTLTYPE_STRING, CTLTYPE_STRUCT};
use libc::{c_int, c_void};

// CTL_VM identifiers

pub const VM_METER: i32 = 1;
/// Struct loadavg
pub const VM_LOADAVG: i32 = 2;
/// PSSTRINGS
pub const VM_PSSTRINGS: i32 = 3;
/// Struct uvmexp
pub const VM_UVMEXP: i32 = 4;
/// int
pub const VM_SWAPENCRYPT: i32 = 5;
/// int: number of kmem_map pages
pub const VM_NKMEMPAGES: i32 = 6;
pub const VM_ANONMIN: i32 = 7;
pub const VM_VTEXTMIN: i32 = 8;
pub const VM_VNODEMIN: i32 = 9;
pub const VM_MAXSLP: i32 = 10;
pub const VM_USPACE: i32 = 11;
/// Config for userland malloc
pub const VM_MALLOC_CONF: i32 = 12;
/// Number of valid vm ids
pub const VM_MAXID: i32 = 13;

pub const CTL_VM_NAMES: [(&str, i32); 13] = [
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
	pub val: *mut c_void
}

#[repr(C)]
pub enum uvm_exp_counters {
	// stat counters
	/// Page fault count
	faults = 0,
	/// Pageing operation count
	pageins,

	// Fault subcounters
	/// Number of times fault was out of RAM
	flt_noram,
	/// Number of times fault was out of anons
	flt_noanon,
	/// Number of times fault was out of amap chunks
	flt_noamap,
	/// Number of times fault had to wait on a page
	flt_pgwait,
	/// Number of times fault found a release page
	flt_rele,
	/// Number of times fault relock is a success
	flt_relck,
	/// Number of times fault relock failed
	flt_norelck,
	/// Number of times fault gets anon page
	flt_anget,
	/// Number of times fault retries on anon get
	flt_anretry,
	/// Number of times fault clears "needs copy"
	flt_amcopy,
	/// Number of times fault maps a neighbor anon page
	flt_namap,
	/// Number of times fault maps to a neighbor obj page
	flt_nomap,
	/// Number of times fault does a locked pgo_get
	flt_lget,
	/// Number of times fault does un unlocked get
	flt_get,
	/// Number of times fault anon (case 1a)
	flt_anon,
	/// Number of times fault anon cow (case 1b)
	flt_acow,
	/// Number of times fault is on object page (2a)
	flt_obj,
	/// Number of times fault promotes with copy (2b)
	flt_prcopy,
	/// Number of times fault promotes with zerofill (2b)
	flt_przero,
	/// Number of times fault upgrade is a success
	flt_up,
	/// Number of times fault upgrade failed
	flt_noup,

	exp_ncounters
}
