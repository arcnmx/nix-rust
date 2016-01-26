use std::ffi::CStr;

/// Represents a type that can be converted to a `&CStr` without fail.
///
/// Note: this trait is superceded by `AsRef<CStr>` in Rust 1.7.0
pub trait NixString {
	fn as_ref(&self) -> &CStr;
}

#[cfg(feature = "nixstring")]
mod imp {
	use std::ffi::{CStr, CString};
	use super::NixString;

	#[cfg(std_has_cstr_toowned)]
	impl<'a> NixString for ::std::borrow::Cow<'a, CStr> {
		fn as_ref(&self) -> &CStr {
			self
		}
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
}

#[cfg(not(feature = "nixstring"))]
impl<T: AsRef<CStr>> NixString for T {
	fn as_ref(&self) -> &CStr {
		return AsRef::as_ref(self)
	}
}

#[macro_export]
macro_rules! cstr {
	($s:expr) => {
		unsafe { ::std::ffi::CStr::from_ptr(concat!($s, "\0").as_ptr() as *const _) }
	}
}
