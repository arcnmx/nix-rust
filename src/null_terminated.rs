use std::mem;
use std::iter;
use std::slice;
use std::ops::{Deref, DerefMut};
use std::os::raw::c_char;
use NixString;

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
        mem::transmute(slice)
    }

    pub unsafe fn from_slice_mut_unchecked(slice: &mut [Option<T>]) -> &mut Self {
        mem::transmute(slice)
    }
}

impl<'a, U> NullTerminatedSlice<&'a U> {
    pub fn as_ptr(&self) -> *const *const U {
        self.inner.as_ptr() as *const _
    }
}

impl<'a, T: 'a> IntoIterator for &'a NullTerminatedSlice<T> {
    type Item = &'a T;
    type IntoIter = NullTerminatedIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        NullTerminatedIterator {
            iter: self.iter(),
        }
    }
}

pub struct NullTerminatedIterator<'a, T: 'a> {
    iter: slice::Iter<'a, Option<T>>,
}

impl<'a, T: 'a> Iterator for NullTerminatedIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(&Some(ref next)) = self.iter.next() {
            Some(next)
        } else {
            None
        }
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

pub struct NullTerminatedVec<T> {
    inner: Vec<Option<T>>,
}

impl<T> NullTerminatedVec<T> {
    pub fn new<I: IntoIterator<Item=T>>(iter: I) -> Self {
        NullTerminatedVec {
            inner: iter.into_iter().map(Some).chain(iter::once(None)).collect(),
        }
    }

    pub fn from_vec(mut vec: Vec<Option<T>>) -> Self {
        if vec.last().map(Option::is_none).unwrap_or(false) {
            vec.push(None);
        }

        NullTerminatedVec {
            inner: vec,
        }
    }
}

impl<'a> NullTerminatedVec<&'a c_char> {
    fn from_cstrings<A: NixString + 'a, I: IntoIterator<Item=&'a A>>(iter: I) -> Self {
        unsafe { Self::new(iter.into_iter().map(|s| &*s.as_ref().as_ptr())) }
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

impl<T> AsRef<NullTerminatedSlice<T>> for NullTerminatedVec<T> {
    fn as_ref(&self) -> &NullTerminatedSlice<T> {
        self.deref()
    }
}

impl<T> AsRef<NullTerminatedSlice<T>> for NullTerminatedSlice<T> {
    fn as_ref(&self) -> &Self {
        self
    }
}
