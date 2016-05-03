use std::ops::{Deref, DerefMut};
use std::ffi::CStr;
use std::mem::transmute;
use std::iter;
use libc::c_char;

pub trait IntoRef<'a, T: ?Sized> {
    type Target: 'a + AsRef<T>;

    fn into_ref(self) -> Self::Target;
}

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

impl<T> AsRef<NullTerminatedSlice<T>> for NullTerminatedSlice<T> {
    fn as_ref(&self) -> &Self {
        self
    }
}

pub struct NullTerminatedVec<T> {
    inner: Vec<Option<T>>,
}

impl<T> NullTerminatedVec<T> {
    pub fn from_vec(vec: Vec<Option<T>>) -> Option<Self> {
        if vec.last().map(Option::is_none).unwrap_or(false) {
            Some(unsafe { Self::from_vec_unchecked(vec) })
        } else {
            None
        }
    }

    pub unsafe fn from_vec_unchecked(vec: Vec<Option<T>>) -> Self {
        NullTerminatedVec {
            inner: vec,
        }
    }
}

impl<T> Deref for NullTerminatedVec<T> {
    type Target = NullTerminatedSlice<T>;

    fn deref(&self) -> &Self::Target {
        unsafe {
            NullTerminatedSlice::from_slice_unchecked(&self.inner)
        }
    }
}

impl<T> DerefMut for NullTerminatedVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            NullTerminatedSlice::from_slice_mut_unchecked(&mut self.inner)
        }
    }
}

impl<T> AsRef<NullTerminatedSlice<T>> for NullTerminatedVec<T> {
    fn as_ref(&self) -> &NullTerminatedSlice<T> {
        self
    }
}

impl<'a, T: 'a> IntoRef<'a, NullTerminatedSlice<&'a T>> for &'a NullTerminatedSlice<&'a T> {
    type Target = &'a NullTerminatedSlice<&'a T>;

    fn into_ref(self) -> Self::Target {
        self
    }
}

impl<'a, T: AsRef<CStr> + 'a, I: IntoIterator<Item=T>> IntoRef<'a, NullTerminatedSlice<&'a c_char>> for I {
    type Target = NullTerminatedVec<&'a c_char>;

    fn into_ref(self) -> Self::Target {
        fn cstr_char<'a, S: AsRef<CStr> + 'a>(s: S) -> &'a c_char {
            unsafe {
                &*s.as_ref().as_ptr()
            }
        }

        let terminated = self.into_iter()
            .map(cstr_char)
            .map(Some).chain(iter::once(None)).collect();

        unsafe {
            NullTerminatedVec::from_vec_unchecked(terminated)
        }
    }
}
