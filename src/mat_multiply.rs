use crate::constraints::{MatrixWithOffset, MatrixWithOffsetMut, Offset, Split};
use crate::validation::{validate_matrices, MatricesError};

/// The threshold for when to switch to the naive algorithm.
const THRESHOLD: usize = 2_usize.pow(1);

/// Multiplies two matrices of size 2^a x 2^a using the Binet algorithm and returns the result.
#[expect(clippy::needless_pass_by_value, clippy::missing_errors_doc)]
pub fn multiply(a: Vec<Vec<f64>>, b: Vec<Vec<f64>>) -> Result<Vec<Vec<f64>>, MatricesError> {
    validate_matrices(&a, &b)?;
    let a_offset = Offset::new_unchecked(&a);
    let b_offset = Offset::new_unchecked(&b);

    let n = a_offset.n;
    let mut c = vec![vec![0.0; n]; n];
    let c_offset = Offset::new_unchecked(&c);

    multiply_into(
        &a_offset.as_matrix_with_offset(&a),
        &b_offset.as_matrix_with_offset(&b),
        &mut c_offset.as_matrix_with_offset_mut(&mut c),
    );

    Ok(c)
}

/// Multiplies two matrices of size 2^a x 2^a using the Binet algorithm into the result matrix.
fn multiply_into(a: &MatrixWithOffset, b: &MatrixWithOffset, c: &mut MatrixWithOffsetMut) {
    let n = a.offset.n;
    if n <= THRESHOLD {
        naive_multiply_into(a, b, c);
        return;
    }

    let mid = n / 2;

    let split = a.offset.split();
    let Split {
        s11: a11,
        s12: a12,
        s21: a21,
        s22: a22,
    } = a.create_from_split(&split);

    let split = b.offset.split();
    let Split {
        s11: b11,
        s12: b12,
        s21: b21,
        s22: b22,
    } = b.create_from_split(&split);

    let Split {
        s11: c11_offset,
        s12: c12_offset,
        s21: c21_offset,
        s22: c22_offset,
    } = c.offset.split();

    let mut buffer = vec![vec![0.0; mid]; mid];
    let buffer_offset = Offset::new_unchecked(&buffer);

    // upper left
    multiply_into(
        &a11,
        &b11,
        &mut buffer_offset.as_matrix_with_offset_mut(&mut buffer),
    );
    multiply_into(
        &a12,
        &b21,
        &mut c11_offset.as_matrix_with_offset_mut(c.matrix),
    );
    add_into(
        &mut c11_offset.as_matrix_with_offset_mut(c.matrix),
        &buffer_offset.as_matrix_with_offset(&buffer),
    );

    // upper right
    multiply_into(
        &a11,
        &b12,
        &mut buffer_offset.as_matrix_with_offset_mut(&mut buffer),
    );
    multiply_into(
        &a12,
        &b22,
        &mut c12_offset.as_matrix_with_offset_mut(c.matrix),
    );
    add_into(
        &mut c12_offset.as_matrix_with_offset_mut(c.matrix),
        &buffer_offset.as_matrix_with_offset(&buffer),
    );

    // lower left
    multiply_into(
        &a21,
        &b11,
        &mut buffer_offset.as_matrix_with_offset_mut(&mut buffer),
    );
    multiply_into(
        &a22,
        &b21,
        &mut c21_offset.as_matrix_with_offset_mut(c.matrix),
    );
    add_into(
        &mut c21_offset.as_matrix_with_offset_mut(c.matrix),
        &buffer_offset.as_matrix_with_offset(&buffer),
    );

    // lower right
    multiply_into(
        &a21,
        &b12,
        &mut buffer_offset.as_matrix_with_offset_mut(&mut buffer),
    );
    multiply_into(
        &a22,
        &b22,
        &mut c22_offset.as_matrix_with_offset_mut(c.matrix),
    );
    add_into(
        &mut c22_offset.as_matrix_with_offset_mut(c.matrix),
        &buffer_offset.as_matrix_with_offset(&buffer),
    );
}

/// Adds the elements of matrix `b` into the elements of matrix `a`.
fn add_into(a: &mut MatrixWithOffsetMut, b: &MatrixWithOffset) {
    let n = b.offset.n;

    for i in 0..n {
        for j in 0..n {
            a[(i, j)] += b[(i, j)];
        }
    }
}

/// Multiplies two matrices of size 2^a x 2^a using the naive algorithm into the result matrix.
pub fn naive_multiply_into(
    a: &MatrixWithOffset,
    b: &MatrixWithOffset,
    result: &mut MatrixWithOffsetMut,
) {
    let n = a.offset.n;
    for i in 0..n {
        for j in 0..n {
            result[(i, j)] = 0.0;
            for k in 0..n {
                result[(i, j)] += a[(i, k)] * b[(k, j)];
            }
        }
    }
}
