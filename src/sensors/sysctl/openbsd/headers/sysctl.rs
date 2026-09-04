//! The sensors::sysctl::openbsd::sysctl module contains the constants and
//! structures used in OpenBSD to access sysctl.
//! See /usr/include/sys/sysctl.h

use libc::{c_char, c_int, c_ulong, size_t};

/// Largest number of components supported
pub const CTL_MAX_NAME: c_int = 12;

/// Each subsystem defined by sysctl defines a list of variables
/// for that subsystem. Each name is either a node with further
/// levels defined below it, or it is a leaf of some particular
/// type given below. Each sysctl level defines a set of name/type
/// pairs to be used by sysctl(1) in manipulating the subsystem.
#[repr(C)]
pub struct ctlname {
    /// Subsystem name
    ctl_name: &'static str,
    /// Type of name
    ctl_type: c_int,
}

/// Name is a node
pub const CTLTYPE_NODE: c_int = 1;
/// Name describes an integer
pub const CTLTYPE_INT: c_int = 2;
/// Name describes a string
pub const CTLTYPE_STRING: c_int = 3;
/// Name describes a 64-bit number
pub const CTLTYPE_QUAD: c_int = 4;
/// Name describes a structure
pub const CTLTYPE_STRUCT: c_int = 5;

/// Unused
pub const CTL_UNSPEC: c_int = 0;
/// "High kernel": proc, limits
pub const CTL_KERN: c_int = 1;
/// Virtual memory
pub const CTL_VM: c_int = 2;
// No 3, gap for CTL_FS
/// Network, see /usr/include/sys/socket.h
pub const CTL_NET: c_int = 4;
/// Debugging parameters
pub const CTL_DEBUG: c_int = 5;
/// Generic cpu/io
pub const CTL_HW: c_int = 6;
/// Machine dependent
pub const CTL_MACHDEP: c_int = 7;
// No 8, was CTL_USER, which is removed
/// DDB user interface, see /usr/include/ddb/db_var.h
pub const CTL_DDB: c_int = 9;
/// VFS sysctl's
pub const CTL_VFS: c_int = 10;
/// Number of valid top-level ids
pub const CTL_MAXID: c_int = 11;

/// Map the CTL names to their types
pub const CTL_NAMES: [(&str, c_int); CTL_MAXID as size_t] = [
    ("", 0),
    ("kern", CTLTYPE_NODE),
    ("vm", CTLTYPE_NODE),
    ("gap", 0),
    ("net", CTLTYPE_NODE),
    ("debug", CTLTYPE_NODE),
    ("hw", CTLTYPE_NODE),
    ("machdep", CTLTYPE_NODE),
    ("gap", 0),
    ("ddb", CTLTYPE_NODE),
    ("vfs", CTLTYPE_NODE),
];

// CTL_KERN identifier
/// String: system version
pub const KERN_OSTYPE: c_int = 1;
/// String: system release
pub const KERN_OSRELEASE: c_int = 2;
/// Int: system revision
pub const KERN_OSREV: c_int = 3;
/// String: compile time info
pub const KERN_VERSION: c_int = 4;
/// Int: max vnodes
pub const KERN_MAXVNODES: c_int = 5;
/// Int: max processes
pub const KERN_MAXPROC: c_int = 6;
/// Int: max open files
pub const KERN_MAXFILES: c_int = 7;
/// Int: max arguments to exec
pub const KERN_ARGMAX: c_int = 8;
/// Int: system security level
pub const KERN_SECURELVL: c_int = 9;
/// String: hostname
pub const KERN_HOSTNAME: c_int = 10;
/// Int: host identifier
pub const KERN_HOSTID: c_int = 11;
/// Stuct: struct clockinfo
pub const KERN_CLOCKRATE: c_int = 12;
// No 13, 14 or 15. Were KERN_DNSJACKPORT, KERN_PROC and KERN_FILE
/// Node: kernel profiling info
pub const KERN_PROF: c_int = 16;
/// Int: POSIX.1 version
pub const KERN_POSIX1: c_int = 17;
/// Int: ~ of supplemental group ids
pub const KERN_NGROUPS: c_int = 18;
/// Int: is job control available
pub const KERN_JOB_CONTROL: c_int = 19;
/// Int: saved set-user/group-ID
pub const KERN_SAVED_IDS: c_int = 20;
/// Struct: time kernel was booted
pub const KERN_BOOTTIME: c_int = 21;
/// String: (YP) domain name
pub const KERN_DOMAINNAME: c_int = 22;
/// Int: number of partitions / disk
pub const KERN_MAXPARTITIONS: c_int = 23;
/// Int: raw partition number
pub const KERN_RAWPARTITION: c_int = 24;
/// Int: max threads
pub const KERN_MAXTHREAD: c_int = 25;
/// Int: number of threads
pub const KERN_NTHREADS: c_int = 26;
/// String: kernel build version
pub const KERN_OSVERSION: c_int = 27;
/// Int: listen queue maximum
pub const KERN_SOMAXCONN: c_int = 28;
/// Int: half-open controllable param
pub const KERN_SOMINCONN: c_int = 29;
// No 30 or 31, were KERN_USERMOUNT and KERN_RND
/// Int: no setuid coredumps ever
pub const KERN_NOSUIDCOREDUMP: c_int = 32;
/// Int: file synchronization support
pub const KERN_FSYNC: c_int = 33;
/// Int: SysV message queue support
pub const KERN_SYSVMSG: c_int = 34;
/// Int: SysV semaphore support
pub const KERN_SYSVSEM: c_int = 35;
/// Int: SysV shared memory support
pub const KERN_SYSVSHM: c_int = 36;
// No 37, was KERN_ARND
/// Int: size of message buffer
pub const KERN_MSGBUFSIZE: c_int = 38;
/// Note: malloc statistics
pub const KERN_MALLOCSTATS: c_int = 39;
/// Array: cp_time
pub const KERN_CPTIME: c_int = 40;
/// Struct: vfs cache statistics
pub const KERN_NCHSTATS: c_int = 41;
/// Struct: fork statistics
pub const KERN_FORKSTAT: c_int = 42;
// No 43, was KERN_NSELCOLL
/// Node: tty information
pub const KERN_TTY: c_int = 44;
/// Int: ccpu
pub const KERN_CCPU: c_int = 45;
/// Int: fscale
pub const KERN_FSCALE: c_int = 46;
/// Int: number of processes
pub const KERN_NPROCS: c_int = 47;
/// Message buffer, KERN_MSGBUFSIZE
pub const KERN_MSGBUF: c_int = 48;
/// Struct: pool information
pub const KERN_POOL: c_int = 49;
/// Int: stackgap_random
pub const KERN_STACKGAPRANDOM: c_int = 50;
/// Struct: SysV sem/shm/msg info
pub const KERN_SYSVIPC_INFO: c_int = 51;
/// Int: allowkmem
pub const KERN_ALLOWKMEM: c_int = 52;
/// Int: witnesswatch
pub const KERN_WITNESSWATCH: c_int = 53;
/// Int: splassert
pub const KERN_SPLASSERT: c_int = 54;
/// Node: proc args and env
pub const KERN_PROC_ARGS: c_int = 55;
/// Int: number of open files
pub const KERN_NFILES: c_int = 56;
/// Int: number of tty devices
pub const KERN_TTYCOUNT: c_int = 57;
/// Int: number of vnodes in use
pub const KERN_NUMVNODES: c_int = 58;
/// Struct: mbuf statistics
pub const KERN_MBSTAT: c_int = 59;
/// Node: witness
pub const KERN_WITNESS: c_int = 60;
/// Struct: SysV struct seminfo
pub const KERN_SEMINFO: c_int = 61;
/// Struct: SysV struct shminfo
pub const KERN_SHMINFO: c_int = 62;
/// Node: interrupt counters
pub const KERN_INTRCNT: c_int = 63;
/// Node: watchdog
pub const KERN_WATCHDOG: c_int = 64;
/// Int: allowdt
pub const KERN_ALLOWDT: c_int = 65;
/// Struct: process entries
pub const KERN_PROC: c_int = 66;
/// Number of mclusters
pub const KERN_MAXCLUSTERS: c_int = 67;
/// Node: event counters
pub const KERN_EVCOUNT: c_int = 68;
/// Node: timecounter
pub const KERN_TIMECOUNTER: c_int = 69;
/// Int: locks per uid
pub const KERN_MAXLOCKSPERUID: c_int = 70;
/// Array: cp_time2
pub const KERN_CPTIME2: c_int = 71;
/// Buffer cache % of physmem
pub const KERN_CACHEPCT: c_int = 72;
/// Struct: file entries
pub const KERN_FILE: c_int = 73;
/// Int: w^x sigabrt & core
pub const KERN_WXABORT: c_int = 74;
/// Dev_t console terminal device
pub const KERN_CONSDEV: c_int = 75;
/// Int: Int: number of network livelocks
pub const KERN_NETLIVELOCKS: c_int = 76;
/// Int: enable pool_debug
pub const KERN_POOL_DEBUG: c_int = 77;
/// Node: proc cwd
pub const KERN_PROC_CWD: c_int = 78;
/// Node: proc no broadcast kill
pub const KERN_PROC_NOBROADCASTKILL: c_int = 79;
/// Node: proc vmmap
pub const KERN_PROC_VMMAP: c_int = 80;
/// Allow ptrace globally
pub const KERN_GLOBAL_PTRACE: c_int = 81;
/// Int: console message buffer size
pub const KERN_CONSBUFSIZE: c_int = 82;
/// Console message buffer
pub const KERN_CONSBUF: c_int = 83;
/// Console message buffer
pub const KERN_AUDIO: c_int = 84;
/// Struct: audio properties
pub const KERN_CPUSTATS: c_int = 85;
/// Struct: pf status and stats
pub const KERN_PFSTATUS: c_int = 86;
/// Struct: timeout status and stats
pub const KERN_TIMEOUT_STATS: c_int = 87;
/// Int: adjust RTC time to UTC
pub const KERN_UTC_OFFSET: c_int = 88;
/// Int: Struct: video properties
pub const KERN_VIDEO: c_int = 89;
/// Node: clockintr
pub const KERN_CLOCKINTR: c_int = 90;
/// Int: kernel device tree state serial
pub const KERN_AUTOCONF_SERIAL: c_int = 91;
/// Number of valid kern ids
pub const KERN_MAXID: c_int = 92;

/// Map CTL kern to their types
pub const CTL_KERN_NAMES: [(&str, c_int); KERN_MAXID as size_t] = [
    ("", 0),
    ("ostype", CTLTYPE_STRING),
    ("osrelease", CTLTYPE_STRING),
    ("osrevision", CTLTYPE_INT),
    ("version", CTLTYPE_STRING),
    ("maxvnodes", CTLTYPE_INT),
    ("maxproc", CTLTYPE_INT),
    ("maxfiles", CTLTYPE_INT),
    ("argmax", CTLTYPE_INT),
    ("securelevel", CTLTYPE_INT),
    ("hostname", CTLTYPE_STRING),
    ("hostid", CTLTYPE_INT),
    ("clockrate", CTLTYPE_STRUCT),
    ("gap", 0),
    ("gap", 0),
    ("gap", 0),
    ("profiling", CTLTYPE_NODE),
    ("posix1version", CTLTYPE_INT),
    ("ngroups", CTLTYPE_INT),
    ("job_control", CTLTYPE_INT),
    ("saved_ids", CTLTYPE_INT),
    ("boottime", CTLTYPE_STRUCT),
    ("domainname", CTLTYPE_STRING),
    ("maxpartitions", CTLTYPE_INT),
    ("rawpartition", CTLTYPE_INT),
    ("maxthread", CTLTYPE_INT),
    ("nthreads", CTLTYPE_INT),
    ("osversion", CTLTYPE_STRING),
    ("somaxconn", CTLTYPE_INT),
    ("sominconn", CTLTYPE_INT),
    ("gap", 0),
    ("gap", 0),
    ("nosuidcoredump", CTLTYPE_INT),
    ("fsync", CTLTYPE_INT),
    ("sysvmsg", CTLTYPE_INT),
    ("sysvsem", CTLTYPE_INT),
    ("sysvshm", CTLTYPE_INT),
    ("gap", 0),
    ("msgbufsize", CTLTYPE_INT),
    ("malloc", CTLTYPE_NODE),
    ("cp_time", CTLTYPE_STRUCT),
    ("nchstats", CTLTYPE_STRUCT),
    ("forkstat", CTLTYPE_STRUCT),
    ("gap", 0),
    ("tty", CTLTYPE_NODE),
    ("ccpu", CTLTYPE_INT),
    ("fscale", CTLTYPE_INT),
    ("nprocs", CTLTYPE_INT),
    ("msgbuf", CTLTYPE_STRUCT),
    ("pool", CTLTYPE_NODE),
    ("stackgap_random", CTLTYPE_INT),
    ("sysvipc_info", CTLTYPE_INT),
    ("allowkmem", CTLTYPE_INT),
    ("witnesswatch", CTLTYPE_INT),
    ("splassert", CTLTYPE_INT),
    ("procargs", CTLTYPE_NODE),
    ("nfiles", CTLTYPE_INT),
    ("ttycount", CTLTYPE_INT),
    ("numvnodes", CTLTYPE_INT),
    ("mbstat", CTLTYPE_STRUCT),
    ("witness", CTLTYPE_NODE),
    ("seminfo", CTLTYPE_STRUCT),
    ("shminfo", CTLTYPE_STRUCT),
    ("intrcnt", CTLTYPE_NODE),
    ("watchdog", CTLTYPE_NODE),
    ("allowdt", CTLTYPE_INT),
    ("proc", CTLTYPE_STRUCT),
    ("maxclusters", CTLTYPE_INT),
    ("evcount", CTLTYPE_NODE),
    ("timecounter", CTLTYPE_NODE),
    ("maxlocksperuid", CTLTYPE_INT),
    ("cp_time2", CTLTYPE_STRUCT),
    ("bufcachepercent", CTLTYPE_INT),
    ("file", CTLTYPE_STRUCT),
    ("wxabort", CTLTYPE_INT),
    ("consdev", CTLTYPE_STRUCT),
    ("netlivelocks", CTLTYPE_INT),
    ("pool_debug", CTLTYPE_INT),
    ("proc_cwd", CTLTYPE_NODE),
    ("proc_nobroadcastkill", CTLTYPE_NODE),
    ("proc_vmmap", CTLTYPE_NODE),
    ("global_ptrace", CTLTYPE_INT),
    ("consbufsize", CTLTYPE_INT),
    ("consbuf", CTLTYPE_STRUCT),
    ("audio", CTLTYPE_STRUCT),
    ("cpustats", CTLTYPE_STRUCT),
    ("pfstatus", CTLTYPE_STRUCT),
    ("timeout_stats", CTLTYPE_STRUCT),
    ("utc_offset", CTLTYPE_INT),
    ("video", CTLTYPE_STRUCT),
    ("clockintr", CTLTYPE_NODE),
    ("autoconf_serial", CTLTYPE_INT),
];

// KERN_PROC subtypes
/// Everything but kernel threads
pub const KERN_PROC_ALL: c_int = 0;
/// By process id
pub const KERN_PROC_PID: c_int = 1;
/// By process group id
pub const KERN_PROC_PGRP: c_int = 2;
/// By session of pid
pub const KERN_PROC_SESSION: c_int = 3;
/// By controlling tty
pub const KERN_PROC_TTY: c_int = 4;
/// By effective uid
pub const KERN_PROC_UID: c_int = 5;
/// By real uid
pub const KERN_PROC_RUID: c_int = 6;
/// Also return kernel threads
pub const KERN_PROC_KTHREAD: c_int = 7;
/// Also return normal threads
pub const KERN_PROC_SHOW_THREADS: c_int = 0x40000000;

// KERN_SYSVIPC_INFO subtypes
/// Msginfo and msgid_ds
pub const KERN_SYSVIPC_MSG_INFO: c_int = 1;
/// Seminfo and semid_ds
pub const KERN_SYSVIPC_SEM_INFO: c_int = 2;
/// Shminfo and shmid_ds
pub const KERN_SYSVIPC_SHM_INFO: c_int = 3;

// KERN_PROC_ARGS subtypes
pub const KERN_PROC_ARGV: c_int = 1;
pub const KERN_PROC_NARGV: c_int = 2;
pub const KERN_PROC_ENV: c_int = 3;
pub const KERN_PROC_NENV: c_int = 4;

// KERN_AUDIO
pub const KERN_AUDIO_RECORD: c_int = 1;
pub const KERN_AUDIO_KBDCONTROL: c_int = 2;
pub const KERN_AUDIO_MAXID: c_int = 3;

/// Map the kern audio names to their types
pub const CTL_KERN_AUDIO_NAMES: [(&str, c_int); KERN_AUDIO_MAXID as size_t] = [
    ("", 0),
    ("record", CTLTYPE_INT),
    ("kbdcontrol", CTLTYPE_INT),
];

// KERN_VIDEO
pub const KERN_VIDEO_RECORD: c_int = 1;
pub const KERN_VIDEO_MAXID: c_int = 2;

/// Map the kern video names to their types
pub const CTL_KERN_VIDEO_NAMES: [(&str, c_int); KERN_VIDEO_MAXID as size_t] = [("", 0), ("record", CTLTYPE_INT)];

// KERN_WITNESS
pub const KERN_WITNESS_WATCH: c_int = 1;
pub const KERN_WITNESS_LOCKTRACE: c_int = 2;
pub const KERN_WITNESS_MAXID: c_int = 3;

/// Map their kern witness names to their types
pub const CTL_KERN_WITNESS_NAMES: [(&str, c_int); KERN_WITNESS_MAXID as size_t] =
    [("", 0), ("watch", CTLTYPE_INT), ("locktrace", CTLTYPE_INT)];

/*
 * KERN_PROC subtype ops return arrays of relatively fixed size
 * structures of process info.   Use 8 byte alignment, and new
 * elements should only be added to the end of this structure so
 * binary compatibility can be preserved.
 */
pub const KI_NGROUPS: c_int = 16;
/// Includes NUL. From /usr/include/sys/syslimits.h:83
pub const KI_MAXCOMLEN: c_int = 24;
pub const KI_WMESGLEN: c_int = 8;
pub const KI_MAXLOGNAME: c_int = 32;
pub const KI_EMULNAMELEN: c_int = 8;

pub const KI_NOCPU: u64 = u64::MAX;

/// Controlling tty vnode active
pub const EPROC_CTTY: c_int = 0x01;
/// Session leader
pub const EPROC_SLEADER: c_int = 0x02;
/// Has unveil settings
pub const EPROC_UNVEIL: c_int = 0x04;
/// Unveil is locked
pub const EPROC_LKUNVAIL: c_int = 0x08;

/// See /usr/include/sys/sysctl.h
#[allow(non_snake_case)]
pub struct kinfo_proc {
    /// PTR: linked run/sleep queue
    pub p_forw: u64,
    /// PTR: linked run/sleep queue
    pub p_back: u64,
    /// PTR: address of proc
    pub p_paddr: u64,
    /// PTR: Kernel virtual addr or u-area
    pub p_addr: u64,
    /// PTR: Ptr to open files structure
    pub p_fd: u64,
    /// PTR: unused, always zero
    pub p_stats: u64,
    /// PTR: Process limits
    pub p_limit: u64,
    /// PTR: Address space
    pub p_vmspace: u64,
    /// PTR: Signal actions, state
    pub p_sigacts: u64,
    /// PTR: session pointer
    pub p_sess: u64,
    /// PTR: tty session pointer
    pub p_tsess: u64,
    /// PTR: Exit information
    pub p_ru: u64,
    /// LONG: extra kinfo_proc flags
    pub p_eflag: i32,
    /// Unused, always zero
    pub p_exitsig: i32,
    /// INT: P_* flags
    pub p_flag: i32,
    /// PID_T: Process identifier
    pub p_pid: i32,
    /// PID_T: Parent process id
    pub p_ppid: i32,
    /// PID_T: session id
    pub p_sid: i32,
    /// PID_T: process group id. sys/proc.h hijacks p_pgid
    pub p__pgid: i32,
    /// PID_T: tty process group id
    pub p_tpgid: i32,
    /// UID_T: effective user id
    pub p_uid: u32,
    /// UID_T: real user id
    pub p_ruid: u32,
    /// GID_T: effective group id
    pub p_gid: u32,
    /// GID_T: real group id
    pub p_rgid: u32,
    /// GID_T: groups
    pub p_groups: [u32; KI_NGROUPS as size_t],
    /// SHORT: number of groups
    pub p_ngroups: i16,
    /// SHORT: job control counter
    pub p_jobc: i16,
    /// DEV_T: controlling tty dev
    pub p_tdev: u32,
    ///U_INT: Time averaged value pf p_cpticks
    pub p_estcpu: u32,
    /// STRUCT TIMEVAL: real time
    pub p_rtime_sec: u32,
    /// STRUCT TIMEVAL: real time
    pub p_rtime_usec: u32,
    /// INT: Ticks of cpu time
    pub p_cpticks: i32,
    /// FIXPT_T: %cpu for this process
    pub p_pctcpu: u32,
    /// Unused, always zero
    pub p_swtime: u32,
    /// U_INT: Time since last blocked
    pub p_slptime: u32,
    /// INT: PSCHED_* flags
    pub p_schedflags: i32,
    /// U_QUAD_T: Stat clock hits in user mode
    pub p_uticks: u64,
    /// U_QUAD_T: Stat clock hits in system mode
    pub p_sticks: u64,
    /// U_QUAD_T: Stat clock hits processing ints
    pub p_iticks: u64,
    /// PTR: Trace to vnode or file
    pub p_ptracep: u64,
    /// INT: Kernel trace points
    pub p_traceflag: i32,
    /// INT: If non-zero: don't swap
    pub p_holdcnt: i32,
    /// INT: Signals arrived but not delivered
    pub p_siglist: i32,
    /// SIGSET_T:  Current signal mask
    pub p_sigmask: u32,
    /// SIGSET_T: Signals being ignored
    pub p_sigignore: u32,
    /// SIGSET_T: Signals being caught by user
    pub p_sigcatch: u32,
    /// CHAR: S* process status (from LWP)
    pub p_stat: i8,
    /// U_CHAR: Process priority
    pub p_priority: u8,
    /// U_CHAR: User-priority based on p_estcpu and ps_nice
    pub p_usrpri: u8,
    /// U_CHAR: Process "nice" value
    pub p_nice: u8,
    /// U_SHORT: Exit status for wait; also stop signal
    pub p_xstat: u16,
    /// U_SHORT: unused
    pub p_spare: u16,
    pub p_comm: [u8; KI_MAXLOGNAME as size_t],
    /// wchan message
    pub p_wmesg: [u8; KI_WMESGLEN as size_t],
    /// PTR: sleep address
    pub p_wchan: u64,
    /// setlogin() name
    pub p_login: [u8; KI_MAXLOGNAME as size_t],
    /// SEGSZ_T: current resident set size in pages
    pub p_vm_rssize: i32,
    /// SEGSZ_T: text size (pages)
    pub p_vm_tsize: i32,
    /// SEGSZ_T: data size (pages)
    pub p_vm_dsize: i32,
    /// SEGSZ_T: stack size (pages)
    pub p_vm_ssize: i32,
    /// CHAR: following p_u* members from struct user are valid. 64-bits for alignment
    pub p_uvalid: i64,
    /// STRUCT TIMEVAL: starting time
    pub p_ustart_sec: u64,
    /// STRUCT TIMEVAL: starting time
    pub p_ustart_usec: u32,
    /// STRUCT TIMEVAL: user time
    pub p_uutime_sec: u32,
    /// STRUCT TIMEVAL: user  time
    pub p_uutime_usec: u32,
    /// STRUCT TIMEVAL: system time
    pub p_ustime_sec: u32,
    /// STRUCT TIMEVAL: system time
    pub p_ustime_usec: u32,
    /// LONG: max resident set size
    pub p_uru_maxrss: u64,
    /// LONG: integral shared memory size
    pub p_uru_ixrss: u64,
    /// LONG: integral unshared data
    pub p_uru_idrss: u64,
    /// LONG: integral unshared stack
    pub p_uru_isrrs: u64,
    /// LONG: page reclaims
    pub p_uru_minflt: u64,
    /// LONG: page faults
    pub p_uru_majflt: u64,
    /// LONG: swaps
    pub p_uru_nswap: u64,
    /// LONG: block input operations
    pub p_uru_inblock: u64,
    /// LONG: block output operations
    pub p_uru_oublock: u64,
    /// LONG: messages sent
    pub p_uru_msgsnd: u64,
    /// LONG: messages received
    pub p_uru_msgrcv: u64,
    /// LONG: signals received
    pub p_uru_nsignals: u64,
    /// LONG: voluntary context switches
    pub p_uru_nvcsw: u64,
    /// LONG: involuntary context switches
    pub p_uru_nivcsw: u64,
    /// STRUCT TIMEVAL: child u+s time
    pub p_uctime_sec: u32,
    /// STRUCT TIMEVAL: child u+s time
    pub p_uctime_usec: u32,
    /// UINT: PS_* flags on the process
    pub p_psflags: u32,
    /// UINT: Accounting flags
    pub p_acflag: u32,
    /// UID_T: saved user id
    pub p_svuid: u32,
    /// GID_T: saved group id
    pub p_svgid: u32,
    /// syscall emulation name
    pub p_emul: [u8; KI_EMULNAMELEN as size_t],
    /// RLIM_T: soft limit for rss
    pub p_rlim_rss_cur: u64,
    /// LONG: CPU id
    pub p_cpuid: u64,
    /// VSIZE_T: virtual size
    pub p_vm_map_size: u64,
    /// PID_T: Thread identifier
    pub p_tid: i32,
    /// U_INT: Routing table identifier
    pub p_rtableid: u32,
    /// U_INT64_T: Pledge flags
    pub p_pledge: u64,
    /// Thread name
    pub p_name: [u8; KI_MAXCOMLEN as size_t],
}

/// VM address range entry, matching struct vm_map_entry.  Useful for
/// debuggers to know process's addresses.
///
/// To iterate entries, set the last kve_end as the base address into
/// kve_start.
pub struct kinfo_vmentry {
    /// vaddr_t
    pub kve_start: c_ulong,
    /// vaddr_t
    pub kve_end: c_ulong,
    /// vsize_t
    pub kve_guard: c_ulong,
    /// vsize_t
    pub kve_fspace: c_ulong,
    /// vsize_t
    pub kve_fspace_augment: c_ulong,
    /// voff_t
    pub kve_offset: u64,
    pub kve_wired_count: c_int,
    pub kve_etype: c_int,
    pub kve_protection: c_int,
    pub kve_max_protection: c_int,
    pub kve_advice: c_int,
    pub kve_inheritance: c_int,
    pub kve_flags: u8,
}

// keep in sync with UVM_ET_*
pub const KVE_ET_OBJ: c_int = 0x00000001;
pub const KVE_ET_SUBMAP: c_int = 0x00000002;
pub const KVE_ET_COPYONWRITE: c_int = 0x00000004;
pub const KVE_ET_NEEDSCOPY: c_int = 0x00000008;
pub const KVE_ET_HOLE: c_int = 0x00000010;
pub const KVE_ET_NOFAULT: c_int = 0x00000020;
pub const KVE_ET_STACK: c_int = 0x00000040;
pub const KVE_ET_WC: c_int = 0x00000080;
pub const KVE_ET_CONCEAL: c_int = 0x00000100;
pub const KVE_ET_SYSCALL: c_int = 0x00000200;
pub const KVE_ET_FREEMAPPED: c_int = 0x00000800;

pub const KVE_PROT_NONE: c_int = 0x00000000;
pub const KVE_PROT_READ: c_int = 0x00000001;
pub const KVE_PROT_WRITE: c_int = 0x00000002;
pub const KVE_PROT_EXEC: c_int = 0x00000004;

pub const KVE_ADV_NORMAL: c_int = 0x00000000;
pub const KVE_ADV_RANDOM: c_int = 0x00000001;
pub const KVE_ADV_SEQUENTIAL: c_int = 0x00000002;

pub const KVE_INH_SHARE: c_int = 0x00000000;
pub const KVE_INH_COPY: c_int = 0x00000010;
pub const KVE_INH_NONE: c_int = 0x00000020;
pub const KVE_INH_ZERO: c_int = 0x00000030;

pub const KVE_F_STATIC: c_int = 0x01;
pub const KVE_F_KMEM: c_int = 0x02;

/*
 * kern.file returns an array of these structures, which are designed
 * both to be immune to 32/64 bit emulation issues and to
 * provide backwards compatibility.  The order differs slightly from
 * that of the real struct file, and some fields are taken from other
 * structures (struct vnode, struct proc) in order to make the file
 * information more useful.
 */
pub const KERN_FILE_BYFILE: c_int = 1;
pub const KERN_FILE_BYPID: c_int = 2;
pub const KERN_FILE_BYUID: c_int = 3;
pub const KERN_FILESLOP: c_int = 10;

pub const KERN_FILE_TEXT: c_int = -1;
pub const KERN_FILE_CDIR: c_int = -2;
pub const KERN_FILE_RDIR: c_int = -3;
pub const KERN_FILE_TRACE: c_int = -4;

/// Rounded up from 90
pub const KI_MNAMELEN: c_int = 96;
pub const KI_UNPPATHLEN: c_int = 104;

pub struct kinfo_file {
    /// PTR: address of struct file
    pub f_fileaddr: u64,
    /// UINT: flags (see fcntl.h)
    pub f_flag: u32,
    /// UINT: internal flags
    pub f_iflags: u32,
    /// INT: descriptor
    pub f_type: u32,
    /// UINT: reference count
    pub f_count: u32,
    /// UINT: references from msg queue
    pub f_msgcount: u32,
    /// INT: number active users
    pub f_usecount: u32,
    /// PTR: creds for descriptors
    pub f_ucred: u64,
    /// UID_T: descriptor credentials
    pub f_uid: u32,
    /// GID_T: descriptor credentials
    pub f_gid: u32,
    /// PTR: address of fileops
    pub f_ops: u64,
    /// OFF_T: offset
    pub f_offset: u64,
    /// PTR: descriptor data
    pub f_data: u64,
    /// UINT64: number of read transfers
    pub f_rxfer: u64,
    /// UINT64: number of write transfers
    pub f_rwfer: u64,
    /// UINT64: number of seek operations
    pub f_seek: u64,
    /// UINT64: total bytes read
    pub f_rbytes: u64,
    /// UINT64: total bytes written
    pub f_wbytes: u64,
    /// PTR: socket, specinfo, etc
    pub v_un: u64,
    /// ENUM: vnode type
    pub v_type: u32,
    /// ENUM: type of underlying data
    pub v_tag: u32,
    /// UINT: vnode flags
    pub v_flag: u32,
    /// DEV_T raw device
    pub va_rdev: u32,
    /// PTR: private data for fs
    pub v_data: u64,
    /// PTR: mount info for fs
    pub v_mount: u64,
    /// LONG: file id
    pub va_fileid: u64,
    /// UINT64_T: file size in bytes
    pub va_size: u64,
    /// MODE_T: file access mode and type
    pub va_mode: u32,
    /// DEV_T: filesystem device
    pub va_fsid: u32,
    pub f_mntonname: [u8; KI_MNAMELEN as size_t],
    /// SHORT: socket type
    pub so_type: u32,
    /// SHORT: socket state
    pub so_state: u32,
    /// PTR: socket pcb. For non-root, -1 if not NULL
    pub so_pcb: u64,
    /// SHORT: socket protocol type
    pub so_protocol: u32,
    /// INT: socket domain family
    pub so_family: u32,
    /// PTR: pointer to per-protocol pcb
    pub inp_ppcb: u64,
    /// SHORT: local inet port
    pub inp_lport: u32,
    /// STRUCT: local inet addr
    pub inp_laddru: [u32; 4],
    /// SHORT: foreign inet port
    pub inp_fport: u32,
    /// STRUCT: foreign inet addr
    pub inp_faddru: [u32; 4],
    /// PTR: connected socket cntrl block
    pub inp_conn: u64,
    /// PTR: link with other direction
    pub pipe_peer: u64,
    /// UINT: pipe status info
    pub pipe_state: u32,
    /// INT: number of pending events
    pub kq_count: u32,
    /// INT: kqueue status information
    pub kq_state: u32,
    /// Unused
    pub __unused1: u32,
    /// PID_T: process id
    pub p_pid: u32,
    /// INT: descriptor number
    pub fd_fd: i32,
    /// CHAR: open file flags
    pub fd_ofileflags: u32,
    /// UID_T: process credentials
    pub p_uid: u32,
    /// GID_T: process credentials
    pub p_gid: u32,
    /// PID_T: thread id
    pub p_tid: u32,
    pub p_comm: [u8; KI_MAXCOMLEN as size_t],
    /// UINT: Routing table identifier
    pub inp_rtableid: u32,
    /// PTR: f_data of spliced socket
    pub so_splice: u64,
    /// OFF_T: already spliced count or -1 ir this is target of splice
    pub so_splicelen: i64,
    /// LONG: chars in receive buf
    pub so_rcv_cc: u64,
    /// LONG: chars in send buf
    pub so_snd_cc: u64,
    /// PTR: CONNECTED SOCKETS
    pub unp_refs: u64,
    /// PTR: link to next connected socket
    pub unp_nextref: u64,
    /// PTR: address ps the socket address
    pub unp_addr: u64,
    pub unp_path: [u8; KI_UNPPATHLEN as size_t],
    /// CHAR: raw protocol id
    pub inp_proto: u32,
    /// SHORT: tcp state
    pub t_state: u32,
    /// ULONG: tcp receive window
    pub t_rcv_wnd: u64,
    /// ULONG: tcp send window
    pub t_snd_wnd: u64,
    /// ULONG: tcp congestion-controlled window
    pub t_snd_cwnd: u64,
    /// NLINK_T: number of references to file
    pub va_nlink: u32,
}

// KERN_INTRCNT
/// Int: # intrcnt
pub const KERN_INTRCNT_NUM: c_int = 1;
/// Node: intrcnt
pub const KERN_INTRCNT_CNT: c_int = 2;
/// Node: names
pub const KERN_INTRCNT_NAME: c_int = 3;
/// Node: interrupt vector
pub const KERN_INTRCNT_VECTOR: c_int = 4;
pub const KERN_INTRCNT_MAXID: c_int = 5;

/// Map the CTL kern intrcnt names to their types
/// See /usr/include/sys/sysctl:851
pub const CTL_KERN_INTRCNT_NAMES: [(&str, c_int); 4] = [
    ("", 0),
    ("nintrcnt", CTLTYPE_INT),
    ("intrcnt", CTLTYPE_NODE),
    ("intrname", CTLTYPE_NODE),
];

// KERN_WATCHDOG
pub const KERN_WATCHDOG_PERIOD: c_int = 1;
pub const KERN_WATCHDOG_AUTO: c_int = 2;
pub const KERN_WATCHDOG_MAXID: c_int = 3;

pub const CTL_KERN_WATCHDOG_NAMES: [(&str, c_int); KERN_WATCHDOG_MAXID as size_t] =
    [("", 0), ("period", CTLTYPE_INT), ("auto", CTLTYPE_INT)];

// KERN_TIMECOUNTER

/// Int: number of revolutions
pub const KERN_TIMECOUNTER_TICK: c_int = 1;
/// Int: log a warning when time change
pub const KERN_TIMECOUNTER_TIMESTEPWARNINGS: c_int = 2;
/// String: tick hardware used
pub const KERN_TIMECOUNTER_HARDWARE: c_int = 3;
/// String: tick hardware used
pub const KERN_TIMECOUNTER_CHOICE: c_int = 4;
pub const KERN_TIMECOUNTER_MAXID: c_int = 5;

/// Map the CTL kern timetounter names to their types
pub const CTL_KERN_TIMECOUNTER_NAMES: [(&str, c_int); KERN_TIMECOUNTER_MAXID as size_t] = [
    ("", 0),
    ("tick", CTLTYPE_INT),
    ("timestepwarnings", CTLTYPE_INT),
    ("hardware", CTLTYPE_STRING),
    ("choice", CTLTYPE_STRING),
];

// KERN_CLOCKINTR
/// Struct: stats
pub const KERN_CLOCKINTR_STATS: c_int = 1;
pub const KERN_CLOCKINTR_MAXID: c_int = 2;

/// Map the CTL kern clockintr names to their types
pub const CTL_KERN_CLOCKINTR_NAMES: [(&str, c_int); KERN_CLOCKINTR_MAXID as size_t] = [("", 0), ("stats", CTLTYPE_STRUCT)];

// CTL_HW identifiers
/// String: machine class
pub const HW_MACHINE: c_int = 1;
/// String: specific machine model
pub const HW_MODEL: c_int = 2;
/// Int: number of configured cpus
pub const HW_NCPU: c_int = 3;
/// Int: machine byte order
pub const HW_BYTEORDER: c_int = 4;
/// Int: total memory
pub const HW_PHYSMEM: c_int = 5;
/// Int: non-kernel memory
pub const HW_USERMEM: c_int = 6;
/// Int: software page size
pub const HW_PAGESIZE: c_int = 7;
/// Strings: disk drive names
pub const HW_DISKNAMES: c_int = 8;
/// Struct: diskstats[]
pub const HW_DISKSTATS: c_int = 9;
/// Int: number of disks
pub const HW_DISKCOUNT: c_int = 10;
/// Node: hardware monitors
pub const HW_SENSORS: c_int = 11;
/// Get CPU frequency
pub const HW_CPUSPEED: c_int = 12;
/// Set CPU performance
pub const HW_SETPERF: c_int = 13;
/// String: vendor name
pub const HW_VENDOR: c_int = 14;
/// String: product name
pub const HW_PRODUCT: c_int = 15;
/// String: hardware version
pub const HW_VERSION: c_int = 16;
/// String: hardware serial number
pub const HW_SERIALNO: c_int = 17;
/// String: universal unique id
pub const HW_UUID: c_int = 18;
/// Quad: total memory
pub const HW_PHYSMEM64: c_int = 19;
/// Quad: non-kernel memory
pub const HW_USERMEM64: c_int = 20;
/// Int: number of cpus found
pub const HW_NCPUFOUND: c_int = 21;
/// Allow power button shutdown
pub const HW_ALLOWPOWERDOWN: c_int = 22;
/// Set performance policy
pub const HW_PERFPOLICY: c_int = 23;
/// Int: enable SMT/HT/CMT
pub const HW_SMT: c_int = 24;
/// Int: number of cpus being used
pub const HW_NCPUONLINE: c_int = 25;
/// Int: machine has wall-power
pub const HW_POWER: c_int = 26;
/// Node: battery
pub const HW_BATTERY: c_int = 27;
/// Strings: ucom names
pub const HW_UCOMNAMES: c_int = 28;
/// String: cpu types to block
pub const HW_BLOCKCPU: c_int = 29;
/// Number of valid hw ids
pub const HW_MAXID: c_int = 30;

/// Map the CTL hw names to their type
/// See /usr/include/sys/sysctl.h:933
pub const CTL_HW_NAMES: [(&str, c_int); HW_MAXID as size_t] = [
    ("", 0),
    ("machine", CTLTYPE_STRING),
    ("model", CTLTYPE_STRING),
    ("ncpu", CTLTYPE_INT),
    ("byteorder", CTLTYPE_INT),
    ("gap", 0),
    ("gap", 0),
    ("pagesize", CTLTYPE_INT),
    ("disknames", CTLTYPE_STRING),
    ("diskstats", CTLTYPE_STRUCT),
    ("diskcount", CTLTYPE_INT),
    ("sensors", CTLTYPE_NODE),
    ("cpuspeed", CTLTYPE_INT),
    ("setperf", CTLTYPE_INT),
    ("vendor", CTLTYPE_STRING),
    ("product", CTLTYPE_STRING),
    ("version", CTLTYPE_STRING),
    ("serialno", CTLTYPE_STRING),
    ("uuid", CTLTYPE_STRING),
    ("physmem", CTLTYPE_QUAD),
    ("usermem", CTLTYPE_QUAD),
    ("ncpufound", CTLTYPE_INT),
    ("allowpowerdown", CTLTYPE_INT),
    ("perfpolicy", CTLTYPE_STRING),
    ("smt", CTLTYPE_INT),
    ("ncpuonline", CTLTYPE_INT),
    ("power", CTLTYPE_INT),
    ("battery", CTLTYPE_NODE),
    ("ucomnames", CTLTYPE_STRING),
    ("blockcpu", CTLTYPE_STRING),
];

// HW_BATTERY
/// Int: battery charging mode
pub const HW_BATTERY_CHARGEMODE: c_int = 1;
/// Int: battery start charge percent
pub const HW_BATTERY_CHARGESTART: c_int = 2;
/// Int: battery stop charge percent
pub const HW_BATTERY_CHARGESTOP: c_int = 3;
pub const HW_BATTERY_MAXID: c_int = 4;

/// Mat the CTL hw battery names to their types
pub const CTL_HW_BATTERY_NAMES: [(&str, c_int); HW_BATTERY_MAXID as size_t] = [
    ("", 0),
    ("chargemode", CTLTYPE_INT),
    ("chargestart", CTLTYPE_INT),
    ("chargestop", CTLTYPE_INT),
];

/*
 * CTL_DEBUG definitions
 *
 * Second level identifier specified which debug variale.
 * Third level identifier specifies which structure component.
 */
/// String: variable name
pub const CTL_DEBUG_NAME: c_int = 1;
/// Int: variale value
pub const CTL_DEBUG_VALUE: c_int = 2;
pub const CTL_DEBUG_MAXID: c_int = 20;

/// CTL_DEBUG variables.
///
/// These are declared as separate variables so that they can be
/// individually initialized at the location of their associated
/// variable. The loader prevents multiple use by issuing errors
/// if a variable is initialized in more than one place. They are
/// aggregated into an array in debug_sysctl(), so that it can
/// conveniently locate them when queried. If more debugging
/// variables are added, they must also be declared here and also
/// entered into the array.
pub struct ctldebug {
    /// Name of debugging variable
    pub debugname: *mut c_char,
    /// Pointer to debugging variable
    pub debugvar: *mut c_int,
}

/// Exported sysctl variable with valid bounds. Both bounds are inclusive to
/// allow full range of values.
pub struct sysctl_bounded_args {
    /// identifier shared with userspace as a CTL_ constant
    pub mib: c_int,
    /// Never NULL
    pub var: *mut c_int,
    /// Checking is disabled if minimum == maximum
    pub minimum: c_int,
    /// Read-only variable if minimum > maximum
    pub maximum: c_int,
}

/// Special case minimum,maximum marker for sysctl_bounded_args.
pub const SYSCTL_INT_READONLY: (c_int, c_int) = (1, 0);
