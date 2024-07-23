use std::time::Instant;
use k2_tree::K2Tree;
use k2_tree::matrix::BitMatrix;
use std::mem::{size_of, size_of_val};
use rand::{Rng, thread_rng};
use rand_distr::{Normal, Distribution};

// Manual function to traverse the BitMatrix and get the value at (x, y)
fn get_bit_manual(matrix: &BitMatrix, x: usize, y: usize) -> bool {
    for j in 0..matrix.height {
        for i in 0..matrix.width {
            if i == x && j == y {
                return matrix.get(i, j).unwrap();
            }
        }
    }
    false
}

//
// This functions tests the time taken to find the value of a bit in a BitMatrix and a K2Tree.
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

fn print_matrix(matrix: &BitMatrix) {
    for y in 0..matrix.height {
        for x in 0..matrix.width {
            print!("{}", if matrix.get(x, y).unwrap() { 1 } else { 0 });
        }
        println!();
    }
}

fn print_tree(tree: &K2Tree) {
    let stem_block_size = tree.stem_k * tree.stem_k;
    let leaf_block_size = tree.leaf_k * tree.leaf_k;
    let mut current_index = 0;

    print!("[");

    for i in 0..(tree.stems.len() / stem_block_size) {
        if i > 0 {
            print!(", ");
        }
        for j in 0..stem_block_size {
            print!("{}", tree.stems[current_index + j] as u8);
        }
        current_index += stem_block_size;
    }

    print!("; ");

    current_index = 0;
    for i in 0..(tree.leaves.len() / leaf_block_size) {
        if i > 0 {
            print!(", ");
        }
        for j in 0..leaf_block_size {
            print!("{}", tree.leaves[current_index + j] as u8);
        }
        current_index += leaf_block_size;
    }

    println!("]");
}
