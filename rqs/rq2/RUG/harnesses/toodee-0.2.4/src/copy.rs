use core::cmp::Ordering;
use crate::toodee::*;
use crate::view::*;
use crate::ops::*;
/// Provides basic copying operations for `TooDee` structures.
pub trait CopyOps<T>: TooDeeOpsMut<T> {
    /// Copies data from another slice into this area. The source slice's length
    /// must match the size of this object's area. Data is copied row by row.
    ///
    /// # Examples
    ///
    /// ```
    /// use toodee::{TooDee,TooDeeOps,TooDeeOpsMut,CopyOps};
    /// let ascending = vec![0, 1, 2, 3, 4];
    /// let mut toodee : TooDee<u32> = TooDee::new(10, 5);
    /// toodee.view_mut((5, 1), (10, 2)).copy_from_slice(&ascending);
    /// ```
    fn copy_from_slice(&mut self, src: &[T])
    where
        T: Copy,
    {
        let cols = self.num_cols();
        assert_eq!(cols * self.num_rows(), src.len());
        for (d, s) in self.rows_mut().zip(src.chunks_exact(cols)) {
            d.copy_from_slice(s)
        }
    }
    /// Clones data from another slice into this area. The source slice's length
    /// must match the size of this object's area. Data is cloned row by row.
    ///
    /// # Examples
    ///
    /// ```
    /// use toodee::{TooDee,TooDeeOps,TooDeeOpsMut,CopyOps};
    /// let ascending = vec![0, 1, 2, 3, 4];
    /// let mut toodee : TooDee<u32> = TooDee::new(10, 5);
    /// toodee.view_mut((5, 1), (10, 2)).clone_from_slice(&ascending);
    /// ```
    fn clone_from_slice(&mut self, src: &[T])
    where
        T: Clone,
    {
        let cols = self.num_cols();
        assert_eq!(cols * self.num_rows(), src.len());
        for (d, s) in self.rows_mut().zip(src.chunks_exact(cols)) {
            d.clone_from_slice(s)
        }
    }
    /// Copies data from another `TooDeeOps` object into this one. The source and
    /// destination dimensions must match.
    ///
    /// # Examples
    ///
    /// ```
    /// use toodee::{TooDee,TooDeeOps,TooDeeOpsMut,CopyOps};
    /// let ascending = TooDee::from_vec(5, 1, vec![0, 1, 2, 3, 4]);
    /// let mut toodee : TooDee<u32> = TooDee::new(10, 5);
    /// toodee.view_mut((5, 1), (10, 2)).copy_from_toodee(&ascending);
    /// ```
    fn copy_from_toodee(&mut self, src: &impl TooDeeOps<T>)
    where
        T: Copy,
    {
        assert_eq!(self.size(), src.size());
        for (d, s) in self.rows_mut().zip(src.rows()) {
            d.copy_from_slice(s);
        }
    }
    /// Copies data from another `TooDeeOps` object into this one. The source and
    /// destination dimensions must match.
    ///
    /// # Examples
    ///
    /// ```
    /// use toodee::{TooDee,TooDeeOps,TooDeeOpsMut,CopyOps};
    /// let ascending = TooDee::from_vec(5, 1, vec![0, 1, 2, 3, 4]);
    /// let mut toodee : TooDee<u32> = TooDee::new(10, 5);
    /// toodee.view_mut((5, 1), (10, 2)).clone_from_toodee(&ascending);
    /// ```
    fn clone_from_toodee(&mut self, src: &impl TooDeeOps<T>)
    where
        T: Clone,
    {
        assert_eq!(self.size(), src.size());
        for (d, s) in self.rows_mut().zip(src.rows()) {
            d.clone_from_slice(s);
        }
    }
    /// Copies the `src` area (top-left to bottom-right) to a destination area. `dest` specifies
    /// the top-left position of destination area. The `src` area will be partially overwritten
    /// if the regions overlap.
    ///
    /// # Panics
    ///
    /// Panics if:
    /// - `src` dimensions are outside the array's bounds
    /// - there's insufficient room to copy all of `src` to `dest`
    ///
    /// # Examples
    ///
    /// ```
    /// use toodee::{TooDee,TooDeeOps,TooDeeOpsMut,CopyOps};
    /// let mut toodee : TooDee<u32> = TooDee::new(10, 5);
    /// toodee.view_mut((0, 0), (5, 1)).fill(42);
    /// assert_eq!(toodee[(3,1)], 0);
    /// toodee.copy_within(((0, 0), (5, 1)), (0, 1));
    /// assert_eq!(toodee[(3,1)], 42);
    /// ```
    fn copy_within(&mut self, src: (Coordinate, Coordinate), dest: Coordinate)
    where
        T: Copy,
    {
        let (top_left, bottom_right) = src;
        assert!(top_left.0 <= bottom_right.0);
        assert!(top_left.1 <= bottom_right.1);
        let num_cols = self.num_cols();
        let num_rows = self.num_rows();
        assert!(bottom_right.0 <= num_cols);
        assert!(bottom_right.1 <= num_rows);
        let cols = bottom_right.0 - top_left.0;
        let rows = bottom_right.1 - top_left.1;
        assert!(dest.0 + cols <= num_cols);
        assert!(dest.1 + rows <= num_rows);
        match top_left.1.cmp(&dest.1) {
            Ordering::Less => {
                let row_offset = dest.1 - top_left.1;
                for r in (top_left.1..bottom_right.1).rev() {
                    let (s, d) = self.row_pair_mut(r, r + row_offset);
                    d[dest.0..dest.0 + cols]
                        .copy_from_slice(&s[top_left.0..bottom_right.0]);
                }
            }
            Ordering::Greater => {
                let row_offset = top_left.1 - dest.1;
                for r in top_left.1..bottom_right.1 {
                    let (s, d) = self.row_pair_mut(r, r - row_offset);
                    d[dest.0..dest.0 + cols]
                        .copy_from_slice(&s[top_left.0..bottom_right.0]);
                }
            }
            Ordering::Equal => {
                for r in top_left.1..bottom_right.1 {
                    let row_data = &mut self[r];
                    row_data.copy_within(top_left.0..bottom_right.0, dest.0);
                }
            }
        }
    }
}
impl<T> CopyOps<T> for TooDeeViewMut<'_, T> {}
impl<T> CopyOps<T> for TooDee<T> {
    fn copy_from_slice(&mut self, src: &[T])
    where
        T: Copy,
    {
        self.data_mut().copy_from_slice(src);
    }
    fn clone_from_slice(&mut self, src: &[T])
    where
        T: Clone,
    {
        self.data_mut().clone_from_slice(src);
    }
    fn copy_from_toodee(&mut self, src: &impl TooDeeOps<T>)
    where
        T: Copy,
    {
        assert_eq!(self.size(), src.size());
        let num_cols = self.num_cols();
        let mut v = self.data_mut();
        for r in src.rows() {
            let (fst, snd) = v.split_at_mut(num_cols);
            fst.copy_from_slice(r);
            v = snd;
        }
    }
    fn clone_from_toodee(&mut self, src: &impl TooDeeOps<T>)
    where
        T: Clone,
    {
        assert_eq!(self.size(), src.size());
        let num_cols = self.num_cols();
        let mut v = self.data_mut();
        for r in src.rows() {
            let (fst, snd) = v.split_at_mut(num_cols);
            fst.clone_from_slice(r);
            v = snd;
        }
    }
}
#[cfg(test)]
mod tests_rug_107 {
    use super::*;
    use crate::{TooDee, TooDeeOps, TooDeeOpsMut, CopyOps};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10)) = <(usize, usize, u32, u32, u32, u32, u32, usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: TooDee<u32> = TooDee::init(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let p1 = &[rug_fuzz_3, rug_fuzz_4, rug_fuzz_5, rug_fuzz_6];
        p0.view_mut((rug_fuzz_7, rug_fuzz_8), (rug_fuzz_9, rug_fuzz_10))
            .copy_from_slice(p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_108 {
    use super::*;
    use crate::{TooDee, TooDeeOps, CopyOps};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12)) = <(usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: TooDee<u32> = TooDee::init(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let p1: &[u32] = &[
            rug_fuzz_3,
            rug_fuzz_4,
            rug_fuzz_5,
            rug_fuzz_6,
            rug_fuzz_7,
            rug_fuzz_8,
            rug_fuzz_9,
            rug_fuzz_10,
            rug_fuzz_11,
            rug_fuzz_12,
        ];
        p0.clone_from_slice(p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_109 {
    use super::*;
    use crate::{TooDee, TooDeeOps, CopyOps};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(usize, usize, u32, usize, usize, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: TooDee<u32> = TooDee::init(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let mut p1: TooDee<u32> = TooDee::init(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        p0.copy_from_toodee(&p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_110 {
    use super::*;
    use crate::{TooDee, TooDeeOps, CopyOps};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(usize, usize, u32, usize, usize, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: TooDee<u32> = TooDee::init(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let mut p1: TooDee<u32> = TooDee::init(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        p0.clone_from_toodee(&p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_111 {
    use super::*;
    use crate::{TooDee, TooDeeOps, TooDeeOpsMut, CopyOps};
    #[test]
    fn test_copy_within() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17, mut rug_fuzz_18, mut rug_fuzz_19, mut rug_fuzz_20, mut rug_fuzz_21)) = <(usize, usize, u32, usize, usize, u32, usize, usize, u32, usize, usize, u32, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: TooDee<u32> = TooDee::init(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        p0[(rug_fuzz_3, rug_fuzz_4)] = rug_fuzz_5;
        p0[(rug_fuzz_6, rug_fuzz_7)] = rug_fuzz_8;
        p0[(rug_fuzz_9, rug_fuzz_10)] = rug_fuzz_11;
        let p1 = ((rug_fuzz_12, rug_fuzz_13), (rug_fuzz_14, rug_fuzz_15));
        let p2 = (rug_fuzz_16, rug_fuzz_17);
        p0.copy_within(p1, p2);
        debug_assert_eq!(p0[(rug_fuzz_18, rug_fuzz_19)], 1);
        debug_assert_eq!(p0[(rug_fuzz_20, rug_fuzz_21)], 2);
             }
});    }
}
#[cfg(test)]
mod tests_rug_114 {
    use super::*;
    use crate::CopyOps;
    use crate::{TooDee, TooDeeOps};
    #[test]
    fn test_copy_from_toodee() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(usize, usize, u32, usize, usize, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut src: TooDee<u32> = TooDee::init(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let mut dest: TooDee<u32> = TooDee::init(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        dest.copy_from_toodee(&src);
             }
});    }
}
#[cfg(test)]
mod tests_rug_115 {
    use super::*;
    use crate::copy::CopyOps;
    use crate::{TooDee, TooDeeOps};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(usize, usize, u32, usize, usize, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: TooDee<u32> = TooDee::init(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let mut p1: TooDee<u32> = TooDee::init(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        p0.clone_from_toodee(&p1);
             }
});    }
}
