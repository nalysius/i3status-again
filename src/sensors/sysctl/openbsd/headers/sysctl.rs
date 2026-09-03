//! The sensors::sysctl::openbsd::sysctl module contains the constants and
//! structures used in OpenBSD to access sysctl.
//! See /usr/include/sys/sysctl.h

/// Largest number of components supported
pub const CTL_MAX_NAME: i32 = 12;

/// Each subsystem defined by sysctl defines a list of variables
/// for that subsystem. Each name is either a node with further
/// levels defined below it, or it is a leaf of some particular
/// type given below. Each sysctl level defines a set of name/type
/// pairs to be used by sysctl(1) in manipulating the subsystem.
#[repr(C)]
pub struct CtlName {
    /// Subsystem name
    ctl_name: &'static str,
    /// Type of name
    ctl_type: i32,
}

/// Name is a node
pub const CTLTYPE_NODE: i32 = 1;
/// Name describes an integer
pub const CTLTYPE_INT: i32 = 2;
/// Name describes a string
pub const CTLTYPE_STRING: i32 = 3;
/// Name describes a 64-bit number
pub const CTLTYPE_QUAD: i32 = 4;
/// Name describes a structure
pub const CTLTYPE_STRUCT: i32 = 5;

/// Unused
pub const CTL_UNSPEC: i32 = 0;
/// "High kernel": proc, limits
pub const CTL_KERN: i32 = 1;
/// Virtual memory
pub const CTL_VM: i32 = 2;
// No 3, gap for CTL_FS
/// Network, see /usr/include/sys/socket.h
pub const CTL_NET: i32 = 4;
/// Debugging parameters
pub const CTL_DEBUG: i32 = 5;
/// Generic cpu/io
pub const CTL_HW: i32 = 6;
/// Machine dependent
pub const CTL_MACHDEP: i32 = 7;
// No 8, was CTL_USER, which is removed
/// DDB user interface, see /usr/include/ddb/db_var.h
pub const CTL_DDB: i32 = 9;
/// VFS sysctl's
pub const CTL_VFS: i32 = 10;
/// Number of valid top-level ids
pub const CTL_MAXID: i32 = 11;

/// Map the CTL names to their types
pub const CTL_NAMES: [(&str, i32); 11] = [
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
pub const KERN_OSTYPE: i32 = 1;
/// String: system release
pub const KERN_OSRELEASE: i32 = 2;
/// Int: system revision
pub const KERN_OSREV: i32 = 3;
/// String: compile time info
pub const KERN_VERSION: i32 = 4;
/// Int: max vnodes
pub const KERN_MAXVNODES: i32 = 5;
/// Int: max processes
pub const KERN_MAXPROC: i32 = 6;
/// Int: max open files
pub const KERN_MAXFILES: i32 = 7;
/// Int: max arguments to exec
pub const KERN_ARGMAX: i32 = 8;
/// Int: system security level
pub const KERN_SECURELVL: i32 = 9;
/// String: hostname
pub const KERN_HOSTNAME: i32 = 10;
/// Int: host identifier
pub const KERN_HOSTID: i32 = 11;
/// Stuct: struct clockinfo
pub const KERN_CLOCKRATE: i32 = 12;
// No 13, 14 or 15. Were KERN_DNSJACKPORT, KERN_PROC and KERN_FILE
/// Node: kernel profiling info
pub const KERN_PROF: i32 = 16;
/// Int: POSIX.1 version
pub const KERN_POSIX1: i32 = 17;
/// Int: ~ of supplemental group ids
pub const KERN_NGROUPS: i32 = 18;
/// Int: is job control available
pub const KERN_JOB_CONTROL: i32 = 19;
/// Int: saved set-user/group-ID
pub const KERN_SAVED_IDS: i32 = 20;
/// Struct: time kernel was booted
pub const KERN_BOOTTIME: i32 = 21;
/// String: (YP) domain name
pub const KERN_DOMAINNAME: i32 = 22;
/// Int: number of partitions / disk
pub const KERN_MAXPARTITIONS: i32 = 23;
/// Int: raw partition number
pub const KERN_RAWPARTITION: i32 = 24;
/// Int: max threads
pub const KERN_MAXTHREAD: i32 = 25;
/// Int: number of threads
pub const KERN_NTHREADS: i32 = 26;
/// String: kernel build version
pub const KERN_OSVERSION: i32 = 27;
/// Int: listen queue maximum
pub const KERN_SOMAXCONN: i32 = 28;
/// Int: half-open controllable param
pub const KERN_SOMINCONN: i32 = 29;
// No 30 or 31, were KERN_USERMOUNT and KERN_RND
/// Int: no setuid coredumps ever
pub const KERN_NOSUIDCOREDUMP: i32 = 32;
/// Int: file synchronization support
pub const KERN_FSYNC: i32 = 33;
/// Int: SysV message queue support
pub const KERN_SYSVMSG: i32 = 34;
/// Int: SysV semaphore support
pub const KERN_SYSVSEM: i32 = 35;
/// Int: SysV shared memory support
pub const KERN_SYSVSHM: i32 = 36;
// No 37, was KERN_ARND
/// Int: size of message buffer
pub const KERN_MSGBUFSIZE: i32 = 38;
/// Note: malloc statistics
pub const KERN_MALLOCSTATS: i32 = 39;
/// Array: cp_time
pub const KERN_CPTIME: i32 = 40;
/// Struct: vfs cache statistics
pub const KERN_NCHSTATS: i32 = 41;
/// Struct: fork statistics
pub const KERN_FORKSTAT: i32 = 42;
// No 43, was KERN_NSELCOLL
/// Node: tty information
pub const KERN_TTY: i32 = 44;
/// Int: ccpu
pub const KERN_CCPU: i32 = 45;
/// Int: fscale
pub const KERN_FSCALE: i32 = 46;
/// Int: number of processes
pub const KERN_NPROCS: i32 = 47;
/// Message buffer, KERN_MSGBUFSIZE
pub const KERN_MSGBUF: i32 = 48;
/// Struct: pool information
pub const KERN_POOL: i32 = 49;
/// Int: stackgap_random
pub const KERN_STACKGAPRANDOM: i32 = 50;
/// Struct: SysV sem/shm/msg info
pub const KERN_SYSVIPC_INFO: i32 = 51;
/// Int: allowkmem
pub const KERN_ALLOWKMEM: i32 = 52;
/// Int: witnesswatch
pub const KERN_WITNESSWATCH: i32 = 53;
/// Int: splassert
pub const KERN_SPLASSERT: i32 = 54;
/// Node: proc args and env
pub const KERN_PROC_ARGS: i32 = 55;
/// Int: number of open files
pub const KERN_NFILES: i32 = 56;
/// Int: number of tty devices
pub const KERN_TTYCOUNT: i32 = 57;
/// Int: number of vnodes in use
pub const KERN_NUMVNODES: i32 = 58;
/// Struct: mbuf statistics
pub const KERN_MBSTAT: i32 = 59;
/// Node: witness
pub const KERN_WITNESS: i32 = 60;
/// Struct: SysV struct seminfo
pub const KERN_SEMINFO: i32 = 61;
/// Struct: SysV struct shminfo
pub const KERN_SHMINFO: i32 = 62;
/// Node: interrupt counters
pub const KERN_INTRCNT: i32 = 63;
/// Node: watchdog
pub const KERN_WATCHDOG: i32 = 64;
/// Int: allowdt
pub const KERN_ALLOWDT: i32 = 65;
/// Struct: process entries
pub const KERN_PROC: i32 = 66;
/// Number of mclusters
pub const KERN_MAXCLUSTERS: i32 = 67;
/// Node: event counters
pub const KERN_EVCOUNT: i32 = 68;
/// Node: timecounter
pub const KERN_TIMECOUNTER: i32 = 69;
/// Int: locks per uid
pub const KERN_MAXLOCKSPERUID: i32 = 70;
/// Array: cp_time2
pub const KERN_CPTIME2: i32 = 71;
/// Buffer cache % of physmem
pub const KERN_CACHEPCT: i32 = 72;
/// Struct: file entries
pub const KERN_FILE: i32 = 73;
/// Int: w^x sigabrt & core
pub const KERN_WXABORT: i32 = 74;
/// Dev_t console terminal device
pub const KERN_CONSDEV: i32 = 75;
/// Int: Int: number of network livelocks
pub const KERN_NETLIVELOCKS: i32 = 76;
/// Int: enable pool_debug
pub const KERN_POOL_DEBUG: i32 = 77;
/// Node: proc cwd
pub const KERN_PROC_CWD: i32 = 78;
/// Node: proc no broadcast kill
pub const KERN_PROC_NOBROADCASTKILL: i32 = 79;
/// Node: proc vmmap
pub const KERN_PROC_VMMAP: i32 = 80;
/// Allow ptrace globally
pub const KERN_GLOBAL_PTRACE: i32 = 81;
/// Int: console message buffer size
pub const KERN_CONSBUFSIZE: i32 = 82;
/// Console message buffer
pub const KERN_CONSBUF: i32 = 83;
/// Console message buffer
pub const KERN_AUDIO: i32 = 84;
/// Struct: audio properties
pub const KERN_CPUSTATS: i32 = 85;
/// Struct: pf status and stats
pub const KERN_PFSTATUS: i32 = 86;
/// Struct: timeout status and stats
pub const KERN_TIMEOUT_STATS: i32 = 87;
/// Int: adjust RTC time to UTC
pub const KERN_UTC_OFFSET: i32 = 88;
/// Int: Struct: video properties
pub const KERN_VIDEO: i32 = 89;
/// Node: clockintr
pub const KERN_CLOCKINTR: i32 = 90;
/// Int: kernel device tree state serial
pub const KERN_AUTOCONF_SERIAL: i32 = 91;
/// Number of valid kern ids
pub const KERN_MAXID: i32 = 92;

/// Map CTL kern to their types
pub const CTL_KERN_NAMES: [(&str, i32); 92] = [
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
pub const KERN_PROC_ALL: i32 = 0;
/// By process id
pub const KERN_PROC_PID: i32 = 1;
/// By process group id
pub const KERN_PROC_PGRP: i32 = 2;
/// By session of pid
pub const KERN_PROC_SESSION: i32 = 3;
/// By controlling tty
pub const KERN_PROC_TTY: i32 = 4;
/// By effective uid
pub const KERN_PROC_UID: i32 = 5;
/// By real uid
pub const KERN_PROC_RUID: i32 = 6;
/// Also return kernel threads
pub const KERN_PROC_KTHREAD: i32 = 7;
/// Also return normal threads
pub const KERN_PROC_SHOW_THREADS: i32 = 0x40000000;

// KERN_SYSVIPC_INFO subtypes
/// Msginfo and msgid_ds
pub const KERN_SYSVIPC_MSG_INFO: i32 = 1;
/// Seminfo and semid_ds
pub const KERN_SYSVIPC_SEM_INFO: i32 = 2;
/// Shminfo and shmid_ds
pub const KERN_SYSVIPC_SHM_INFO: i32 = 3;

// KERN_PROC_ARGS subtypes
pub const KERN_PROC_ARGV: i32 = 1;
pub const KERN_PROC_NARGV: i32 = 2;
pub const KERN_PROC_ENV: i32 = 3;
pub const KERN_PROC_NENV: i32 = 4;

// KERN_AUDIO
pub const KERN_AUDIO_RECORD: i32 = 1;
pub const KERN_AUDIO_KBDCONTROL: i32 = 2;
pub const KERN_AUDIO_MAXID: i32 = 3;

/// Map the kern audio names to their types
pub const CTL_KERN_AUDIO_NAMES: [(&str, i32); 3] = [
    ("", 0),
    ("record", CTLTYPE_INT),
    ("kbdcontrol", CTLTYPE_INT),
];

// KERN_VIDEO
pub const KERN_VIDEO_RECORD: i32 = 1;
pub const KERN_VIDEO_MAXID: i32 = 2;

/// Map the kern video names to their types
pub const CTL_KERN_VIDEO_NAMES: [(&str, i32); 2] = [("", 0), ("record", CTLTYPE_INT)];

// KERN_WITNESS
pub const KERN_WITNESS_WATCH: i32 = 1;
pub const KERN_WITNESS_LOCKTRACE: i32 = 2;
pub const KERN_WITNESS_MAXID: i32 = 3;

/// Map their kern witness names to their types
pub const CTL_KERN_WITNESS_NAMES: [(&str, i32); 3] =
    [("", 0), ("watch", CTLTYPE_INT), ("locktrace", CTLTYPE_INT)];

/*
 * KERN_PROC subtype ops return arrays of relatively fixed size
 * structures of process info.   Use 8 byte alignment, and new
 * elements should only be added to the end of this structure so
 * binary compatibility can be preserved.
 */
pub const KI_NGROUPS: usize = 16;
/// Includes NUL. From /usr/include/sys/syslimits.h:83
pub const KI_MAXCOMLEN: usize = 24;
pub const KI_WMESGLEN: usize = 8;
pub const KI_MAXLOGNAME: usize = 32;
pub const KI_EMULNAMELEN: usize = 8;

pub const KI_NOCPU: u64 = u64::MAX;

/// Controlling tty vnode active
pub const EPROC_CTTY: i32 = 0x01;
/// Session leader
pub const EPROC_SLEADER: i32 = 0x02;
/// Has unveil settings
pub const EPROC_UNVEIL: i32 = 0x04;
/// Unveil is locked
pub const EPROC_LKUNVAIL: i32 = 0x08;

/// See /usr/include/sys/sysctl.h
#[allow(non_snake_case)]
pub struct KinfoProc {
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
    pub p_eflag: u64,
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
    pub p_groups: [u32; KI_NGROUPS],
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
    pub p_comm: [u8; KI_MAXLOGNAME],
    /// wchan message
    pub p_wmesg: [u8; KI_WMESGLEN],
    /// PTR: sleep address
    pub p_wchan: u64,
    /// setlogin() name
    pub p_login: [u8; KI_MAXLOGNAME],
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
    pub p_emul: [u8; KI_EMULNAMELEN],
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
    pub p_name: [u8; KI_MAXCOMLEN],
}

/// VM address range entry, matching struct vm_map_entry.  Useful for
/// debuggers to know process's addresses.
///
/// To iterate entries, set the last kve_end as the base address into
/// kve_start.
pub struct KinfoVmEntry {
    /// vaddr_t
    pub kve_start: libc::c_ulong,
    /// vaddr_t
    pub kve_end: libc::c_ulong,
    /// vsize_t
    pub kve_guard: libc::c_ulong,
    /// vsize_t
    pub kve_fspace: libc::c_ulong,
    /// vsize_t
    pub kve_fspace_augment: libc::c_ulong,
    /// voff_t
    pub kve_offset: u64,
    pub kve_wired_count: libc::c_int,
    pub kve_etype: libc::c_int,
    pub kve_protection: libc::c_int,
    pub kve_max_protection: libc::c_int,
    pub kve_advice: libc::c_int,
    pub kve_inheritance: libc::c_int,
    pub kve_flags: u8,
}

// keep in sync with UVM_ET_*
pub const KVE_ET_OBJ: i32 = 0x00000001;
pub const KVE_ET_SUBMAP: i32 = 0x00000002;
pub const KVE_ET_COPYONWRITE: i32 = 0x00000004;
pub const KVE_ET_NEEDSCOPY: i32 = 0x00000008;
pub const KVE_ET_HOLE: i32 = 0x00000010;
pub const KVE_ET_NOFAULT: i32 = 0x00000020;
pub const KVE_ET_STACK: i32 = 0x00000040;
pub const KVE_ET_WC: i32 = 0x00000080;
pub const KVE_ET_CONCEAL: i32 = 0x00000100;
pub const KVE_ET_SYSCALL: i32 = 0x00000200;
pub const KVE_ET_FREEMAPPED: i32 = 0x00000800;

pub const KVE_PROT_NONE: i32 = 0x00000000;
pub const KVE_PROT_READ: i32 = 0x00000001;
pub const KVE_PROT_WRITE: i32 = 0x00000002;
pub const KVE_PROT_EXEC: i32 = 0x00000004;

pub const KVE_ADV_NORMAL: i32 = 0x00000000;
pub const KVE_ADV_RANDOM: i32 = 0x00000001;
pub const KVE_ADV_SEQUENTIAL: i32 = 0x00000002;

pub const KVE_INH_SHARE: i32 = 0x00000000;
pub const KVE_INH_COPY: i32 = 0x00000010;
pub const KVE_INH_NONE: i32 = 0x00000020;
pub const KVE_INH_ZERO: i32 = 0x00000030;

pub const KVE_F_STATIC: i32 = 0x01;
pub const KVE_F_KMEM: i32 = 0x02;

/*
 * kern.file returns an array of these structures, which are designed
 * both to be immune to 32/64 bit emulation issues and to
 * provide backwards compatibility.  The order differs slightly from
 * that of the real struct file, and some fields are taken from other
 * structures (struct vnode, struct proc) in order to make the file
 * information more useful.
 */
pub const KERN_FILE_BYFILE: i32 = 1;
pub const KERN_FILE_BYPID: i32 = 2;
pub const KERN_FILE_BYUID: i32 = 3;
pub const KERN_FILESLOP: i32 = 10;

pub const KERN_FILE_TEXT: i32 = -1;
pub const KERN_FILE_CDIR: i32 = -2;
pub const KERN_FILE_RDIR: i32 = -3;
pub const KERN_FILE_TRACE: i32 = -4;

/// Rounded up from 90
pub const KI_MNAMELEN: usize = 96;
pub const KI_UNPPATHLEN: usize = 104;

pub struct KinfoFile {
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
    pub f_mntonname: [u8; KI_MNAMELEN],
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
    pub p_comm: [u8; KI_MAXCOMLEN],
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
    pub unp_path: [u8; KI_UNPPATHLEN],
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
pub const KERN_INTRCNT_NUM: i32 = 1;
/// Node: intrcnt
pub const KERN_INTRCNT_CNT: i32 = 2;
/// Node: names
pub const KERN_INTRCNT_NAME: i32 = 3;
/// Node: interrupt vector
pub const KERN_INTRCNT_VECTOR: i32 = 4;
pub const KERN_INTRCNT_MAXID: i32 = 5;

/// Map the CTL kern intrcnt names to their types
/// See /usr/include/sys/sysctl:851
pub const CTL_KERN_INTRCNT_NAMES: [(&str, i32); 4] = [
    ("", 0),
    ("nintrcnt", CTLTYPE_INT),
    ("intrcnt", CTLTYPE_NODE),
    ("intrname", CTLTYPE_NODE),
];

// KERN_WATCHDOG
pub const KERN_WATCHDOG_PERIOD: i32 = 1;
pub const KERN_WATCHDOG_AUTO: i32 = 2;
pub const KERN_WATCHDOG_MAXID: i32 = 3;

pub const CTL_KERN_WATCHDOG_NAMES: [(&str, i32); 3] =
    [("", 0), ("period", CTLTYPE_INT), ("auto", CTLTYPE_INT)];

// KERN_TIMECOUNTER

/// Int: number of revolutions
pub const KERN_TIMECOUNTER_TICK: i32 = 1;
/// Int: log a warning when time change
pub const KERN_TIMECOUNTER_TIMESTEPWARNINGS: i32 = 2;
/// String: tick hardware used
pub const KERN_TIMECOUNTER_HARDWARE: i32 = 3;
/// String: tick hardware used
pub const KERN_TIMECOUNTER_CHOICE: i32 = 4;
pub const KERN_TIMECOUNTER_MAXID: i32 = 5;

/// Map the CTL kern timetounter names to their types
pub const CTL_KERN_TIMECOUNTER_NAMES: [(&str, i32); 5] = [
    ("", 0),
    ("tick", CTLTYPE_INT),
    ("timestepwarnings", CTLTYPE_INT),
    ("hardware", CTLTYPE_STRING),
    ("choice", CTLTYPE_STRING),
];

// KERN_CLOCKINTR
/// Struct: stats
pub const KERN_CLOCKINTR_STATS: i32 = 1;
pub const KERN_CLOCKINTR_MAXID: i32 = 2;

/// Map the CTL kern clockintr names to their types
pub const CTL_KERN_CLOCKINTR_NAMES: [(&str, i32); 2] = [("", 0), ("stats", CTLTYPE_STRUCT)];

// CTL_HW identifiers
/// String: machine class
pub const HW_MACHINE: i32 = 1;
/// String: specific machine model
pub const HW_MODEL: i32 = 2;
/// Int: number of configured cpus
pub const HW_NCPU: i32 = 3;
/// Int: machine byte order
pub const HW_BYTEORDER: i32 = 4;
/// Int: total memory
pub const HW_PHYSMEM: i32 = 5;
/// Int: non-kernel memory
pub const HW_USERMEM: i32 = 6;
/// Int: software page size
pub const HW_PAGESIZE: i32 = 7;
/// Strings: disk drive names
pub const HW_DISKNAMES: i32 = 8;
/// Struct: diskstats[]
pub const HW_DISKSTATS: i32 = 9;
/// Int: number of disks
pub const HW_DISKCOUNT: i32 = 10;
/// Node: hardware monitors
pub const HW_SENSORS: i32 = 11;
/// Get CPU frequency
pub const HW_CPUSPEED: i32 = 12;
/// Set CPU performance
pub const HW_SETPERF: i32 = 13;
/// String: vendor name
pub const HW_VENDOR: i32 = 14;
/// String: product name
pub const HW_PRODUCT: i32 = 15;
/// String: hardware version
pub const HW_VERSION: i32 = 16;
/// String: hardware serial number
pub const HW_SERIALNO: i32 = 17;
/// String: universal unique id
pub const HW_UUID: i32 = 18;
/// Quad: total memory
pub const HW_PHYSMEM64: i32 = 19;
/// Quad: non-kernel memory
pub const HW_USERMEM64: i32 = 20;
/// Int: number of cpus found
pub const HW_NCPUFOUND: i32 = 21;
/// Allow power button shutdown
pub const HW_ALLOWPOWERDOWN: i32 = 22;
/// Set performance policy
pub const HW_PERFPOLICY: i32 = 23;
/// Int: enable SMT/HT/CMT
pub const HW_SMT: i32 = 24;
/// Int: number of cpus being used
pub const HW_NCPUONLINE: i32 = 25;
/// Int: machine has wall-power
pub const HW_POWER: i32 = 26;
/// Node: battery
pub const HW_BATTERY: i32 = 27;
/// Strings: ucom names
pub const HW_UCOMNAMES: i32 = 28;
/// String: cpu types to block
pub const HW_BLOCKCPU: i32 = 29;
/// Number of valid hw ids
pub const HW_MAXID: i32 = 30;

/// Map the CTL hw names to their type
/// See /usr/include/sys/sysctl.h:933
pub const CTL_HW_NAMES: [(&str, i32); 30] = [
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
pub const HW_BATTERY_CHARGEMODE: i32 = 1;
/// Int: battery start charge percent
pub const HW_BATTERY_CHARGESTART: i32 = 2;
/// Int: battery stop charge percent
pub const HW_BATTERY_CHARGESTOP: i32 = 3;
pub const HW_BATTERY_MAXID: i32 = 4;

/// Mat the CTL hw battery names to their types
pub const CTL_HW_BATTERY_NAMES: [(&str, i32); 4] = [
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
pub const CTL_DEBUG_NAME: i32 = 1;
/// Int: variale value
pub const CTL_DEBUG_VALUE: i32 = 2;
pub const CTL_DEBUG_MAXID: i32 = 20;

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
pub struct CtlDebug {
    /// Name of debugging variable
    pub debugname: *mut libc::c_char,
    /// Pointer to debugging variable
    pub debugvar: *mut libc::c_int,
}

/// Exported sysctl variable with valid bounds. Both bounds are inclusive to
/// allow full range of values.
///
pub struct SysctlBoundedArgs {
    /// identifier shared with userspace as a CTL_ constant
    pub mib: libc::c_int,
    /// Never NULL
    pub var: *mut libc::c_int,
    /// Checking is disabled if minimum == maximum
    pub minimum: libc::c_int,
    /// Read-only variable if minimum > maximum
    pub maximum: libc::c_int,
}

/// Special case minimum,maximum marker for sysctl_bounded_args.
pub const SYSCTL_INT_READONLY: (i32, i32) = (1, 0);
