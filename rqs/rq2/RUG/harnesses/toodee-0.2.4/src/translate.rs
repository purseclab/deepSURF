use crate::ops::*;
/// Provides implementations for translate (also known as scroll) operations, and other internal data
/// movement operations such as flipping.
pub trait TranslateOps<T>: TooDeeOpsMut<T> {
    /// Translate (or scroll) the entire area. The `mid` coordinate will be moved to (0, 0), and
    /// all other elements will be moved in the same fashion. All the original data is preserved by
    /// wrapping at the array edges.
    ///
    /// If you don't want the wrapped data, simply overwrite it after translation.
    ///
    /// # Examples
    ///
    /// ```
    /// use toodee::{TooDee,TooDeeOps,TranslateOps};
    /// let v = vec![42u32; 15];
    /// let mut toodee : TooDee<u32> = TooDee::from_vec(5, 3, v);
    /// toodee[1][1] = 1;
    /// // move (1, 1) to (0, 0)
    /// toodee.translate_with_wrap((1, 1));
    /// assert_eq!(toodee[0][0], 1);
    /// assert_eq!(toodee[1][1], 42);
    /// ```
    ///
    /// ```
    /// use toodee::{TooDee,TooDeeOps,TranslateOps};
    /// let v = vec![42u32; 15];
    /// let mut toodee : TooDee<u32> = TooDee::from_vec(5, 3, v);
    /// // set (4, 2) to 1
    /// toodee[(4, 2)] = 1;
    /// // move (4, 2) to (0, 0)
    /// toodee.translate_with_wrap((4, 2));
    /// assert_eq!(toodee[0][0], 1);
    /// assert_eq!(toodee[2][4], 42);
    /// ```
    fn translate_with_wrap(&mut self, mid: Coordinate) {
        let (mut col_mid, mut row_mid) = mid;
        let num_cols = self.num_cols();
        let num_rows = self.num_rows();
        assert!(col_mid <= num_cols);
        assert!(row_mid <= num_rows);
        if col_mid == num_cols {
            col_mid = 0;
        }
        if row_mid == num_rows {
            row_mid = 0;
        }
        if row_mid == 0 {
            if col_mid != 0 {
                for r in self.rows_mut() {
                    r.rotate_left(col_mid);
                }
            }
            return;
        }
        let row_adj_abs = num_rows - row_mid;
        let mut swap_count = 0;
        let mut base_row = 0;
        while swap_count < num_rows {
            let mut mid = col_mid;
            let mut next_row = base_row + row_adj_abs;
            loop {
                if next_row >= num_rows {
                    next_row -= num_rows;
                }
                swap_count += 1;
                if base_row == next_row {
                    if mid > 0 {
                        unsafe {
                            self.get_unchecked_row_mut(base_row).rotate_left(mid);
                        }
                    }
                    break;
                } else {
                    let (base_ref, next_ref) = self.row_pair_mut(base_row, next_row);
                    unsafe {
                        if mid > 0 {
                            base_ref
                                .get_unchecked_mut(..mid)
                                .swap_with_slice(
                                    next_ref.get_unchecked_mut(num_cols - mid..num_cols),
                                );
                        }
                        if mid < num_cols {
                            base_ref
                                .get_unchecked_mut(mid..num_cols)
                                .swap_with_slice(
                                    next_ref.get_unchecked_mut(..num_cols - mid),
                                );
                        }
                    }
                    mid += col_mid;
                    if mid >= num_cols {
                        mid -= num_cols;
                    }
                }
                next_row += row_adj_abs;
            }
            if swap_count >= num_rows {
                break;
            }
            base_row += 1;
        }
    }
    /// Flips (or mirrors) the rows.
    ///
    /// # Examples
    ///
    /// ```
    /// use toodee::{TooDee,TooDeeOps,TranslateOps};
    /// let v = vec![42u32; 15];
    /// let mut toodee : TooDee<u32> = TooDee::from_vec(5, 3, v);
    /// toodee[0][1] = 1;
    /// toodee.flip_rows();
    /// assert_eq!(toodee[2][1], 1);
    /// ```
    fn flip_rows(&mut self) {
        let mut iter = self.rows_mut();
        while let (Some(r1), Some(r2)) = (iter.next(), iter.next_back()) {
            r1.swap_with_slice(r2);
        }
    }
    /// Flips (or mirrors) the columns.
    ///
    /// ```
    /// use toodee::{TooDee,TooDeeOps,TranslateOps};
    /// let v = vec![42u32; 15];
    /// let mut toodee : TooDee<u32> = TooDee::from_vec(5, 3, v);
    /// toodee[1][1] = 1;
    /// toodee.flip_cols();
    /// assert_eq!(toodee[1][3], 1);
    /// ```
    fn flip_cols(&mut self) {
        for r in self.rows_mut() {
            r.reverse();
        }
    }
}
impl<T, O> TranslateOps<T> for O
where
    O: TooDeeOpsMut<T>,
{}
#[cfg(test)]
mod tests_rug_104 {
    use super::*;
    use crate::{TooDee, TooDeeOps, TooDeeOpsMut, TranslateOps};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut v = vec![42u32; 15];
        let mut toodee: TooDee<u32> = TooDee::from_vec(rug_fuzz_0, rug_fuzz_1, v);
        let mid: (usize, usize) = (rug_fuzz_2, rug_fuzz_3);
        toodee.translate_with_wrap(mid);
             }
});    }
}
#[cfg(test)]
mod tests_rug_105 {
    use super::*;
    use crate::{TooDee, TooDeeOps, TooDeeOpsMut, TranslateOps};
    #[test]
    fn test_flip_rows() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(usize, usize, usize, usize, u32, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let v = vec![42u32; 15];
        let mut toodee: TooDee<u32> = TooDee::from_vec(rug_fuzz_0, rug_fuzz_1, v);
        toodee[(rug_fuzz_2, rug_fuzz_3)] = rug_fuzz_4;
        toodee.flip_rows();
        debug_assert_eq!(toodee[(rug_fuzz_5, rug_fuzz_6)], 1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_106 {
    use super::*;
    use crate::{TooDee, TooDeeOps, TranslateOps};
    #[test]
    fn test_flip_cols() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(usize, usize, usize, usize, u32, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let v = vec![42u32; 15];
        let mut toodee: TooDee<u32> = TooDee::from_vec(rug_fuzz_0, rug_fuzz_1, v);
        toodee[(rug_fuzz_2, rug_fuzz_3)] = rug_fuzz_4;
        toodee.flip_cols();
        debug_assert_eq!(toodee[(rug_fuzz_5, rug_fuzz_6)], 1);
             }
});    }
}
