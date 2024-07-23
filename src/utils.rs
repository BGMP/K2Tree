use std::error::Error;
use std::fs::OpenOptions;
use csv::{WriterBuilder};
use k2_tree::K2Tree;
use k2_tree::matrix::BitMatrix;

// Manual function to traverse the BitMatrix and get the value at (x, y)
pub fn get_bit_manual(matrix: &BitMatrix, x: usize, y: usize) -> bool {
    for j in 0..matrix.height {
        for i in 0..matrix.width {
            if i == x && j == y {
                return matrix.get(i, j).unwrap();
            }
        }
    }
    false
}

// Function to print the BitMatrix
pub fn print_matrix(matrix: &BitMatrix) {
    for y in 0..matrix.height {
        for x in 0..matrix.width {
            print!("{}", if get_bit_manual(matrix, x, y) { 1 } else { 0 });
        }
        println!();
    }
}

// Function to print the K2Tree
pub fn print_tree(tree: &K2Tree) {
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

pub fn write_time_to_csv(data: &[(usize, u128, u128)], file_path: &str) -> Result<(), Box<dyn Error>> {
    let file_exists = std::fs::metadata(file_path).is_ok();

    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;

    let mut wtr = WriterBuilder::new()
        .has_headers(!file_exists)
        .from_writer(file);

    // Write the header if the file does not exist
    if !file_exists {
        wtr.write_record(&["n", "bitmatrix", "k2tree"])?;
    }

    // Write the data rows
    for &(n, bitmatrix, k2tree) in data {
        wtr.write_record(&[n.to_string(), bitmatrix.to_string(), k2tree.to_string()])?;
    }

    wtr.flush()?;
    Ok(())
}

pub fn write_space_to_csv(data: &[(usize, usize, usize)], file_path: &str) -> Result<(), Box<dyn Error>> {
    let file_exists = std::fs::metadata(file_path).is_ok();

    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;

    let mut wtr = WriterBuilder::new()
        .has_headers(!file_exists)
        .from_writer(file);

    // Write the header if the file does not exist
    if !file_exists {
        wtr.write_record(&["n", "bitmatrix", "k2tree"])?;
    }

    for &(n, bitmatrix, k2tree) in data {
        wtr.write_record(&[n.to_string(), bitmatrix.to_string(), k2tree.to_string()])?;
    }

    wtr.flush()?;
    Ok(())
}
