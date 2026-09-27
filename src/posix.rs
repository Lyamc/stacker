//! The few libc symbols stacker needs, declared here so this crate does not
//! depend on the `libc` crate. Values match the platform C headers.

use std::ffi::{c_int, c_void};

pub type Pthread = usize;

#[repr(C, align(8))]
pub struct PthreadAttr {
    _opaque: [u64; 32],
}

pub const PROT_NONE: c_int = 0;
pub const PROT_READ: c_int = 1;
pub const PROT_WRITE: c_int = 2;
pub const MAP_FAILED: *mut c_void = !0usize as *mut c_void;

#[cfg(any(target_os = "linux", target_os = "android"))]
pub const MAP_PRIVATE: c_int = 0x02;
#[cfg(any(target_os = "linux", target_os = "android"))]
pub const MAP_FIXED: c_int = 0x10;
#[cfg(all(any(target_os = "linux", target_os = "android"), not(target_arch = "mips")))]
pub const MAP_ANON: c_int = 0x20;
#[cfg(all(any(target_os = "linux", target_os = "android"), target_arch = "mips"))]
pub const MAP_ANON: c_int = 0x800;
#[cfg(any(target_os = "linux", target_os = "android"))]
pub const SC_PAGE_SIZE: c_int = 30;

#[cfg(any(
    target_os = "macos",
    target_os = "ios",
    target_os = "freebsd",
    target_os = "dragonfly",
    target_os = "netbsd",
    target_os = "openbsd",
))]
pub const MAP_PRIVATE: c_int = 0x0002;
#[cfg(any(
    target_os = "macos",
    target_os = "ios",
    target_os = "freebsd",
    target_os = "dragonfly",
    target_os = "netbsd",
    target_os = "openbsd",
))]
pub const MAP_FIXED: c_int = 0x0010;
#[cfg(any(
    target_os = "macos",
    target_os = "ios",
    target_os = "freebsd",
    target_os = "dragonfly",
    target_os = "netbsd",
    target_os = "openbsd",
))]
pub const MAP_ANON: c_int = 0x1000;

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub const SC_PAGE_SIZE: c_int = 29;
#[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
pub const SC_PAGE_SIZE: c_int = 47;
#[cfg(any(target_os = "netbsd", target_os = "openbsd"))]
pub const SC_PAGE_SIZE: c_int = 28;

#[cfg(target_os = "openbsd")]
pub const MAP_STACK: c_int = 0x4000;

#[cfg(any(target_os = "solaris", target_os = "illumos"))]
pub const MAP_PRIVATE: c_int = 0x0002;
#[cfg(any(target_os = "solaris", target_os = "illumos"))]
pub const MAP_FIXED: c_int = 0x0010;
#[cfg(any(target_os = "solaris", target_os = "illumos"))]
pub const MAP_ANON: c_int = 0x0100;
#[cfg(any(target_os = "solaris", target_os = "illumos"))]
pub const SC_PAGE_SIZE: c_int = 11;

#[cfg(target_os = "haiku")]
pub const MAP_PRIVATE: c_int = 0x02;
#[cfg(target_os = "haiku")]
pub const MAP_FIXED: c_int = 0x04;
#[cfg(target_os = "haiku")]
pub const MAP_ANON: c_int = 0x08;
#[cfg(target_os = "haiku")]
pub const SC_PAGE_SIZE: c_int = 27;

unsafe extern "C" {
    pub fn mmap(
        addr: *mut c_void,
        len: usize,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: i64,
    ) -> *mut c_void;
    pub fn munmap(addr: *mut c_void, len: usize) -> c_int;
    pub fn mprotect(addr: *mut c_void, len: usize, prot: c_int) -> c_int;
    pub fn sysconf(name: c_int) -> i64;

    pub fn pthread_self() -> Pthread;
    pub fn pthread_attr_init(attr: *mut PthreadAttr) -> c_int;
    pub fn pthread_attr_destroy(attr: *mut PthreadAttr) -> c_int;
    pub fn pthread_attr_getstack(
        attr: *const PthreadAttr,
        stackaddr: *mut *mut c_void,
        stacksize: *mut usize,
    ) -> c_int;

    #[cfg(any(target_os = "linux", target_os = "solaris", target_os = "netbsd", target_os = "haiku"))]
    pub fn pthread_getattr_np(thread: Pthread, attr: *mut PthreadAttr) -> c_int;
    #[cfg(any(target_os = "freebsd", target_os = "dragonfly", target_os = "illumos"))]
    pub fn pthread_attr_get_np(thread: Pthread, attr: *mut PthreadAttr) -> c_int;

    #[cfg(any(target_os = "macos", target_os = "ios"))]
    pub fn pthread_get_stackaddr_np(thread: Pthread) -> *mut c_void;
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    pub fn pthread_get_stacksize_np(thread: Pthread) -> usize;

    #[cfg(target_os = "openbsd")]
    pub fn pthread_stackseg_np(thread: Pthread, stack: *mut StackT) -> c_int;
}

#[cfg(target_os = "openbsd")]
#[repr(C)]
pub struct StackT {
    pub ss_sp: *mut c_void,
    pub ss_size: usize,
    pub ss_flags: c_int,
}
