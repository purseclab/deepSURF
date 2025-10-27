use std::ptr;
pub trait VecExt<T> {
    fn push_option(&mut self, o: Option<T>);
    fn grow_to_with<F: FnMut() -> T>(&mut self, size: usize, f: F);
    fn grow_to(&mut self, size: usize, value: T)
    where
        T: Clone;
    fn grow_to_default(&mut self, size: usize)
    where
        T: Default;
    fn insert_slice_copy(&mut self, index: usize, slice: &[T])
    where
        T: Copy;
    fn insert_slice_clone(&mut self, index: usize, slice: &[T])
    where
        T: Clone;
    fn extend_from_slice_copy(&mut self, slice: &[T])
    where
        T: Copy;
}
impl<T> VecExt<T> for Vec<T> {
    #[inline]
    fn push_option(&mut self, o: Option<T>) {
        if let Some(o) = o {
            self.push(o);
        }
    }
    #[inline]
    fn grow_to_with<F: FnMut() -> T>(&mut self, size: usize, f: F) {
        if size > self.len() {
            self.resize_with(size, f);
        }
    }
    #[inline]
    fn grow_to(&mut self, size: usize, value: T)
    where
        T: Clone,
    {
        if size > self.len() {
            self.resize(size, value);
        }
    }
    #[inline]
    fn grow_to_default(&mut self, size: usize)
    where
        T: Default,
    {
        if size > self.len() {
            self.resize_with(size, T::default);
        }
    }
    #[inline]
    fn insert_slice_copy(&mut self, index: usize, slice: &[T])
    where
        T: Copy,
    {
        let vlen = self.len();
        let slen = slice.len();
        assert!(index <= vlen);
        assert!(slice.len() <= isize::MAX as usize);
        let dlen = vlen + slen;
        if dlen > self.capacity() {
            self.reserve(slice.len());
        }
        unsafe {
            {
                let s = slice.as_ptr();
                let p = self.as_mut_ptr().add(index);
                ptr::copy(p, p.add(slen), vlen - index);
                ptr::copy_nonoverlapping(s, p, slen);
            }
            self.set_len(dlen);
        }
    }
    #[inline]
    fn insert_slice_clone(&mut self, index: usize, slice: &[T])
    where
        T: Clone,
    {
        let vlen = self.len();
        let slen = slice.len();
        assert!(index <= vlen);
        assert!(slice.len() <= isize::MAX as usize);
        let dlen = vlen + slen;
        if dlen > self.capacity() {
            self.reserve(slice.len());
        }
        unsafe {
            {
                let mut p = self.as_mut_ptr().add(index);
                ptr::copy(p, p.add(slen), vlen - index);
                for v in slice {
                    ptr::write(p, v.clone());
                    p = p.offset(1);
                }
            }
            self.set_len(dlen);
        }
    }
    fn extend_from_slice_copy(&mut self, slice: &[T])
    where
        T: Copy,
    {
        self.insert_slice_copy(self.len(), slice);
    }
}
#[test]
fn insert_extra() {
    let mut a = vec![1, 2, 7, 8];
    let b = [3, 4, 5, 6];
    a.insert_slice_copy(2, &b);
    a.extend_from_slice_copy(&[9, 10, 11, 12, 13, 14, 15, 16]);
    assert_eq!(& a,& [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
    assert_eq!(a.len(), 16);
}
#[test]
fn insert_extra_b() {
    let mut a = vec![1, 2, 7, 8];
    let b = [3, 4, 5, 6];
    a.insert_slice_clone(2, &b);
    assert_eq!(& a,& [1, 2, 3, 4, 5, 6, 7, 8]);
    assert_eq!(a.len(), 8);
}
#[cfg(test)]
mod tests_rug_60 {
    use super::*;
    use crate::imp::VecExt;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut v31: std::vec::Vec<i32> = std::vec::Vec::new();
        let o: std::option::Option<i32> = Some(rug_fuzz_0);
        v31.push_option(o);
             }
});    }
}
#[cfg(test)]
mod tests_rug_61 {
    use super::*;
    use crate::imp::VecExt;
    use crate::arc_slice::ArcSlice;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut v31: std::vec::Vec<i32> = std::vec::Vec::new();
        let size: usize = rug_fuzz_0;
        let mut f = || rug_fuzz_1;
        v31.grow_to_with(size, f);
             }
});    }
}
#[cfg(test)]
mod tests_rug_63 {
    use super::*;
    use crate::imp::VecExt;
    use crate::arc_slice::ArcSlice;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut v31: std::vec::Vec<i32> = std::vec::Vec::new();
        let size: usize = rug_fuzz_0;
        v31.grow_to_default(size);
             }
});    }
}
#[cfg(test)]
mod tests_rug_64 {
    use super::*;
    use crate::imp::VecExt;
    use std::ptr;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(usize, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut v31: std::vec::Vec<i32> = std::vec::Vec::new();
        let index: usize = rug_fuzz_0;
        let slice: &[i32] = &[rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        v31.insert_slice_copy(index, slice);
             }
});    }
}
#[cfg(test)]
mod tests_rug_65 {
    use super::*;
    use crate::imp::VecExt;
    use crate::arc_slice::ArcSlice;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(i32, i32, i32, usize, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut v31: std::vec::Vec<i32> = std::vec::Vec::new();
        v31.push(rug_fuzz_0);
        v31.push(rug_fuzz_1);
        v31.push(rug_fuzz_2);
        let index: usize = rug_fuzz_3;
        let slice_data = [rug_fuzz_4, rug_fuzz_5, rug_fuzz_6];
        let slice: &[i32] = &slice_data;
        <std::vec::Vec<i32>>::insert_slice_clone(&mut v31, index, slice);
        debug_assert_eq!(v31, vec![1, 4, 5, 6, 2, 3]);
             }
});    }
}
#[cfg(test)]
mod tests_rug_66 {
    use super::*;
    use crate::imp::VecExt;
    use crate::arc_slice::ArcSlice;
    #[test]
    fn test_extend_from_slice_copy() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(i32, i32, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: std::vec::Vec<i32> = std::vec::Vec::new();
        p0.push(rug_fuzz_0);
        p0.push(rug_fuzz_1);
        let slice: &[i32] = &[rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        p0.extend_from_slice_copy(slice);
        debug_assert_eq!(p0, vec![1, 2, 3, 4, 5]);
             }
});    }
}
