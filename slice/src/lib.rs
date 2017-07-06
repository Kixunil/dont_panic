//! Non-panicking drop-in replacement for slices. Instead of panic it causes link time error if
//! bounds are not checked. (Not fully drop-in replacement yet. Some features are missing.)
//!
//! # Example
//!
//! ```no_compile
//! #[macro_use]
//! extern crate dont_panic_slice;
//!
//! use dont_panic_slice::DPSlice;
//!
//! fn main() {
//!     let arr = [0, 1, 2, 3];
//!     let dps = <&DPSlice<_>>::from(&arr as &[_]);
//!     assert_eq!(dps[0], 0);
//!     assert_eq!(dps[3], 3);
//!     // This would not compile (instead of run time panicking)
//!     assert_eq!(dps[42], 42);
//! }
//! ```
//!
//! You must compile it with `--release`. If you don't want to slow down debug builds, you can use
//! `--features=panic` to switch to normal panicking behaviour. 

#![no_std]

#[macro_use]
extern crate dont_panic;

pub struct DPSlice<T>([T]);

impl<T> DPSlice<T> {
    pub fn as_rust_slice(this: &Self) -> &[T] {
        unsafe { ::core::mem::transmute(this) }
    }

    pub fn as_rust_slice_mut(this: &mut Self) -> &mut [T] {
        unsafe { ::core::mem::transmute(this) }
    }

    pub fn len(&self) -> usize {
        Self::as_rust_slice(self).len()
    }

    pub fn is_empty(&self) -> bool {
        Self::as_rust_slice(self).is_empty()
    }

    pub fn first(&self) -> Option<&T> {
        Self::as_rust_slice(self).first()
    }

    pub fn first_mut(&mut self) -> Option<&mut T> {
        Self::as_rust_slice_mut(self).first_mut()
    }

    pub fn split_first(&self) -> Option<(&T, &[T])> {
        Self::as_rust_slice(self).split_first()
    }

    pub fn split_first_mut(&self) -> Option<(&T, &[T])> {
        Self::as_rust_slice(self).split_first()
    }

    pub fn split_last(&self) -> Option<(&T, &[T])> {
        Self::as_rust_slice(self).split_last()
    }

    pub fn split_last_mut(&mut self) -> Option<(&T, &[T])> {
        Self::as_rust_slice_mut(self).split_last()
    }

    pub fn swap(&mut self, a: usize, b: usize) {
        if a > self.len() {
            dont_panic!("index out of bounds: the len is {} but the index is {}", self.len(), a);
        }

        if b > self.len() {
            dont_panic!("index out of bounds: the len is {} but the index is {}", self.len(), b);
        }

        Self::as_rust_slice_mut(self).swap(a, b);
    }

    pub fn windows(&self, size: usize) -> ::core::slice::Windows<T> {
        dp_assert!(size != 0);

        Self::as_rust_slice(self).windows(size)
    }

    pub fn chunks(&self, size: usize) -> ::core::slice::Chunks<T> {
        dp_assert!(size != 0);

        Self::as_rust_slice(self).chunks(size)
    }

    pub fn chunks_mut(&mut self, size: usize) -> ::core::slice::ChunksMut<T> {
        dp_assert!(size != 0);

        Self::as_rust_slice_mut(self).chunks_mut(size)
    }

    pub fn split_at(&self, mid: usize) -> (&[T], &[T]) {
        if mid > self.len() {
            dont_panic!("index {} out of range for slice of length {}", mid, self.len());
        }

        Self::as_rust_slice(self).split_at(mid)
    }

    pub fn split_at_mut(&mut self, mid: usize) -> (&mut [T], &mut [T]) {
        if mid > self.len() {
            dont_panic!("index {} out of range for slice of length {}", mid, self.len());
        }

        Self::as_rust_slice_mut(self).split_at_mut(mid)
    }
}

impl<'a, T> From<&'a [T]> for &'a DPSlice<T> {
    fn from(slice: &[T]) -> Self {
        unsafe { ::core::mem::transmute(slice) }
    }
}

/* Coherence :'(
impl<'a, T> From<&'a DPSlice<T>> for &'a [T] {
    fn from(slice: &DPSlice<T>) -> Self {
        unsafe { ::core::mem::transmute(slice) }
    }
}
*/

impl<'a, T> From<&'a mut [T]> for &'a mut DPSlice<T> {
    fn from(slice: &mut [T]) -> Self {
        unsafe { ::core::mem::transmute(slice) }
    }
}

/* Coherence :'(
impl<'a, T> From<&'a mut DPSlice<T>> for &'a mut [T] {
    fn from(slice: &mut DPSlice<T>) -> Self {
        unsafe { ::core::mem::transmute(slice) }
    }
}
*/

impl<T> ::core::ops::Index<usize> for DPSlice<T> {
    type Output = T;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        Self::as_rust_slice(self).get(index).unwrap_or_else(|| dont_panic!("index out of bounds: the len is {} but the index is {}", self.len(), index))
    }
}

impl<T> ::core::ops::IndexMut<usize> for DPSlice<T> {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        Self::as_rust_slice_mut(self).get_mut(index).unwrap_or_else(|| dont_panic!("index out of bounds: the len is {} but the index is {}", self.len(), index))
    }
}

#[cfg(test)]
mod tests {
    use ::DPSlice;

    #[test]
    fn it_works() {
        let arr = [0, 1, 2, 3];
        let dps = <&DPSlice<_>>::from(&arr as &[_]);
        assert_eq!(dps[0], 0);
        assert_eq!(dps[3], 3);
    }

    #[cfg(feature = "panic")]
    #[test]
    #[should_panic]
    fn panic() {
        let arr = [0, 1, 2, 3];
        let dps = <&DPSlice<_>>::from(&arr as &[_]);
        assert_eq!(dps[42], 0);
        assert_eq!(dps[3], 3);
    }

    #[cfg(feature = "panic")]
    #[test]
    fn no_panic() {
        let arr = [0, 1, 2, 3];
        let dps = <&DPSlice<_>>::from(&arr as &[_]);
        assert_eq!(dps[0], 0);
        assert_eq!(dps[3], 3);
    }
}
