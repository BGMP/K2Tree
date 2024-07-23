use std::fmt::Alignment::Center;
use std::fmt::{Display, Formatter};
use std::time::Instant;
use bitvec::bitvec;
use k2_tree::K2Tree;
use k2_tree::matrix::BitMatrix;
use std::mem::{size_of, size_of_val};

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

fn main() {
    // For a 4x4 example
    let mut m1 = BitMatrix::with_dimensions(4, 4);

    // Q1
    m1.set(0, 0, true).unwrap();
    m1.set(0, 1, true).unwrap();
    m1.set(1, 1, true).unwrap();

    // Q2 and Q3 are all zeros

    // Q4
    m1.set(2, 2, true).unwrap();
    m1.set(2, 3, true).unwrap();
    m1.set(3, 2, true).unwrap();

    print_matrix(&m1);

    println!("-----------------");

    // For a 8x8 example (https://docs.rs/k2_tree/latest/k2_tree/#original-bit-matrix)
    let mut m2 = BitMatrix::with_dimensions(8, 8);

    // Q1 is all zeroes

    // Q2
    m2.set(4, 0, true).unwrap();
    m2.set(6, 0, true).unwrap();
    m2.set(6, 1, true).unwrap();
    m2.set(7, 1, true).unwrap();
    m2.set(6, 3, true).unwrap();

    // Q3
    m2.set(0, 4, true).unwrap();
    m2.set(2, 4, true).unwrap();
    m2.set(0, 5, true).unwrap();

    // Q4
    m2.set(6, 4, true).unwrap();
    m2.set(7, 4, true).unwrap();

    // Measure the time to find the value of a bit in the BitMatrix
    let x = 6;
    let y = 1;

    let start_matrix = Instant::now();
    let bit_value_matrix = m2.get(x, y).unwrap();
    let duration_matrix = start_matrix.elapsed();

    println!("Time taken to find the value of a bit in BitMatrix: {} nanoseconds", duration_matrix.as_nanos());
    println!("Bit value at ({}, {}): {}", x, y, bit_value_matrix);

    let mut tree = K2Tree::from_matrix(m2.clone(), 2, 2).unwrap();

    // Measure the time to find the value of a bit in the K2Tree
    let start_tree = Instant::now();
    let bit_value_tree = tree.get(x, y).unwrap();
    let duration_tree = start_tree.elapsed();

    println!("Time taken to find the value of a bit in K2Tree: {} nanoseconds", duration_tree.as_nanos());
    println!("Bit value at ({}, {}): {}", x, y, bit_value_tree);

    // Measure space used by BitMatrix
    let bit_matrix_size = size_of_val(&m2) + (m2.width * m2.height) / 8;
    println!("Space used by BitMatrix: {} bytes", bit_matrix_size);

    // Measure space used by K2Tree
    let k2_tree_size = size_of_val(&tree) + tree.stems.len() * size_of::<u8>() + tree.leaves.len() * size_of::<u8>();
    println!("Space used by K2Tree: {} bytes", k2_tree_size);

    print_tree(&tree);
}
