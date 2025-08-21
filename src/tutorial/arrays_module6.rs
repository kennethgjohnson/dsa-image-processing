mod warm_up_2d_arrays_and_flattening_them;
use crate::tutorial::{
    arrays_module6::warm_up_2d_arrays_and_flattening_them::make_matrix,
    common_util::{print_header, print_output_row_ratio_compare_result_nanos},
};
use std::time::{Duration, Instant};
use warm_up_2d_arrays_and_flattening_them::warm_up_2d_arrays_and_flattening_them;

pub fn arrays_module6_multi_dimensional_arrays_and_cache() {
    println!("Image Processing DSA - Module 6: Multi-Dimensional Arrays & Cache");

    // 1) Warm-Up - 2D Arrays
    // 1a) 2d Arrays in rust.
    // 1b) flatten to 1D form: Row-Major
    // 1c) flatten to 1D form: Col-Major
    warm_up_2d_arrays_and_flattening_them();

    // 2) Basic Matrix Operations
    // 2a1) Row Sum
    // 2a2) Col Sum
    // 2b) Transpose
    // 2c) Rotate 90 degrees clockwise
    basic_matrix_operations();

    // 3) Matrix Multiplication
    // 3a) Naive Triple-loop (Investigate inefficiencies with innerlooop jump around)
    // 3b) Optimize: precompute transpose(B) and use it to get cache-friendly row access.
    // 3c) Loop Tiling / Blocking in Matrix Multiplication
    // Note: Experement with blocksizes 8,16,32,64
    // 3d) Compare the performance of all 3 approaches using The performance comparison
    // suggestion of all three matrix multiplication approaches on matrices of size
    // 500 and 1000

    // 4) 2D Prefix Sums
    // 4a) Implement the Prefix sum
    // 4b) Implement Submatrix sums using inclusion-exclusion.

    // 5) Cache-Friendly vs. Cache-Unfriendly Traversals
    // 5a) Row-major sums
    // 5b) Col-major sums
    // 5c) Evaluate Claim: row-major is faster on large matrices.

    // 6) Advanced Patterns
    // 6a) Spiral Traversal

    // 6b) Maximum submatrix sum (Kadane's 2D Extension)

    // 7) Project: Mini Image Processor
    // 7a) Setup the basic
    // 7b) Brightness Adjustment
    // 7c) Transpose image (x/y swap)
    // 7d) Rotate 90 Degrees
    // 7e) Blur (replace each pixel with average of it's neighbors)
    // 7f) Edge detection (difference with neighbors)
}

fn basic_matrix_operations() {
    println!("==> 2) Basic Matrix Operations\n");
    // 2) Basic Matrix Operations
    row_sum_column_sum_impl();
    row_sum_column_sum_benchmarks();

    // 2b) Transpose
    transpose_impl(); // In progress
    transpose_benchmarks(); // In progress

    // 2c) Rotate 90 degrees clockwise
}

// TODO: implement block transposing, 1d row major with block transposing.
fn transpose_benchmarks() {
    println!("---> Transposing matrix benchmarks");
    let columns = ["Data Size", "Square Matrix (ns)"];
    print_header(&columns);
    let start_size = 4;
    let number_of_doubles = 11;
    // starting at
    let arr_sizes: Vec<usize> = (0..number_of_doubles).map(|i| start_size << i).collect();

    for size in &arr_sizes {
        let mut arr_time: Vec<Duration> = Vec::with_capacity(1000);

        for _ in 0..1000 {
            let matrix = make_matrix(size.clone() / 2, size.clone() / 2, 0);
            let start = Instant::now();
            let _transposed = transpose_poor_cache_locality(matrix);
            std::hint::black_box(_transposed);
            arr_time.push(start.elapsed());
        }

        let time = Duration::from_nanos(
            (arr_time.iter().map(|d| d.as_nanos()).sum::<u128>() / arr_time.len() as u128) as u64,
        );

        println!(
            "  {0:>1$} | {2:3$} ",
            size,
            columns[0].chars().count() - 1,
            time.as_nanos(),
            columns[1].chars().count() - 1
        );
    }
}

// TODO: implement block transposing, 1d row major with block transposing.
fn transpose_impl() {
    println!("---> Transposing matrix");

    let matrix = make_matrix(10, 10, 0);
    println!("Before:");
    print_matrix(&matrix);
    let trasposed_matrix = transpose_poor_cache_locality(matrix);
    println!("After:");
    print_matrix(&trasposed_matrix);
}

fn print_matrix(matrix: &Vec<Vec<i32>>) {
    print!(" {:>4} |", "idx");
    for col_idx in 0..matrix[0].len() {
        print!(" {:>4} |", col_idx);
    }
    print!("\n");
    for _ in 0..=matrix[0].len() {
        print!("------|");
    }
    print!("\n");
    for row_idx in 0..matrix.len() {
        let row = &*matrix[row_idx];
        print!(" {:>4} |", row_idx);
        row.iter().for_each(|column_value| {
            print!(" {:>4} |", column_value);
        });
        print!("\n");
    }
    println!("\n");
}

// This implementation of transpose has poor cache locality due to striding memory access,
// Is really only fast up to ~32x32 after which the whole matrix can no longer fit in cache.
// Block-Tiling the matrix is the better approach, and Block-Tiling with 1d row major arrays
// holding the matrix is the even bettter than that approach.
fn transpose_poor_cache_locality<T: Clone>(mut matrix: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let input_row_count = matrix.len();

    if input_row_count < 1 || matrix[0].len() < 1 {
        panic!("Matrix must be a 2d matrix with at least 1 element.");
    }

    let input_col_count = matrix[0].len();

    if matrix.len() == matrix[0].len() {
        // This is an inplace swap so excluding the diagonals we only need to swap
        // on a half and it will auto swap the other half as a result.

        // for each row
        for idx_row in 0..input_row_count {
            // Split the matrix into two mutable slices so we can memory swap later
            let (top, bottom) = matrix.split_at_mut(idx_row + 1);

            // start indexing after the diagonal + 1, +1 since the first element is on the diagonal
            for idx_col in idx_row + 1..input_col_count {
                //std::mem::swap(&mut matrix[idx_row][idx_col], &mut matrix[idx_col][idx_row]);
                std::mem::swap(
                    &mut top[idx_row][idx_col],
                    &mut bottom[idx_col - (idx_row + 1)][idx_row],
                );
            }
        }
        matrix
    } else {
        // Make new rows vector based on amount of columns
        let mut new_matrix = Vec::with_capacity(input_col_count);
        // Make new column vectors based on amount of rows.
        for _ in 0..input_col_count {
            new_matrix.push(Vec::with_capacity(input_row_count));
        }
        for input_row in matrix {
            // moves ownership
            //let drained_row = input_row.drain(..).collect::<Vec<T>>();
            //for (input_col_idx, val) in drained_row.into_iter().enumerate() {
            for (input_col_idx, val) in input_row.into_iter().enumerate() {
                new_matrix[input_col_idx].push(val);
            }
        }
        new_matrix
    }
}

fn row_sum_column_sum_benchmarks() {
    println!(
        "---> Column Sum vs Row Sum-For vs Row Sum-Itterator vs Row Sum-Coax-LLVM-to-SIMD-Vector-Optimize"
    );
    let columns = [
        "Data Size^2",
        "Column-based (ns)",
        "Row-based For (ns)",
        "      Ratio", // Ratio values getting a bit big so hacking a bit of space
        "Row-based For (ns)",
        "Row-based Itterator (ns)",
        "      Ratio", // Ratio values getting a bit big so hacking a bit of space
        "Row-based Itterator (ns)",
        "Row-based coax llvm to simd vector optimize (ns)",
        "      Ratio",
    ];
    print_header(&columns);
    let start_size = 256;
    let number_of_doubles = 6;
    // starting at
    let arr_sizes: Vec<usize> = (0..number_of_doubles).map(|i| start_size << i).collect();

    for size in &arr_sizes {
        let mut arr_time_col: Vec<Duration> = Vec::with_capacity(10);
        let mut arr_time_row_iter: Vec<Duration> = Vec::with_capacity(10);
        let mut arr_time_row: Vec<Duration> = Vec::with_capacity(10);
        let mut arr_time_row_llvm_coax: Vec<Duration> = Vec::with_capacity(10);

        for _ in 0..10 {
            let matrix = make_matrix(size.clone(), size.clone(), 1);
            let start = Instant::now();
            let _sum = col_sum_matrix(&matrix);
            std::hint::black_box(_sum);
            arr_time_col.push(start.elapsed());
        }

        for _ in 0..10 {
            let matrix = make_matrix(size.clone(), size.clone(), 1);
            let start = Instant::now();
            let _sum = row_sum_matrix(&matrix);
            std::hint::black_box(_sum);
            arr_time_row.push(start.elapsed());
        }

        for _ in 0..10 {
            let matrix = make_matrix(size.clone(), size.clone(), 1);
            let start = Instant::now();
            let _sum = row_sum_matrix_iterator(&matrix);
            std::hint::black_box(_sum);
            arr_time_row_iter.push(start.elapsed());
        }

        for _ in 0..10 {
            let matrix = make_matrix(size.clone(), size.clone(), 1);
            let start = Instant::now();
            let _sum = row_sum_matrix_trying_to_coax_llvm_to_optimize_using_simd(&matrix);
            std::hint::black_box(_sum);
            arr_time_row_llvm_coax.push(start.elapsed());
        }

        let time_col = Duration::from_nanos(
            (arr_time_col.iter().map(|d| d.as_nanos()).sum::<u128>() / arr_time_col.len() as u128)
                as u64,
        );

        let time_row = Duration::from_nanos(
            (arr_time_row.iter().map(|d| d.as_nanos()).sum::<u128>() / arr_time_row.len() as u128)
                as u64,
        );

        let time_row_iter = Duration::from_nanos(
            (arr_time_row_iter.iter().map(|d| d.as_nanos()).sum::<u128>()
                / arr_time_row_iter.len() as u128) as u64,
        );

        let time_row_llvm_coax = Duration::from_nanos(
            (arr_time_row_llvm_coax
                .iter()
                .map(|d| d.as_nanos())
                .sum::<u128>()
                / arr_time_row_llvm_coax.len() as u128) as u64,
        );

        print_output_row_ratio_compare_result_nanos(
            &columns,
            *size,
            vec![
                (time_col, time_row),
                (time_row, time_row_iter),
                (time_row_iter, time_row_llvm_coax),
            ],
        );
    }
}

fn row_sum_column_sum_impl() {
    println!("--> Make a Matrix");
    // maximum size that will sum under i32::MAX
    let matrix = make_matrix(50000, 30000, 1);
    println!("done.");
    //println!("{:?}", matrix);
    // 2a1) Row Sum
    println!("--> row_sum_matrix_iterator");
    let start = Instant::now();
    let sum = row_sum_matrix_iterator(&matrix);
    println!("Row Sum Result iterator: {} in {:?}", sum, start.elapsed());
    println!("--> row_sum_matrix");
    let start = Instant::now();
    let sum = row_sum_matrix(&matrix);
    println!("Row Sum Result: {} in {:?}", sum, start.elapsed());
    println!("--> row_sum_matrix_trying_to_coax_llvm_to_optimize_using_simd");
    let start = Instant::now();
    let sum = row_sum_matrix_trying_to_coax_llvm_to_optimize_using_simd(&matrix);
    println!(
        "Row Sum Result LLVM optimization coaxing: {} in {:?}",
        sum,
        start.elapsed()
    );
    // 2a2) Col Sum
    println!("--> col_sum_matrix");
    let start = Instant::now();
    let sum = col_sum_matrix(&matrix);
    println!("Col Sum Result: {} in {:?}", sum, start.elapsed());
}

fn row_sum_matrix_iterator(matrix: &Vec<Vec<i32>>) -> i32 {
    matrix.iter().map(|row| row.iter().sum::<i32>()).sum()
}

fn row_sum_matrix(matrix: &Vec<Vec<i32>>) -> i32 {
    let row_count = matrix.len();
    let col_count = matrix[0].len();
    let mut sum = 0;
    for row_idx in 0..row_count {
        for col_idx in 0..col_count {
            sum += matrix[row_idx][col_idx];
        }
    }
    sum
}

fn row_sum_matrix_trying_to_coax_llvm_to_optimize_using_simd(matrix: &Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    for row in matrix {
        for value in row {
            sum += value;
        }
    }
    sum
}

fn col_sum_matrix(matrix: &Vec<Vec<i32>>) -> i32 {
    let row_count = matrix.len();
    let col_count = matrix[0].len();
    let mut sum = 0;
    for col_idx in 0..col_count {
        for row_idx in 0..row_count {
            sum += matrix[row_idx][col_idx];
        }
    }
    sum
}
