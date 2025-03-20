mod utils;

use crate::integration::utils::test_for_size;
use matrix_rs::multiply;

#[test]
fn test_valid_multiplication() {
    let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
    let b = vec![vec![5.0, 6.0], vec![7.0, 8.0]];
    let expected = vec![vec![19.0, 22.0], vec![43.0, 50.0]];

    assert_eq!(multiply(a, b).unwrap(), expected);
}

#[test]
fn test_minimum_valid_matrix_size() {
    let a = vec![vec![42.0]];
    let b = vec![vec![1.0]];
    let expected = vec![vec![42.0]];

    assert_eq!(multiply(a, b).unwrap(), expected);
}

#[test]
fn test_invalid_matrix_not_power_of_two() {
    let a = vec![
        vec![1.0, 2.0, 3.0],
        vec![4.0, 5.0, 6.0],
        vec![7.0, 8.0, 9.0],
    ];
    let b = a.clone();

    assert!(multiply(a, b).is_err());
}

#[test]
fn test_rectangular_matrix_should_fail() {
    let a = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
    let b = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];

    assert!(multiply(a, b).is_err());
}

#[test]
fn test_irregular_matrix_should_fail() {
    let a = vec![vec![1.0, 2.0], vec![3.0, 4.0, 5.0]];
    let b = vec![vec![6.0, 7.0], vec![8.0, 9.0]];

    assert!(multiply(a, b).is_err());
}

#[test]
fn test_empty_matrix_should_fail() {
    let a: Vec<Vec<f64>> = vec![];
    let b: Vec<Vec<f64>> = vec![];
    assert!(multiply(a, b).is_err());
}

#[test]
fn test_large_valid_power_of_two_matrix() {
    let a = vec![
        vec![1.0, 2.0, 3.0, 4.0],
        vec![5.0, 6.0, 7.0, 8.0],
        vec![9.0, 10.0, 11.0, 12.0],
        vec![13.0, 14.0, 15.0, 16.0],
    ];
    let b = vec![
        vec![1.0, 0.0, 0.0, 0.0],
        vec![0.0, 1.0, 0.0, 0.0],
        vec![0.0, 0.0, 1.0, 0.0],
        vec![0.0, 0.0, 0.0, 1.0],
    ];
    let result = multiply(a.clone(), b);
    assert_eq!(result.unwrap(), a);
}

#[test]
fn test_check_behaviour_with_nalgebra() {
    test_for_size(2);
    test_for_size(4);
    test_for_size(8);
    test_for_size(2_usize.pow(7));
    test_for_size(2_usize.pow(8));
}
