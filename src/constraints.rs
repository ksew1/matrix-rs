use std::ops::{Index, IndexMut};

/// A matrix with an offset.
/// The indexing is done via the offset.
pub struct MatrixWithOffset<'a> {
    pub matrix: &'a [Vec<f64>],
    pub offset: &'a Offset,
}

impl<'a> MatrixWithOffset<'a> {
    /// Creates a new `MatrixWithOffset` from a `Split`.
    pub fn create_from_split(
        &self,
        Split { s11, s12, s21, s22 }: &'a Split<Offset>,
    ) -> Split<MatrixWithOffset<'a>> {
        Split {
            s11: s11.as_matrix_with_offset(self.matrix),
            s12: s12.as_matrix_with_offset(self.matrix),
            s21: s21.as_matrix_with_offset(self.matrix),
            s22: s22.as_matrix_with_offset(self.matrix),
        }
    }
}

impl Index<(usize, usize)> for MatrixWithOffset<'_> {
    type Output = f64;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.matrix[self.offset.row_start + row][self.offset.col_start + col]
    }
}

/// A mutable matrix with an offset.
/// The indexing is done via the offset.
pub struct MatrixWithOffsetMut<'a> {
    pub matrix: &'a mut [Vec<f64>],
    pub offset: &'a Offset,
}

impl Index<(usize, usize)> for MatrixWithOffsetMut<'_> {
    type Output = f64;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.matrix[self.offset.row_start + row][self.offset.col_start + col]
    }
}

impl IndexMut<(usize, usize)> for MatrixWithOffsetMut<'_> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self.matrix[self.offset.row_start + row][self.offset.col_start + col]
    }
}

/// An offset into a matrix.
#[derive(PartialEq)]
pub struct Offset {
    pub row_start: usize,
    pub col_start: usize,
    pub n: usize,
}

/// A split into four quadrants.
pub struct Split<T> {
    pub s11: T,
    pub s12: T,
    pub s21: T,
    pub s22: T,
}
impl Offset {
    /// Creates a new [`Offset`] from a matrix.
    /// It does not check if all rows and columns have the same length and just assumes that they do.
    pub fn new_unchecked(matrix: &[Vec<f64>]) -> Offset {
        Offset {
            row_start: 0,
            col_start: 0,
            n: matrix.len(),
        }
    }

    /// Creates a new [`MatrixWithOffset`] from a matrix.
    pub fn as_matrix_with_offset<'a>(&'a self, data: &'a [Vec<f64>]) -> MatrixWithOffset<'a> {
        MatrixWithOffset {
            matrix: data,
            offset: self,
        }
    }

    /// Creates a new [`MatrixWithOffsetMut`] from a matrix.
    pub fn as_matrix_with_offset_mut<'a>(
        &'a self,
        data: &'a mut [Vec<f64>],
    ) -> MatrixWithOffsetMut<'a> {
        MatrixWithOffsetMut {
            matrix: data,
            offset: self,
        }
    }

    /// Splits the offset into four quadrants.
    pub fn split(&self) -> Split<Offset> {
        let mid = self.n / 2;
        let s11 = Offset {
            row_start: self.row_start,
            col_start: self.col_start,
            n: mid,
        };
        let s12 = Offset {
            row_start: self.row_start,
            col_start: self.col_start + mid,
            n: mid,
        };
        let s21 = Offset {
            row_start: self.row_start + mid,
            col_start: self.col_start,
            n: mid,
        };
        let s22 = Offset {
            row_start: self.row_start + mid,
            col_start: self.col_start + mid,
            n: mid,
        };
        Split { s11, s12, s21, s22 }
    }
}
