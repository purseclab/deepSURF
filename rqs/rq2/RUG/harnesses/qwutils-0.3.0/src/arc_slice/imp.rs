use super::*;
use std::ops::RangeBounds;
impl<T> ArcSlice<T> {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Vec::new()),
            slice: 0..0,
        }
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: Arc::new(Vec::with_capacity(capacity)),
            slice: 0..0,
        }
    }
    /// slice of the current slice
    ///
    /// will not allocate
    #[inline]
    pub fn slice<S>(&self, range: S) -> Self
    where
        S: RangeBounds<usize>,
    {
        Self {
            inner: self.inner.refc(),
            slice: slice_slice(&self.slice, range),
        }
    }
    /// will always allocate and clone
    pub fn extract(&self) -> Vec<T>
    where
        T: Clone,
    {
        self.extract_with_capacity(self.len())
    }
    pub fn extracted(&self) -> Self
    where
        T: Clone,
    {
        self.extracted_with_capacity(self.len())
    }
    pub fn extract_with_capacity(&self, capacity: usize) -> Vec<T>
    where
        T: Clone,
    {
        let mut dest = Vec::with_capacity(self.len().max(capacity));
        dest.extend_from_slice(&self);
        dest
    }
    pub fn extracted_with_capacity(&self, capacity: usize) -> Self
    where
        T: Clone,
    {
        if self.is_unsliced() {
            self.refc()
        } else {
            Self::from(self.extract_with_capacity(capacity))
        }
    }
    #[inline]
    pub fn len(&self) -> usize {
        assert!(self.slice.end >= self.slice.start);
        assert!(self.slice.end <= self.inner.len());
        let len = self.slice.end - self.slice.start;
        assert_eq!(len, self[..].len());
        len
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        assert!(self.slice.end >= self.slice.start);
        assert!(self.slice.end <= self.inner.len());
        self.slice.start == self.slice.end
    }
    /// whether this slice vievs the complete backing vec
    #[inline]
    pub fn is_unsliced(&self) -> bool {
        self.slice.start == 0 && self.slice.end == self.len()
    }
    /// Minimize memory usage.
    /// This will only work for unique ArcSlices and then would reallocate and move the contents.
    pub fn compact(&mut self) -> bool {
        if let Some(e) = Arc::get_mut(&mut self.inner) {
            if self.slice.start == 0 {
                e.truncate(self.slice.end);
                e.shrink_to_fit();
            } else {
                let mut dest = Vec::new();
                dest.reserve_exact(e.len());
                dest.append(e);
                assert_eq!(dest.capacity(), dest.len());
                self.slice = 0..dest.len();
                *e = dest;
            }
            true
        } else {
            false
        }
    }
    pub fn truncate(&mut self, len: usize) {
        if len == 0 {
            self.slice = 0..0;
        } else {
            self.slice.end = self.len().min(self.slice.start + len);
        }
    }
    pub fn swap_remove(&mut self, index: usize) -> T
    where
        T: Clone,
    {
        assert!(index < self.len(), "swap_remove out of bounds");
        if index == self.len() - 1 {
            self.remove(index)
        } else if let Some(e) = Arc::get_mut(&mut self.inner) {
            e.truncate(self.slice.end);
            self.slice.end -= 1;
            e.swap_remove(self.slice.start + index)
        } else {
            let origin = &self[..];
            let mut dest = Vec::with_capacity(self.len() - 1);
            let left = &origin[..index];
            dest.extend_from_slice(left);
            let removed = origin[index].clone();
            let right = &origin[(index + 1)..(origin.len() - 1)];
            let end = origin.last().unwrap().clone();
            dest.push(end);
            dest.extend_from_slice(right);
            *self = Self::from(dest);
            removed
        }
    }
    pub fn remove(&mut self, index: usize) -> T
    where
        T: Clone,
    {
        assert!(index < self.len(), "remove out of bounds");
        if let Some(e) = Arc::get_mut(&mut self.inner) {
            e.truncate(self.slice.end);
            self.slice.end -= 1;
            e.remove(self.slice.start + index)
        } else {
            let origin = &self[..];
            let mut dest = Vec::with_capacity(self.len() - 1);
            let left = &origin[..index];
            dest.extend_from_slice(left);
            let removed = origin[index].clone();
            let right = &origin[(index + 1)..];
            dest.extend_from_slice(right);
            *self = Self::from(dest);
            removed
        }
    }
    pub fn retain<F>(&mut self, mut f: F)
    where
        T: Clone,
        F: FnMut(&T) -> bool,
    {
        if let Some(e) = Arc::get_mut(&mut self.inner) {
            e.truncate(self.slice.end);
            e.retain(f);
            self.slice.end = self.slice.start + e.len();
        } else {
            let origin = &self[..];
            let mut dest = Vec::with_capacity(self.len());
            for v in origin {
                if f(v) {
                    dest.push(v.clone());
                }
            }
            *self = Self::from(dest);
        }
    }
    pub fn insert(&mut self, index: usize, element: T)
    where
        T: Clone,
    {
        assert!(index <= self.len(), "insert out of bounds");
        if let Some(e) = Arc::get_mut(&mut self.inner) {
            e.truncate(self.slice.end);
            e.insert(self.slice.start + index, element);
            self.slice.end += 1;
        } else {
            let origin = &self[..];
            let mut dest = Vec::with_capacity(self.len() + 1);
            let (left, right) = origin.split_at(index);
            dest.extend_from_slice(left);
            dest.push(element);
            dest.extend_from_slice(right);
            *self = Self::from(dest);
        }
    }
    pub fn insert_slice(&mut self, index: usize, s: &[T])
    where
        T: Clone,
    {
        assert!(index <= self.len(), "insert out of bounds");
        if s.is_empty() {
            return;
        }
        if let Some(e) = Arc::get_mut(&mut self.inner) {
            e.truncate(self.slice.end);
            e.insert_slice_clone(self.slice.start + index, s);
            self.slice.end += s.len();
        } else {
            let origin = &self[..];
            let mut dest = Vec::with_capacity(self.len() + s.len());
            let (left, right) = origin.split_at(index);
            dest.extend_from_slice(left);
            dest.extend_from_slice(s);
            dest.extend_from_slice(right);
            *self = Self::from(dest);
        }
    }
    pub fn split_at(&mut self, at: usize) -> (Self, Self) {
        assert!(at <= self.len(), "`at` out of bounds");
        let start = self.slice(..at);
        let end = self.slice(at..);
        (start, end)
    }
    pub fn split_off(&mut self, at: usize) -> Self {
        let (start, end) = self.split_at(at);
        *self = start;
        end
    }
    pub fn resize_with<F>(&mut self, new_len: usize, mut f: F)
    where
        T: Clone,
        F: FnMut() -> T,
    {
        if new_len > self.len() {
            let (vec, slice) = self._make_mut_with_capacity(self.len() + 1);
            vec.truncate(slice.end);
            vec.reserve((slice.start + new_len).saturating_sub(vec.capacity()));
            for _ in slice.len()..new_len {
                vec.push(f());
            }
            self.slice.end = slice.start + new_len;
        } else {
            self.truncate(new_len);
        }
    }
    pub fn resize<F>(&mut self, new_len: usize, value: T)
    where
        T: Clone,
    {
        self.resize_with(new_len, move || value.clone())
    }
    pub fn resize_default<F>(&mut self, new_len: usize)
    where
        T: Clone + Default,
    {
        self.resize_with(new_len, || T::default())
    }
    pub fn push(&mut self, v: T)
    where
        T: Clone,
    {
        self.insert(self.len(), v)
    }
    pub fn pop(&mut self) -> Option<T>
    where
        T: Clone,
    {
        (self.slice.end > self.slice.start).map(|| { self.remove(self.len() - 1) })
    }
    pub fn append(&mut self, other: &mut Vec<T>)
    where
        T: Clone,
    {
        if !other.is_empty() {
            let other_len = other.len();
            let (vec, slice) = self._make_mut_with_capacity(self.len() + other_len);
            vec.truncate(slice.end);
            vec.append(other);
            self.slice.end += other_len;
        }
    }
    pub fn extend_from_slice(&mut self, other: &[T])
    where
        T: Clone,
    {
        if !other.is_empty() {
            let other_len = other.len();
            let (vec, slice) = self._make_mut_with_capacity(self.len() + other_len);
            vec.truncate(slice.end);
            vec.extend_from_slice(other);
            self.slice.end += other_len;
        }
    }
    pub fn clear(&mut self) {
        self.truncate(0);
    }
    /// mutably access the mutable Vec inside
    /// Note that self.slice will eventually mutate
    pub fn _make_mut(&mut self) -> (&mut Vec<T>, &mut Range<usize>)
    where
        T: Clone,
    {
        self._make_mut_with_capacity(self.len())
    }
    pub fn _make_mut_with_capacity(
        &mut self,
        capacity: usize,
    ) -> (&mut Vec<T>, &mut Range<usize>)
    where
        T: Clone,
    {
        if Arc::get_mut(&mut self.inner).is_some() {
            (Arc::make_mut(&mut self.inner), &mut self.slice)
        } else {
            *self = self.extracted_with_capacity(capacity);
            Arc::make_mut(&mut self.inner);
            assert!(self.is_unsliced());
            self._make_mut_with_capacity(capacity)
        }
    }
    pub fn _make_mut_extracted(&mut self) -> &mut Vec<T>
    where
        T: Clone,
    {
        self._make_mut_extracted_with_capacity(self.len())
    }
    pub fn _make_mut_extracted_with_capacity(&mut self, capacity: usize) -> &mut Vec<T>
    where
        T: Clone,
    {
        *self = self.extracted_with_capacity(capacity);
        assert!(self.is_unsliced());
        self._make_mut_with_capacity(capacity).0
    }
}
impl<T> Default for ArcSlice<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T> From<Vec<T>> for ArcSlice<T> {
    fn from(v: Vec<T>) -> Self {
        Self::from(Arc::new(v))
    }
}
impl<T> From<Arc<Vec<T>>> for ArcSlice<T> {
    fn from(v: Arc<Vec<T>>) -> Self {
        Self {
            slice: 0..v.len(),
            inner: v,
        }
    }
}
#[inline]
fn slice_slice<S>(range: &Range<usize>, slice: S) -> Range<usize>
where
    S: RangeBounds<usize>,
{
    let (os, oe) = (range.start, range.end);
    let (mut s, mut e) = (os, oe);
    match slice.end_bound() {
        std::ops::Bound::Included(&b) => e = oe.min(b + 1 + os),
        std::ops::Bound::Excluded(&b) => e = oe.min(b + os),
        std::ops::Bound::Unbounded => {}
    }
    match slice.start_bound() {
        std::ops::Bound::Included(&b) => s = os.max(b + os),
        std::ops::Bound::Excluded(&b) => s = os.max(b - 1 + os),
        std::ops::Bound::Unbounded => {}
    }
    assert!(
        s >= os && s <= oe && e >= os && e <= oe && e >= s, "Inner slice out of bounds"
    );
    s..e
}
#[test]
fn ultrion() {
    let mut a = ArcSlice::from(&b"abcd"[..]);
    assert_eq!(a,& b"abcd"[..]);
    assert_eq!(& a[..],& b"abcd"[..]);
    assert_eq!(& a[2..],& b"cd"[..]);
    assert_eq!(& a[1..3],& b"bc"[..]);
    assert_eq!(& a[2..2],& b""[..]);
    a.extend_from_slice(&b"gh"[..]);
    a.insert_slice(4, &b"ef"[..]);
    assert_eq!(& a[..],& b"abcdefgh"[..]);
    let mut b = a.slice(2..4);
    assert_eq!(& b[..],& b"cd"[..]);
    b.extend_from_slice(&b"io"[..]);
    assert_eq!(& b[..],& b"cdio"[..]);
    assert_eq!(& a[..],& b"abcdefgh"[..]);
    a.remove(2);
    assert_eq!(& a[..],& b"abdefgh"[..]);
    a.swap_remove(2);
    assert_eq!(& a[..],& b"abhefg"[..]);
    let mut c = a.refc();
    c.push(b'f');
    c.retain(|&c| c == b'h' || c == b'f');
    assert_eq!(& c[..],& b"hff"[..]);
    assert_eq!(& a[..],& b"abhefg"[..]);
    a.pop();
    assert_eq!(& a[..],& b"abhef"[..]);
}
#[cfg(test)]
mod tests_rug_24 {
    use super::*;
    use std::ops::Range;
    use std::ops::Bound;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: Range<usize> = rug_fuzz_0..rug_fuzz_1;
        let mut p1 = (Bound::Included(&rug_fuzz_2), Bound::Excluded(&rug_fuzz_3));
        crate::arc_slice::imp::slice_slice(&p0, p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_25 {
    use super::*;
    use arc_slice::ArcSlice;
    use std::sync::Arc;
    #[test]
    fn test_new() {
        let _rug_st_tests_rug_25_rrrruuuugggg_test_new = 0;
        let new_arc_slice = ArcSlice::<u32>::new();
        debug_assert_eq!(new_arc_slice.inner.len(), 0);
        debug_assert_eq!(new_arc_slice.slice.start, 0);
        debug_assert_eq!(new_arc_slice.slice.end, 0);
        let _rug_ed_tests_rug_25_rrrruuuugggg_test_new = 0;
    }
}
#[cfg(test)]
mod tests_rug_28 {
    use super::*;
    use arc_slice::ArcSlice;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = vec![rug_fuzz_0, 2, 3];
        let arc_slice = ArcSlice::from(data);
        ArcSlice::<i32>::extract(&arc_slice);
             }
});    }
}
#[cfg(test)]
mod tests_rug_33 {
    use super::*;
    #[test]
    fn test_is_empty() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = arc_slice::ArcSlice {
            inner: Arc::new(vec![rug_fuzz_0, 2, 3]),
            slice: Range {
                start: rug_fuzz_1,
                end: rug_fuzz_2,
            },
        };
        debug_assert!(p0.is_empty());
             }
});    }
}
#[cfg(test)]
mod tests_rug_35 {
    use super::*;
    use arc_slice::{ArcSlice, Arc};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut inner = vec![rug_fuzz_0, 2, 3];
        let slice = rug_fuzz_1..rug_fuzz_2;
        let mut arc_slice: ArcSlice<i32> = ArcSlice {
            inner: Arc::new(inner),
            slice,
        };
        arc_slice.compact();
             }
});    }
}
#[cfg(test)]
mod tests_rug_37 {
    use super::*;
    use crate::arc_slice::ArcSlice;
    #[test]
    fn test_swap_remove() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = ArcSlice {
            inner: Arc::new(vec![rug_fuzz_0, 2, 3]),
            slice: rug_fuzz_1..rug_fuzz_2,
        };
        let p1: usize = rug_fuzz_3;
        p0.swap_remove(p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_38 {
    use super::*;
    use crate::arc_slice::ArcSlice;
    use crate::arc_slice::ArcSlice as OtherArcSlice;
    #[test]
    fn test_remove() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut data = vec![rug_fuzz_0, 2, 3, 4, 5];
        let mut arc_slice = OtherArcSlice::from(data.clone());
        let index = rug_fuzz_1;
        ArcSlice::<i32>::remove(&mut arc_slice, index);
             }
});    }
}
#[cfg(test)]
mod tests_rug_40 {
    use super::*;
    use arc_slice::ArcSlice;
    use std::sync::Arc;
    use std::ops::Range;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(i32, usize, usize, usize, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let v26: ArcSlice<i32> = ArcSlice {
            inner: Arc::new(vec![rug_fuzz_0, 2, 3]),
            slice: rug_fuzz_1..rug_fuzz_2,
        };
        let mut index: usize = rug_fuzz_3;
        let element_to_insert = Some(rug_fuzz_4);
        let mut p0 = v26;
        let p1 = index;
        let p2 = element_to_insert.unwrap();
        p0.insert(p1, p2);
             }
});    }
}
#[cfg(test)]
mod tests_rug_41 {
    use super::*;
    use arc_slice::ArcSlice;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, usize, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = ArcSlice::<i32>::from(vec![rug_fuzz_0, 2, 3, 4]);
        let p1: usize = rug_fuzz_1;
        let p2: &[i32] = &[rug_fuzz_2, rug_fuzz_3];
        p0.insert_slice(p1, p2);
             }
});    }
}
#[cfg(test)]
mod tests_rug_43 {
    use super::*;
    use arc_slice::ArcSlice;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut data: Vec<i32> = vec![rug_fuzz_0, 2, 3, 4, 5];
        let at: usize = rug_fuzz_1;
        let mut arc_slice_data = ArcSlice::from(data);
        let result = <ArcSlice<i32>>::split_off(&mut arc_slice_data, at);
        debug_assert_eq!(arc_slice_data.as_ref(), & [1, 2]);
        debug_assert_eq!(result.as_ref(), & [3, 4, 5]);
             }
});    }
}
#[cfg(test)]
mod tests_rug_47 {
    use super::*;
    use arc_slice::ArcSlice;
    use std::sync::Arc;
    use std::ops::Range;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, usize, usize, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let v26: ArcSlice<i32> = ArcSlice {
            inner: Arc::new(vec![rug_fuzz_0, 2, 3]),
            slice: rug_fuzz_1..rug_fuzz_2,
        };
        let mut p0: arc_slice::ArcSlice<i32> = v26;
        let mut p1: i32 = rug_fuzz_3;
        p0.push(p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_49 {
    use super::*;
    use crate::arc_slice::ArcSlice;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_49_rrrruuuugggg_test_rug = 0;
        let mut v31: std::vec::Vec<i32> = std::vec::Vec::new();
        let mut p0 = ArcSlice::<i32>::new();
        let mut p1 = &mut v31;
        p0.append(p1);
        let _rug_ed_tests_rug_49_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_51 {
    use super::*;
    use arc_slice::ArcSlice;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: ArcSlice<i32> = ArcSlice::from(vec![rug_fuzz_0, 2, 3]);
        <arc_slice::ArcSlice<i32>>::clear(&mut p0);
        debug_assert_eq!(p0.len(), 0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_53 {
    use super::*;
    use arc_slice::{ArcSlice, Arc};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut inner = vec![rug_fuzz_0, 2, 3];
        let slice = rug_fuzz_1..rug_fuzz_2;
        let mut arc_slice = ArcSlice {
            inner: Arc::new(inner),
            slice,
        };
        let capacity: usize = rug_fuzz_3;
        let (mut p0, mut p1) = arc_slice._make_mut_with_capacity(capacity);
             }
});    }
}
#[cfg(test)]
mod tests_rug_56 {
    use super::*;
    use crate::arc_slice::ArcSlice;
    use std::default::Default;
    #[test]
    fn test_default() {
        let _rug_st_tests_rug_56_rrrruuuugggg_test_default = 0;
        let default_value: ArcSlice<i32> = ArcSlice::<i32>::default();
        let _rug_ed_tests_rug_56_rrrruuuugggg_test_default = 0;
    }
}
