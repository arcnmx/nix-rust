use errno::{Errno, Result};
use libc::pid_t;

mod ffi {
    use libc::c_int;

    pub const _LINUX_CAPABILITY_VERSION_1: u32 = 0x19980330;
    pub const _LINUX_CAPABILITY_VERSION_2: u32 = 0x20071026;
    pub const _LINUX_CAPABILITY_VERSION_3: u32 = 0x20080522;

    #[repr(C)]
    pub struct cap_user_header_t {
        pub version: u32,
        pub pid: c_int,
    }

    #[derive(Clone, Copy, Debug)]
    #[repr(C)]
    pub struct cap_user_data_t {
        pub effective: u32,
        pub permitted: u32,
        pub inheritable: u32,
    }

    extern {
        pub fn capget(hdrp: *mut cap_user_header_t, datap: *mut cap_user_data_t) -> c_int;
        pub fn capset(hdrp: *mut cap_user_header_t, datap: *const cap_user_data_t) -> c_int;
    }
}

bitflags!(
    #[repr(C)]
    flags CapabilityFlags: u64 {
        const CAP_CHOWN = 1 << 0,
        const CAP_DAC_OVERRIDE = 1 << 1,
        const CAP_DAC_READ_SEARCH = 1 << 2,
        const CAP_FOWNER = 1 << 3,
        const CAP_FSETID = 1 << 4,
        const CAP_KILL = 1 << 5,
        const CAP_SETGID = 1 << 6,
        const CAP_SETUID = 1 << 7,
        const CAP_SETPCAP = 1 << 8,
        const CAP_LINUX_IMMUTABLE = 1 << 9,
        const CAP_NET_BIND_SERVICE = 1 << 10,
        const CAP_NET_BROADCAST = 1 << 11,
        const CAP_NET_ADMIN = 1 << 12,
        const CAP_NET_RAW = 1 << 13,
        const CAP_IPC_LOCK = 1 << 14,
        const CAP_IPC_OWNER = 1 << 15,
        const CAP_SYS_MODULE = 1 << 16,
        const CAP_SYS_RAWIO = 1 << 17,
        const CAP_SYS_CHROOT = 1 << 18,
        const CAP_SYS_PTRACE = 1 << 19,
        const CAP_SYS_PACCT = 1 << 20,
        const CAP_SYS_ADMIN = 1 << 21,
        const CAP_SYS_BOOT = 1 << 22,
        const CAP_SYS_NICE = 1 << 23,
        const CAP_SYS_RESOURCE = 1 << 24,
        const CAP_SYS_TIME = 1 << 25,
        const CAP_SYS_TTY_CONFIG = 1 << 26,
        const CAP_MKNOD = 1 << 27,
        const CAP_LEASE = 1 << 28,
        const CAP_AUDIT_WRITE = 1 << 29,
        const CAP_AUDIT_CONTROL = 1 << 30,
        const CAP_SETFCAP = 1 << 31,
        const CAP_MAC_OVERRIDE = 1 << 32,
        const CAP_MAC_ADMIN = 1 << 33,
        const CAP_SYSLOG = 1 << 34,
        const CAP_WAKE_ALARM = 1 << 35,
        const CAP_BLOCK_SUSPEND = 1 << 36,
        const CAP_AUDIT_READ = 1 << 37,
    }
);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CapabilityKind {
    Effective,
    Permitted,
    Inheritable,
}

#[derive(Copy, Clone, Debug)]
pub struct Capabilities {
    inner: [ffi::cap_user_data_t; 2],
}

impl Capabilities {
    #[inline]
    pub fn empty() -> Self {
        unsafe { ::std::mem::zeroed() }
    }

    #[inline]
    pub fn get(&self, kind: CapabilityKind) -> CapabilityFlags {
        let (ms, ls) = match kind {
            CapabilityKind::Effective => (self.inner[1].effective, self.inner[0].effective),
            CapabilityKind::Permitted => (self.inner[1].permitted, self.inner[0].permitted),
            CapabilityKind::Inheritable => (self.inner[1].inheritable, self.inner[0].inheritable),
        };

        CapabilityFlags {
            bits: ((ms as u64) << 32) | ls as u64,
        }
    }

    #[inline]
    pub fn set(&mut self, kind: CapabilityKind, caps: CapabilityFlags) {
        let ms = (caps.bits >> 32) as u32;
        let ls = caps.bits as u32;
        match kind {
            CapabilityKind::Effective => {
                self.inner[1].effective = ms;
                self.inner[0].effective = ls;
            },
            CapabilityKind::Permitted => {
                self.inner[1].permitted = ms;
                self.inner[0].permitted = ls;
            },
            CapabilityKind::Inheritable => {
                self.inner[1].inheritable = ms;
                self.inner[0].inheritable = ls;
            },
        }
    }

    #[inline]
    pub fn set_all(&mut self, caps: CapabilityFlags) {
        self.set(CapabilityKind::Effective, caps);
        self.set(CapabilityKind::Permitted, caps);
        self.set(CapabilityKind::Inheritable, caps);
    }
}

pub fn capget(pid: pid_t) -> Result<Capabilities> {
    let mut hdr = ffi::cap_user_header_t {
        version: ffi::_LINUX_CAPABILITY_VERSION_3,
        pid: pid,
    };

    let mut caps = Capabilities::empty();
    let res = unsafe { ffi::capget(&mut hdr, caps.inner.as_mut_ptr()) };

    try!(Errno::result(res));

    Ok(caps)
}

pub fn capset(pid: pid_t, caps: &Capabilities) -> Result<()> {
    let mut hdr = ffi::cap_user_header_t {
        version: ffi::_LINUX_CAPABILITY_VERSION_3,
        pid: pid,
    };

    let res = unsafe { ffi::capset(&mut hdr, caps.inner.as_ptr()) };

    Errno::result(res).map(drop)
}
