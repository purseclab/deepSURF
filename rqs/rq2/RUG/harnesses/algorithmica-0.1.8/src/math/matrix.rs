pub fn multiply<
    Matrix: AsRef<[Row]>,
    Matrix2: AsRef<[Row2]>,
    Row: AsRef<[f32]>,
    Row2: AsRef<[f32]>,
>(mat1: &Matrix, mat2: &Matrix2) -> Vec<Vec<f32>> {
    let mut result = vec![];
    let m1_row = mat1.as_ref().len();
    let m2_row = mat2.as_ref().len();
    let m2_col = mat2.as_ref()[0].as_ref().len();
    if m1_row == 0 || m2_row == 0 || m2_col != m1_row {
        return vec![];
    }
    for i in 0..m1_row {
        let mut v = vec![];
        for j in 0..m2_col {
            let mut c = 0.0;
            for k in 0..m2_row {
                c += (&mat1.as_ref()[i]).as_ref()[k] * (&mat2.as_ref()[k]).as_ref()[j];
            }
            v.push(c);
        }
        result.push(v);
    }
    result
}
pub fn add<Matrix: AsRef<[Row]>, Row: AsRef<[f32]>>(
    mat1: &Matrix,
    mat2: &Matrix,
) -> Vec<Vec<f32>> {
    let m1_row = mat1.as_ref().len();
    let m1_col = mat1.as_ref()[0].as_ref().len();
    let mut result = vec![];
    for i in 0..m1_row {
        let mut new_row = vec![];
        for j in 0..m1_col {
            new_row
                .push((&mat1.as_ref()[i]).as_ref()[j] + (&mat2.as_ref()[i]).as_ref()[j]);
        }
        result.push(new_row);
    }
    result
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn matrix_mul() {
        let t1 = [[1.0, 2.0], [4.0, 5.0]];
        let t2 = [[1.0, 4.0], [2.0, 5.0]];
        assert_eq!(vec![vec![5.0, 14.0], vec![14.0, 41.0]], multiply(& t1, & t2));
    }
    #[test]
    fn matrix_add() {
        let t1 = [[1.0, 2.0], [4.0, 5.0]];
        let t2 = [[1.0, 4.0], [2.0, 5.0]];
        assert_eq!(vec![vec![2.0, 6.0], vec![6.0, 10.0]], add(& t1, & t2));
    }
}
#[cfg(test)]
mod tests_rug_1 {
    use super::*;
    use std::rc::Rc;
    use std::vec::IntoIter;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f32, f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: Rc<Vec<Vec<f32>>> = Rc::new(
            vec![vec![rug_fuzz_0, 2.0], vec![3.0, 4.0]],
        );
        let mut p1: IntoIter<Vec<f32>> = vec![vec![rug_fuzz_1, 6.0], vec![7.0, 8.0]]
            .into_iter();
        crate::math::matrix::multiply(&*p0, &p1.collect::<Vec<_>>());
             }
});    }
}
#[cfg(test)]
mod tests_rug_2 {
    use super::*;
    use std::vec::Drain;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(f32, f32, usize, usize, usize, usize, usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: Vec<Vec<f32>> = vec![vec![rug_fuzz_0, 2.0], vec![3.0, 4.0]];
        let p1: Vec<Vec<f32>> = vec![vec![rug_fuzz_1, 6.0], vec![7.0, 8.0]];
        let result = crate::math::matrix::add(&p0, &p1);
        debug_assert_eq!(result[rug_fuzz_2] [rug_fuzz_3], 6.0);
        debug_assert_eq!(result[rug_fuzz_4] [rug_fuzz_5], 8.0);
        debug_assert_eq!(result[rug_fuzz_6] [rug_fuzz_7], 10.0);
        debug_assert_eq!(result[rug_fuzz_8] [rug_fuzz_9], 12.0);
             }
});    }
}
