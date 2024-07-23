mod utils;

use std::time::Instant;
use k2_tree::K2Tree;
use k2_tree::matrix::BitMatrix;
use std::mem::{size_of, size_of_val};
use rand::{Rng, thread_rng};
use rand_distr::{Normal, Distribution};
use crate::utils::get_bit_manual;

//
// This function tests the time taken to find the value of a bit in a BitMatrix and a K2Tree.
//
// The function generates a random BitMatrix of size x size and then creates a K2Tree from it.
// After that, it measures the time taken to find the value of a bit in the BitMatrix and the K2Tree.
//
fn test_query_time(matrix_size: usize) {
    println!("-------------------");
    println!("Measuring query time ({}x{}) ...", matrix_size, matrix_size);

    let mut m = BitMatrix::with_dimensions(matrix_size, matrix_size);

    let normal = Normal::new(0.5, 0.9).unwrap();
    let mut rng = thread_rng();

    for _ in 0..5 {
        let x = rng.gen_range(0..matrix_size);
        let y = rng.gen_range(0..matrix_size);
        let value = normal.sample(&mut rng);
        m.set(x, y, true).unwrap();
    }

    let tree = K2Tree::from_matrix(m.clone(), 2, 2).unwrap();

    let x = thread_rng().gen_range(0..matrix_size);
    let y = thread_rng().gen_range(0..matrix_size);

    let start_time_matrix = Instant::now();
    let _ = get_bit_manual(&m, x, y);
    let duration_matrix = start_time_matrix.elapsed();

    let start_time_tree = Instant::now();
    let _ = tree.get(x, y).unwrap();
    let duration_tree = start_time_tree.elapsed();

    println!("BitMatrix: {} nanoseconds", duration_matrix.as_nanos());
    println!("K2Tree: {} nanoseconds", duration_tree.as_nanos());
}

fn main() {
    let sizes = [4, 8, 16, 32, 64, 128, 256, 512];

    for &size in sizes.iter() {
        test_query_time(size);
    }
}
