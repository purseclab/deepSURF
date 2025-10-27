pub fn multiply<
    Matrix: AsRef<[Row]>,
    Matrix2: AsRef<[Row2]>,
    Row: AsRef<[f32]>,
    Row2: AsRef<[f32]>,
>(
    mat1: &Matrix,
    mat2: &Matrix2,
) -> Vec<Vec<f32>> {
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

pub fn add<Matrix: AsRef<[Row]>, Row: AsRef<[f32]>>(mat1: &Matrix, mat2: &Matrix) -> Vec<Vec<f32>> {
    let m1_row = mat1.as_ref().len();
    let m1_col = mat1.as_ref()[0].as_ref().len();
    let mut result = vec![];
    for i in 0..m1_row {
        let mut new_row = vec![];
        for j in 0..m1_col {
            new_row.push((&mat1.as_ref()[i]).as_ref()[j] + (&mat2.as_ref()[i]).as_ref()[j]);
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
        assert_eq!(vec![vec![5.0, 14.0], vec![14.0, 41.0]], multiply(&t1, &t2));
    }

    #[test]
    fn matrix_add() {
        let t1 = [[1.0, 2.0], [4.0, 5.0]];
        let t2 = [[1.0, 4.0], [2.0, 5.0]];
        assert_eq!(vec![vec![2.0, 6.0], vec![6.0, 10.0]], add(&t1, &t2));
    }
}
