mod utils;

use std::time::Instant;
use k2_tree::K2Tree;
use k2_tree::matrix::BitMatrix;
use std::mem::{size_of, size_of_val};
use rand::{Rng, thread_rng};
use rand_distr::{Normal, Distribution};
use crate::utils::{get_bit_manual, print_matrix, write_space_to_csv, write_time_to_csv};

//
// This function tests the time taken to find the value of a bit in a BitMatrix and a K2Tree.
//
// The function generates a random BitMatrix of size x size and then creates a K2Tree from it.
// After that, it measures the time taken to find the value of a bit in the BitMatrix and the K2Tree.
//
fn test_query_time(matrix_size: usize) -> (usize, u128, u128) {
    println!("--------------------------------------");
    println!("Measuring query time ({}x{}) ...", matrix_size, matrix_size);

    let mut m = BitMatrix::with_dimensions(matrix_size, matrix_size);
    let mut rng = thread_rng();
    let normal = Normal::new(0.5, 0.9).unwrap();
    let threshold = 1.28;

    for y in 0..matrix_size {
        for x in 0..matrix_size {
            m.set(x, y, normal.sample(&mut rng) > threshold).unwrap();
        }
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

    println!("> BitMatrix: {} nanoseconds", duration_matrix.as_nanos());
    println!("> K2Tree: {} nanoseconds", duration_tree.as_nanos());

    return (matrix_size, duration_matrix.as_nanos(), duration_tree.as_nanos())
}

///
/// This function tests the space usage of a BitMatrix and a K2Tree.
///
/// The function generates a random BitMatrix of size x size and then creates a K2Tree from it.
/// After that, it measures the space used by the BitMatrix and the K2Tree.
///
fn test_space(matrix_size: usize) -> (usize, usize, usize) {
    println!("Measuring space usage ({}x{}) ...", matrix_size, matrix_size);

    let mut m = BitMatrix::with_dimensions(matrix_size, matrix_size);
    let mut rng = thread_rng();
    let normal = Normal::new(0.5, 0.9).unwrap();
    let threshold = 1.28;

    for y in 0..matrix_size {
        for x in 0..matrix_size {
            m.set(x, y, normal.sample(&mut rng) > threshold).unwrap();
        }
    }

    let tree = K2Tree::from_matrix(m.clone(), 2, 2).unwrap();

    let bit_matrix_size = size_of_val(&m) + (m.width * m.height + 7) / 8;
    let k2_tree_size = size_of_val(&tree)
        + tree.stems.len() * size_of::<u8>()
        + tree.leaves.len() * size_of::<u8>();

    println!("> BitMatrix: {} bytes", bit_matrix_size);
    println!("> K2Tree: {} bytes", k2_tree_size);

    return (matrix_size, bit_matrix_size, k2_tree_size)
}

fn main() {
    let sizes = [4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096]; // n

    let mut time_data = Vec::new();
    let mut space_data = Vec::new();

    for &size in sizes.iter() {
        time_data.push(test_query_time(size));
        space_data.push(test_space(size));
    }

    // Ensure the "data" folder exists
    std::fs::create_dir_all("data").unwrap();

    if let Err(e) = write_time_to_csv(&time_data, "data/time.csv") {
        eprintln!("Error writing time data to CSV: {}", e);
    }

    if let Err(e) = write_space_to_csv(&space_data, "data/space.csv") {
        eprintln!("Error writing space data to CSV: {}", e);
    }
}
