use std::ops::{Deref, DerefMut};
use std::ffi::CStr;
use std::mem::transmute;
use std::iter;
use libc::c_char;

pub struct NullTerminatedSlice<T> {
    inner: [Option<T>],
}

impl<T> NullTerminatedSlice<T> {
    pub fn from_slice(slice: &[Option<T>]) -> Option<&Self> {
        if slice.last().map(Option::is_none).unwrap_or(false) {
            Some(unsafe { Self::from_slice_unchecked(slice) })
        } else {
            None
        }
    }

    pub fn from_slice_mut(slice: &mut [Option<T>]) -> Option<&mut Self> {
        if slice.last().map(Option::is_none).unwrap_or(false) {
            Some(unsafe { Self::from_slice_mut_unchecked(slice) })
        } else {
            None
        }
    }

    pub unsafe fn from_slice_unchecked(slice: &[Option<T>]) -> &Self {
        transmute(slice)
    }

    pub unsafe fn from_slice_mut_unchecked(slice: &mut [Option<T>]) -> &mut Self {
        transmute(slice)
    }
}

impl<'a, U: Sized> NullTerminatedSlice<&'a U> {
    pub fn as_ptr(&self) -> *const *const U {
        self.inner.as_ptr() as *const _
    }
}

impl<T> Deref for NullTerminatedSlice<T> {
    type Target = [Option<T>];

    fn deref(&self) -> &Self::Target {
        &self.inner[..self.inner.len() - 1]
    }
}

impl<T> DerefMut for NullTerminatedSlice<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let len = self.inner.len();
        &mut self.inner[..len - 1]
    }
}

pub trait BorrowNullTerminatedSlice<T> {
    fn borrow_null_terminated_slice<R, F: FnOnce(&NullTerminatedSlice<&T>) -> R>(self, f: F) -> R;
}

impl<T: AsRef<CStr>, I: IntoIterator<Item=T>> BorrowNullTerminatedSlice<c_char> for I {
    fn borrow_null_terminated_slice<R, F: FnOnce(&NullTerminatedSlice<&c_char>) -> R>(self, f: F) -> R {
        fn cstr_char<'a, S: AsRef<CStr> + 'a>(s: S) -> &'a c_char {
            unsafe {
                &*s.as_ref().as_ptr()
            }
        }

        let values: Vec<_> = self.into_iter()
            .map(cstr_char)
            .map(Some).chain(iter::once(None)).collect();
        let terminated = unsafe { NullTerminatedSlice::from_slice_unchecked(&values[..]) };

        f(terminated)
    }
}

impl<'a, T: 'a> BorrowNullTerminatedSlice<T> for &'a NullTerminatedSlice<&'a T> {
    fn borrow_null_terminated_slice<R, F: FnOnce(&NullTerminatedSlice<&T>) -> R>(self, f: F) -> R {
        f(self)
    }
}
