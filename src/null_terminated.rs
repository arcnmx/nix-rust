use std::mem;
use std::iter;
use std::slice;
use std::borrow::Borrow;
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

impl<'a, U: Sized> NullTerminatedSlice<&'a U> {
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

pub struct NullTerminatedArray<T> {
    inner: Box<[Option<T>]>,
}

impl<T> NullTerminatedArray<T> {
    pub fn new<I: IntoIterator<Item=T>>(iter: I) -> Self {
        NullTerminatedArray {
            inner: iter.into_iter().map(Some).chain(iter::once(None)).collect::<Vec<_>>().into_boxed_slice(),
        }
    }

    pub fn from_vec(mut vec: Vec<Option<T>>) -> Self {
        if !vec.last().map(Option::is_none).unwrap_or(false) {
            vec.push(None);
        }

        NullTerminatedArray {
            inner: vec.into_boxed_slice(),
        }
    }
}

impl<'a, T: 'a> NullTerminatedArray<&'a T> {
    fn map_from<A: CMapping<Target=T> + 'a, I: IntoIterator<Item=&'a A>>(iter: I) -> Self {
        Self::new(iter.into_iter().map(|c| c.map()))
    }
}

impl<T> Deref for NullTerminatedArray<T> {
    type Target = NullTerminatedSlice<T>;

    fn deref(&self) -> &Self::Target {
        unsafe {
            NullTerminatedSlice::from_slice_unchecked(&self.inner)
        }
    }
}

impl<T> DerefMut for NullTerminatedArray<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            NullTerminatedSlice::from_slice_mut_unchecked(&mut self.inner)
        }
    }
}

impl<T> AsRef<NullTerminatedSlice<T>> for NullTerminatedArray<T> {
    fn as_ref(&self) -> &NullTerminatedSlice<T> {
        self
    }
}

impl<T> AsRef<NullTerminatedSlice<T>> for NullTerminatedSlice<T> {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<T: Clone> ToOwned for NullTerminatedSlice<T> {
    type Owned = NullTerminatedArray<T>;

    fn to_owned(&self) -> Self::Owned {
        NullTerminatedArray::new(self.into_iter().cloned())
    }
}

impl<T> Borrow<NullTerminatedSlice<T>> for NullTerminatedArray<T> {
    fn borrow(&self) -> &NullTerminatedSlice<T> {
        self
    }
}

pub struct NullTerminatedVec<T, U> {
    inner: Box<[T]>,
    null: NullTerminatedArray<U>,
}

impl<T, U> NullTerminatedVec<T, U> {
    pub fn new<'a, F: FnMut(&'a T) -> U>(vec: Vec<T>, f: F) -> Self where U: 'a, T: 'a {
        let inner = vec.into_boxed_slice();
        let null = NullTerminatedArray::new(inner.iter()
            .map(|t| unsafe { mem::transmute(t) })
            .map(f)
        );

        NullTerminatedVec {
            inner: inner,
            null: null,
        }
    }
}

impl<'a, C: CMapping + 'a> NullTerminatedVec<C, &'a C::Target> {
    pub fn map_from<I: Into<Vec<C>>>(iter: I) -> Self {
        Self::new(iter.into(), |c| c.map())
    }
}

impl<T, U> Deref for NullTerminatedVec<T, U> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T, U> AsRef<NullTerminatedSlice<U>> for NullTerminatedVec<T, U> {
    fn as_ref(&self) -> &NullTerminatedSlice<U> {
        &self.null
    }
}

pub trait CMapping {
    type Target;

    fn map(&self) -> &Self::Target;
}

impl<S: NixString> CMapping for S {
    type Target = c_char;

    fn map(&self) -> &Self::Target {
        unsafe { &*self.as_ref().as_ptr() }
    }
}
