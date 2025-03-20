use matrix_rs::multiply;
use nalgebra::DMatrix;
use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};
use std::sync::{LazyLock, Mutex};

static RNG: LazyLock<Mutex<StdRng>> = LazyLock::new(|| Mutex::new(StdRng::seed_from_u64(42)));

pub fn test_for_size(size: usize) {
    assert!(size.is_power_of_two());

    let matrix_a = DMatrix::generate_random(size);
    let matrix_b = DMatrix::generate_random(size);

    let expected = (&matrix_a * &matrix_b).to_vec();
    let result = multiply(matrix_a.to_vec(), matrix_b.to_vec()).unwrap();

    assert!(are_matrices_approx_equal(&expected, &result, 0.10e-10));
}

fn are_matrices_approx_equal(a: &[Vec<f64>], b: &[Vec<f64>], epsilon: f64) -> bool {
    for (row_a, row_b) in a.iter().zip(b.iter()) {
        for (&val_a, &val_b) in row_a.iter().zip(row_b.iter()) {
            if (val_a - val_b).abs() > epsilon {
                return false;
            }
        }
    }

    true
}

pub trait MatrixExt {
    fn generate_random(size: usize) -> Self;
    fn to_vec(&self) -> Vec<Vec<f64>>;
}

impl MatrixExt for DMatrix<f64> {
    fn generate_random(size: usize) -> Self {
        let random_data: Vec<f64> = (0..(size * size))
            .map(|_| RNG.lock().unwrap().random_range(-10.0..10.0))
            .collect();

        DMatrix::from_vec(size, size, random_data)
    }

    fn to_vec(&self) -> Vec<Vec<f64>> {
        self.row_iter()
            .map(|row| row.iter().copied().collect())
            .collect()
    }
}
