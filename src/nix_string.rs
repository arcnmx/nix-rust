use std::ffi::{CStr, CString};
use std::borrow::Cow;

// TODO: Replace this with AsRef<CStr> once Rust 1.7.0 lands.
pub trait NixString {
    fn as_ref(&self) -> &CStr;
}

impl NixString for CStr {
    fn as_ref(&self) -> &CStr {
        self
    }
}

impl NixString for CString {
    fn as_ref(&self) -> &CStr {
        self
    }
}

impl<'a, T: ?Sized + NixString> NixString for &'a T {
    fn as_ref(&self) -> &CStr {
        NixString::as_ref(*self)
    }
}

impl<'a, T: ?Sized + NixString> NixString for &'a mut T {
    fn as_ref(&self) -> &CStr {
        NixString::as_ref(*self)
    }
}

impl<'a> NixString for Cow<'a, CStr> {
    fn as_ref(&self) -> &CStr {
        self
    }
}

#[macro_export]
macro_rules! cstr {
    ($s:expr) => {
        unsafe { ::std::ffi::CStr::from_ptr(concat!($s, "\0").as_ptr() as *const _) }
    }
}
