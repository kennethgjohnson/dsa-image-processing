mod warm_up_2d_arrays_and_flattening_them;
use crate::tutorial::{
    arrays_module6::warm_up_2d_arrays_and_flattening_them::{
        flatten_row_major, get_row_major, make_matrix,
    },
    common_util::{print_header, print_output_row_ratio_compare_result_nanos},
};
use std::{
    mem::MaybeUninit,
    time::{Duration, Instant},
};
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
    // // 2) Basic Matrix Operations
    row_sum_column_sum_impl();
    row_sum_column_sum_benchmarks();

    // 2b) Transpose
    transpose_impl(); // In progress
    transpose_benchmarks(); // In progress

    // 2c) Rotate 90 degrees clockwise
}

// TODO: implement block transposing, 1d row major with block transposing.
fn transpose_benchmarks() {
    println!("---> Transposing square matrix benchmarks");
    let columns = [
        "Data Size",
        "  2d (ns)",
        "1d RM(ns)",
        "Ratio", // Ratio values getting a bit big so hacking a bit of space
        "  2d (ns)",
        "2d Blocks(64^2) (ns)",
        "Ratio", // Ratio values getting a bit big so hacking a bit of space
        "  2d (ns)",
        "2d Blocks(128^2) (ns)",
        "Ratio", // Ratio values getting a bit big so hacking a bit of space
        "  2d (ns)",
        "2d Blocks(256^2) (ns)",
        "Ratio", // Ratio values getting a bit big so hacking a bit of space
    ];
    print_header(&columns);
    let start_size = 32;
    let number_of_doubles = 9;
    // starting at
    let arr_sizes: Vec<usize> = (0..number_of_doubles).map(|i| start_size << i).collect();

    for size in &arr_sizes {
        let mut arr_time_2d_vec: Vec<Duration> = Vec::with_capacity(10);
        let mut arr_time_1d_row_major_vec: Vec<Duration> = Vec::with_capacity(10);
        let mut arr_time_2d_vec_block_64_tile_loop: Vec<Duration> = Vec::with_capacity(10);
        let mut arr_time_2d_vec_block_128_tile_loop: Vec<Duration> = Vec::with_capacity(10);
        let mut arr_time_2d_vec_block_256_tile_loop: Vec<Duration> = Vec::with_capacity(10);

        for _ in 0..10 {
            let matrix = make_matrix(*size, *size, 0);
            let start = Instant::now();
            let _transposed = transpose_2d_vec_poor_cache_locality(matrix);
            std::hint::black_box(_transposed);
            arr_time_2d_vec.push(start.elapsed());
        }
        for _ in 0..10 {
            let matrix = make_matrix(*size, *size, 0);
            let flat_row_major_matrix = flatten_row_major(&matrix);
            let start = Instant::now();
            let _transposed =
                transpose_1d_row_major_matrix_poor_cache_locality(flat_row_major_matrix, *size);
            std::hint::black_box(_transposed);
            arr_time_1d_row_major_vec.push(start.elapsed());
        }

        for _ in 0..10 {
            let matrix = make_matrix(*size, *size, 0);
            let start = Instant::now();
            // Trying 32x32 blocks first.
            let _transposed = transpose_2d_vec_loop_tiling_aka_block_tiling(matrix, 64);
            std::hint::black_box(_transposed);
            arr_time_2d_vec_block_64_tile_loop.push(start.elapsed());
        }

        for _ in 0..10 {
            let matrix = make_matrix(*size, *size, 0);
            let start = Instant::now();
            // Trying 32x32 blocks first.
            let _transposed = transpose_2d_vec_loop_tiling_aka_block_tiling(matrix, 128);
            std::hint::black_box(_transposed);
            arr_time_2d_vec_block_128_tile_loop.push(start.elapsed());
        }

        for _ in 0..10 {
            let matrix = make_matrix(*size, *size, 0);
            let start = Instant::now();
            // Trying 32x32 blocks first.
            let _transposed = transpose_2d_vec_loop_tiling_aka_block_tiling(matrix, 256);
            std::hint::black_box(_transposed);
            arr_time_2d_vec_block_256_tile_loop.push(start.elapsed());
        }

        let time_2d_vec = Duration::from_nanos(
            (arr_time_2d_vec.iter().map(|d| d.as_nanos()).sum::<u128>()
                / arr_time_2d_vec.len() as u128) as u64,
        );

        let time_1d_row_major_vec = Duration::from_nanos(
            (arr_time_1d_row_major_vec
                .iter()
                .map(|d| d.as_nanos())
                .sum::<u128>()
                / arr_time_1d_row_major_vec.len() as u128) as u64,
        );

        let time_2d_vec_block_64_tiling_loop = Duration::from_nanos(
            (arr_time_2d_vec_block_64_tile_loop
                .iter()
                .map(|d| d.as_nanos())
                .sum::<u128>()
                / arr_time_2d_vec_block_64_tile_loop.len() as u128) as u64,
        );

        let time_2d_vec_block_128_tiling_loop = Duration::from_nanos(
            (arr_time_2d_vec_block_128_tile_loop
                .iter()
                .map(|d| d.as_nanos())
                .sum::<u128>()
                / arr_time_2d_vec_block_128_tile_loop.len() as u128) as u64,
        );

        let time_2d_vec_block_256_tiling_loop = Duration::from_nanos(
            (arr_time_2d_vec_block_256_tile_loop
                .iter()
                .map(|d| d.as_nanos())
                .sum::<u128>()
                / arr_time_2d_vec_block_256_tile_loop.len() as u128) as u64,
        );

        print_output_row_ratio_compare_result_nanos(
            &columns,
            *size * *size,
            vec![
                (time_2d_vec, time_1d_row_major_vec),
                (time_2d_vec, time_2d_vec_block_64_tiling_loop),
                (time_2d_vec, time_2d_vec_block_128_tiling_loop),
                (time_2d_vec, time_2d_vec_block_256_tiling_loop),
            ],
        );
    }

    println!(
        "Interesting outcome: notice the knee around 65536, in line with l1 cache size again of 128k"
    );
    println!(
        "Notice that the more increasingly stagerred and extreme the l1 cache misses become the more Vec<Vec<T>> dominates Row-Major 1d Vec<T>"
    );

    ///////////////////////////////////////////////////////////////////////////////
    println!("\n\nAnd now with 1d looping block tiling");
    let columns = [
        "Data Size",
        "  2d (ns)",
        "1d RM(ns)",
        "Ratio", // Ratio values getting a bit big so hacking a bit of space
        "  2d (ns)",
        "1d RM Blocks(64^2) (ns)",
        "Ratio", // Ratio values getting a bit big so hacking a bit of space
        "  2d (ns)",
        "1d RM Blocks(128^2) (ns)",
        "Ratio", // Ratio values getting a bit big so hacking a bit of space
        "  2d (ns)",
        "1d RM Blocks(256^2) (ns)",
        "Ratio", // Ratio values getting a bit big so hacking a bit of space
    ];
    print_header(&columns);
    let start_size = 32;
    let number_of_doubles = 9;
    // starting at
    let arr_sizes: Vec<usize> = (0..number_of_doubles).map(|i| start_size << i).collect();

    for size in &arr_sizes {
        let mut arr_time_2d_vec: Vec<Duration> = Vec::with_capacity(10);
        let mut arr_time_1d_row_major_vec: Vec<Duration> = Vec::with_capacity(10);
        let mut arr_time_1d_row_major_vec_block_64_tile_loop: Vec<Duration> =
            Vec::with_capacity(10);
        let mut arr_time_1d_row_major_vec_block_128_tile_loop: Vec<Duration> =
            Vec::with_capacity(10);
        let mut arr_time_1d_row_major_vec_block_256_tile_loop: Vec<Duration> =
            Vec::with_capacity(10);

        for _ in 0..10 {
            let matrix = make_matrix(*size, *size, 0);
            let start = Instant::now();
            let _transposed = transpose_2d_vec_poor_cache_locality(matrix);
            std::hint::black_box(_transposed);
            arr_time_2d_vec.push(start.elapsed());
        }
        for _ in 0..10 {
            let matrix = make_matrix(*size, *size, 0);
            let flat_row_major_matrix = flatten_row_major(&matrix);
            let start = Instant::now();
            let _transposed =
                transpose_1d_row_major_matrix_poor_cache_locality(flat_row_major_matrix, *size);
            std::hint::black_box(_transposed);
            arr_time_1d_row_major_vec.push(start.elapsed());
        }

        for _ in 0..10 {
            let matrix = make_matrix(*size, *size, 0);
            let flat_row_major_matrix = flatten_row_major(&matrix);
            let start = Instant::now();
            // Trying 32x32 blocks first.
            let _transposed = transpose_1d_row_major_matrix_loop_tiling_aka_block_tiling(
                flat_row_major_matrix,
                *size,
                64,
            );
            std::hint::black_box(_transposed);
            arr_time_1d_row_major_vec_block_64_tile_loop.push(start.elapsed());
        }

        for _ in 0..10 {
            let matrix = make_matrix(*size, *size, 0);
            let flat_row_major_matrix = flatten_row_major(&matrix);
            let start = Instant::now();
            // Trying 32x32 blocks first.
            let _transposed = transpose_1d_row_major_matrix_loop_tiling_aka_block_tiling(
                flat_row_major_matrix,
                *size,
                128,
            );
            std::hint::black_box(_transposed);
            arr_time_1d_row_major_vec_block_128_tile_loop.push(start.elapsed());
        }

        for _ in 0..10 {
            let matrix = make_matrix(*size, *size, 0);
            let flat_row_major_matrix = flatten_row_major(&matrix);
            let start = Instant::now();
            // Trying 32x32 blocks first.
            let _transposed = transpose_1d_row_major_matrix_loop_tiling_aka_block_tiling(
                flat_row_major_matrix,
                *size,
                256,
            );
            std::hint::black_box(_transposed);
            arr_time_1d_row_major_vec_block_256_tile_loop.push(start.elapsed());
        }

        let time_2d_vec = Duration::from_nanos(
            (arr_time_2d_vec.iter().map(|d| d.as_nanos()).sum::<u128>()
                / arr_time_2d_vec.len() as u128) as u64,
        );

        let time_1d_row_major_vec = Duration::from_nanos(
            (arr_time_1d_row_major_vec
                .iter()
                .map(|d| d.as_nanos())
                .sum::<u128>()
                / arr_time_1d_row_major_vec.len() as u128) as u64,
        );

        let time_1d_row_major_vec_block_64_tiling_loop = Duration::from_nanos(
            (arr_time_1d_row_major_vec_block_64_tile_loop
                .iter()
                .map(|d| d.as_nanos())
                .sum::<u128>()
                / arr_time_1d_row_major_vec_block_64_tile_loop.len() as u128) as u64,
        );

        let time_1d_row_major_vec_block_128_tiling_loop = Duration::from_nanos(
            (arr_time_1d_row_major_vec_block_128_tile_loop
                .iter()
                .map(|d| d.as_nanos())
                .sum::<u128>()
                / arr_time_1d_row_major_vec_block_128_tile_loop.len() as u128) as u64,
        );

        let time_1d_row_major_vec_block_256_tiling_loop = Duration::from_nanos(
            (arr_time_1d_row_major_vec_block_256_tile_loop
                .iter()
                .map(|d| d.as_nanos())
                .sum::<u128>()
                / arr_time_1d_row_major_vec_block_256_tile_loop.len() as u128) as u64,
        );

        print_output_row_ratio_compare_result_nanos(
            &columns,
            *size * *size,
            vec![
                (time_2d_vec, time_1d_row_major_vec),
                (time_2d_vec, time_1d_row_major_vec_block_64_tiling_loop),
                (time_2d_vec, time_1d_row_major_vec_block_128_tiling_loop),
                (time_2d_vec, time_1d_row_major_vec_block_256_tiling_loop),
            ],
        );
    }

    ///////////////////////////////////////////////////////////////////////////////
    println!("\n\nAnd now with 1d looping block tiling write buffer version");
    let columns = [
        "Data Size",
        "  2d (ns)",
        "1d RM(ns)",
        "Ratio", // Ratio values getting a bit big so hacking a bit of space
        "  2d (ns)",
        "1d RM Blocks(64^2) (WriteBuffer) (ns)",
        "Ratio", // Ratio values getting a bit big so hacking a bit of space
        "  2d (ns)",
        "1d RM Blocks(128^2) (WriteBuffer)  (ns)",
        "Ratio", // Ratio values getting a bit big so hacking a bit of space
        "  2d (ns)",
        "1d RM Blocks(256^2) (WriteBuffer) (ns)",
        "Ratio", // Ratio values getting a bit big so hacking a bit of space
    ];
    print_header(&columns);
    let start_size = 32;
    let number_of_doubles = 9;
    // starting at
    let arr_sizes: Vec<usize> = (0..number_of_doubles).map(|i| start_size << i).collect();

    for size in &arr_sizes {
        let mut arr_time_2d_vec: Vec<Duration> = Vec::with_capacity(10);
        let mut arr_time_1d_row_major_vec: Vec<Duration> = Vec::with_capacity(10);
        let mut arr_time_1d_row_major_vec_block_64_tile_loop: Vec<Duration> =
            Vec::with_capacity(10);
        let mut arr_time_1d_row_major_vec_block_128_tile_loop: Vec<Duration> =
            Vec::with_capacity(10);
        let mut arr_time_1d_row_major_vec_block_256_tile_loop: Vec<Duration> =
            Vec::with_capacity(10);

        for _ in 0..10 {
            let matrix = make_matrix(*size, *size, 0);
            let start = Instant::now();
            let _transposed = transpose_2d_vec_poor_cache_locality(matrix);
            std::hint::black_box(_transposed);
            arr_time_2d_vec.push(start.elapsed());
        }
        for _ in 0..10 {
            let matrix = make_matrix(*size, *size, 0);
            let flat_row_major_matrix = flatten_row_major(&matrix);
            let start = Instant::now();
            let _transposed =
                transpose_1d_row_major_matrix_poor_cache_locality(flat_row_major_matrix, *size);
            std::hint::black_box(_transposed);
            arr_time_1d_row_major_vec.push(start.elapsed());
        }

        for _ in 0..10 {
            let matrix = make_matrix(*size, *size, 0);
            let flat_row_major_matrix = flatten_row_major(&matrix);
            let start = Instant::now();
            // Trying 32x32 blocks first.
            let _transposed = transpose_square_tiled(flat_row_major_matrix, *size, 64);
            std::hint::black_box(_transposed);
            arr_time_1d_row_major_vec_block_64_tile_loop.push(start.elapsed());
        }

        for _ in 0..10 {
            let matrix = make_matrix(*size, *size, 0);
            let flat_row_major_matrix = flatten_row_major(&matrix);
            let start = Instant::now();
            // Trying 32x32 blocks first.
            let _transposed = transpose_square_tiled(flat_row_major_matrix, *size, 128);
            std::hint::black_box(_transposed);
            arr_time_1d_row_major_vec_block_128_tile_loop.push(start.elapsed());
        }

        for _ in 0..10 {
            let matrix = make_matrix(*size, *size, 0);
            let flat_row_major_matrix = flatten_row_major(&matrix);
            let start = Instant::now();
            // Trying 32x32 blocks first.
            let _transposed = transpose_square_tiled(flat_row_major_matrix, *size, 256);
            std::hint::black_box(_transposed);
            arr_time_1d_row_major_vec_block_256_tile_loop.push(start.elapsed());
        }

        let time_2d_vec = Duration::from_nanos(
            (arr_time_2d_vec.iter().map(|d| d.as_nanos()).sum::<u128>()
                / arr_time_2d_vec.len() as u128) as u64,
        );

        let time_1d_row_major_vec = Duration::from_nanos(
            (arr_time_1d_row_major_vec
                .iter()
                .map(|d| d.as_nanos())
                .sum::<u128>()
                / arr_time_1d_row_major_vec.len() as u128) as u64,
        );

        let time_1d_row_major_vec_block_64_tiling_loop = Duration::from_nanos(
            (arr_time_1d_row_major_vec_block_64_tile_loop
                .iter()
                .map(|d| d.as_nanos())
                .sum::<u128>()
                / arr_time_1d_row_major_vec_block_64_tile_loop.len() as u128) as u64,
        );

        let time_1d_row_major_vec_block_128_tiling_loop = Duration::from_nanos(
            (arr_time_1d_row_major_vec_block_128_tile_loop
                .iter()
                .map(|d| d.as_nanos())
                .sum::<u128>()
                / arr_time_1d_row_major_vec_block_128_tile_loop.len() as u128) as u64,
        );

        let time_1d_row_major_vec_block_256_tiling_loop = Duration::from_nanos(
            (arr_time_1d_row_major_vec_block_256_tile_loop
                .iter()
                .map(|d| d.as_nanos())
                .sum::<u128>()
                / arr_time_1d_row_major_vec_block_256_tile_loop.len() as u128) as u64,
        );

        print_output_row_ratio_compare_result_nanos(
            &columns,
            *size * *size,
            vec![
                (time_2d_vec, time_1d_row_major_vec),
                (time_2d_vec, time_1d_row_major_vec_block_64_tiling_loop),
                (time_2d_vec, time_1d_row_major_vec_block_128_tiling_loop),
                (time_2d_vec, time_1d_row_major_vec_block_256_tiling_loop),
            ],
        );
    }

    println!("So what did we learn? We learned that when strided access will be ");
    println!("occuring no matter what then Vec<Vec<T>> dominates 1d Row Major Vec<T> ");
    println!("structures due to a happy accident that each sub Vec becomes its own ");
    println!("small continous block apart from the source being read. The result appears ");
    println!("to be that the line being read and written to is expelled less.");
    println!("\n In short - 1d Row Major is better for reads but far worse for writes.");
    println!("          - Vec<Vec<T> is worse for reads (since they are strided by row)");
    println!("            but as a happy co-incidence better also as they are less prone");
    println!("            to cache thrashing on writes.");
    println!("This is of course just my speculation at this point.");
}

fn transpose_impl() {
    println!("---> Transposing matrix (square)");
    let col_count = 10;
    let row_count = 10;
    let matrix = make_matrix(row_count, col_count, 0);
    println!("Before Matrix 2d Vec<Vec<T>>:");
    print_matrix_2d_vec_matrix(&matrix);
    let trasposed_matrix = transpose_2d_vec_poor_cache_locality(matrix);
    println!("After 2d naive Vec<Vec<T>>:");
    print_matrix_2d_vec_matrix(&trasposed_matrix);

    let matrix = make_matrix(row_count, col_count, 0);
    println!("Before 2d Vec Matrix:");
    print_matrix_2d_vec_matrix(&matrix);
    println!("Before 1d Vec Row-Major Matrix:");
    let flatened_row_major_1d_matrix = flatten_row_major(&matrix);
    print_matrix_1d_vec_matrix(&flatened_row_major_1d_matrix, col_count);
    let transposed_row_major_1d_matrix =
        transpose_1d_row_major_matrix_poor_cache_locality(flatened_row_major_1d_matrix, col_count);
    println!("After 1d Vec Row-Major Matrix:");
    print_matrix_1d_vec_matrix(&transposed_row_major_1d_matrix, col_count);

    let matrix = make_matrix(row_count, col_count, 0);
    println!("Before Matrix 2d with block looping Vec<Vec<T>>:");
    print_matrix_2d_vec_matrix(&matrix);
    //3x3 blocks
    let trasposed_matrix = transpose_2d_vec_loop_tiling_aka_block_tiling(matrix, 3);
    println!("After 2d with block looping Vec<Vec<T>>:");
    print_matrix_2d_vec_matrix(&trasposed_matrix);

    let matrix = make_matrix(row_count, col_count, 0);
    println!("Before 1d Vec Row-Major Matrix with block looping:");
    let flatened_row_major_1d_matrix = flatten_row_major(&matrix);
    print_matrix_1d_vec_matrix(&flatened_row_major_1d_matrix, col_count);
    let transposed_row_major_1d_matrix = transpose_1d_row_major_matrix_loop_tiling_aka_block_tiling(
        flatened_row_major_1d_matrix,
        col_count,
        3,
    );
    println!("After 1d Vec Row-Major Matrix with block looping:");
    print_matrix_1d_vec_matrix(&transposed_row_major_1d_matrix, col_count);

    let matrix = make_matrix(row_count, col_count, 0);
    println!("Before 1d Vec Row-Major Matrix with block looping transpose_square_tiled:");
    let flatened_row_major_1d_matrix = flatten_row_major(&matrix);
    print_matrix_1d_vec_matrix(&flatened_row_major_1d_matrix, col_count);
    let transposed_row_major_1d_matrix =
        transpose_square_tiled(flatened_row_major_1d_matrix, col_count, 3);
    println!("After 1d Vec Row-Major Matrix with block looping transpose_square_tiled:");
    print_matrix_1d_vec_matrix(&transposed_row_major_1d_matrix, col_count);
}

fn transpose_1d_row_major_matrix_loop_tiling_aka_block_tiling<T: Copy>(
    mut flat_matrix_row_major_1d: Vec<T>,
    col_count: usize,
    block_size: usize,
) -> Vec<T> {
    let row_count = flat_matrix_row_major_1d.len() / col_count;

    if col_count < 1 || row_count < 1 {
        panic!("Matrix must be a 2d matrix with at least 1 element.");
    }

    if row_count == col_count {
        // This is an inplace swap so excluding the diagonals we only need to swap
        // on a half and it will auto swap the other half as a result.

        // Break the matrix into blocks first with block row/col index co-ordinates

        for block_start_row_idx in (0..row_count).step_by(block_size) {
            for block_start_col_idx in (block_start_row_idx..col_count).step_by(block_size) {
                // Determine the row/cell ends (some blocks get chopped off)
                let row_end = (block_start_row_idx + block_size).min(row_count);
                let col_end = (block_start_col_idx + block_size).min(row_count);

                // Determine if the block being processed is on the diagonal
                if block_start_col_idx == block_start_row_idx {
                    // We are on the diagonal so only swap half the cells.
                    // Transpose top right half
                    for cell_row_idx in block_start_row_idx..row_end {
                        for cell_col_idx in (cell_row_idx + 1)..col_end {
                            let source_idx = cell_row_idx * col_count + cell_col_idx;
                            let target_idx = cell_col_idx * col_count + cell_row_idx;
                            flat_matrix_row_major_1d.swap(source_idx, target_idx);
                        }
                    }
                } else {
                    // We are not on the diagonal so swap all cells
                    // Transpose the entire block
                    for cell_row_idx in block_start_row_idx..row_end {
                        for cell_col_idx in block_start_col_idx..col_end {
                            let source_idx = cell_row_idx * col_count + cell_col_idx;
                            let target_idx = cell_col_idx * col_count + cell_row_idx;
                            flat_matrix_row_major_1d.swap(source_idx, target_idx);
                        }
                    }
                }
            }
        }
        flat_matrix_row_major_1d
    } else {
        // Make new rows vector based on amount of columns
        let mut new_flat_matrix_row_major_1d = Vec::with_capacity(col_count * row_count);

        for block_start_row_idx in (0..row_count).step_by(block_size) {
            for block_start_col_idx in (0..col_count).step_by(block_size) {
                // Determine the row/cell ends (some blocks get chopped off)
                let row_end = (block_start_row_idx + block_size).min(row_count);
                let col_end = (block_start_col_idx + block_size).min(row_count);

                // Transpose the block
                for cell_row_idx in block_start_row_idx..row_end {
                    for cell_col_idx in block_start_col_idx..col_end {
                        let source_idx = cell_row_idx * col_count + cell_col_idx;
                        let target_idx = cell_col_idx * col_count + cell_row_idx;
                        new_flat_matrix_row_major_1d[target_idx] =
                            flat_matrix_row_major_1d[source_idx];
                    }
                }
            }
        }
        new_flat_matrix_row_major_1d
    }
}

// Enhanced version square tiled transpose found
// I need to study these differences still.
// Looks like its using an intermediate buffer to write to before writing that buffer into the strided destination...
// I'm not so sure this is going to be much faster - but what do I know I just test and observe the speed.
//
// Ok I did the benchmarks it is a lie that this is faster it is the same performance.
fn transpose_square_tiled<T: Copy>(mut mat: Vec<T>, n: usize, block: usize) -> Vec<T> {
    let mut buf = vec![MaybeUninit::<T>::uninit(); block * block];

    for i in (0..n).step_by(block) {
        for j in (0..n).step_by(block) {
            let row_end = (i + block).min(n);
            let col_end = (j + block).min(n);

            if i == j {
                // 🔹 Diagonal block: transpose inside a scratch buffer
                for r in i..row_end {
                    for c in j..col_end {
                        buf[(r - i) * block + (c - j)].write(mat[r * n + c]);
                    }
                }
                for r in i..row_end {
                    for c in j..col_end {
                        unsafe {
                            mat[r * n + c] = buf[(c - j) * block + (r - i)].assume_init();
                        }
                    }
                }
            } else if i < j {
                // 🔹 Off-diagonal tile pair (i,j) and (j,i)
                for r in i..row_end {
                    for c in j..col_end {
                        buf[(r - i) * block + (c - j)].write(mat[r * n + c]); // copy tile (i,j)
                    }
                }

                for r in j..col_end {
                    for c in i..row_end {
                        let tmp = mat[r * n + c]; // (j,i) tile element
                        unsafe {
                            mat[r * n + c] = buf[(c - i) * block + (r - j)].assume_init(); // from (i,j)
                        }
                        mat[c * n + r] = tmp; // write into (i,j)
                    }
                }
            }
        }
    }
    mat
}

// This is the optimal solution for matrices that need transposing.
// Vec<Vec<T>> is the best structure it wins because of the strided
// access on the writes. The sub Vec<T> instances play better with
// the cache when writing.
// Best block size on my processor (128K L1) means a square
// matrix performs best with a 64 block size, while a
// rectangle performs better with a 32 block size. The difference
// is due to the square matrix doing in place swapping, while the
// rectangle has to copy to a destination matrix.
fn transpose_2d_vec_loop_tiling_aka_block_tiling<T: Copy>(
    mut matrix: Vec<Vec<T>>,
    block_size: usize,
) -> Vec<Vec<T>> {
    let input_row_count = matrix.len();

    if input_row_count < 1 || matrix[0].len() < 1 {
        panic!("Matrix must be a 2d matrix with at least 1 element.");
    }

    let input_col_count = matrix[0].len();

    if matrix.len() == matrix[0].len() {
        // This is an inplace swap so excluding the diagonals we only need to swap
        // on a half and it will auto swap the other half as a result.

        // Break the matrix into blocks first with block row/col index co-ordinates

        for block_start_row_idx in (0..input_row_count).step_by(block_size) {
            for block_start_col_idx in (block_start_row_idx..input_col_count).step_by(block_size) {
                // Determine the row/cell ends (some blocks get chopped off)
                let row_end = (block_start_row_idx + block_size).min(input_row_count);
                let col_end = (block_start_col_idx + block_size).min(input_row_count);

                // Determine if the block being processed is on the diagonal
                if block_start_col_idx == block_start_row_idx {
                    // We are on the diagonal so only swap half the cells.
                    // Transpose top right half
                    for cell_row_idx in block_start_row_idx..row_end {
                        let (top, bottom) = matrix.split_at_mut(cell_row_idx + 1);
                        for cell_col_idx in (cell_row_idx + 1)..col_end {
                            std::mem::swap(
                                &mut top[cell_row_idx][cell_col_idx],
                                &mut bottom[cell_col_idx - (cell_row_idx + 1)][cell_row_idx],
                            );
                        }
                    }
                } else {
                    // We are not on the diagonal so swap all cells
                    // Transpose the entire block
                    for cell_row_idx in block_start_row_idx..row_end {
                        let (top, bottom) = matrix.split_at_mut(cell_row_idx + 1);
                        for cell_col_idx in block_start_col_idx..col_end {
                            std::mem::swap(
                                &mut top[cell_row_idx][cell_col_idx],
                                &mut bottom[cell_col_idx - (cell_row_idx + 1)][cell_row_idx],
                            );
                        }
                    }
                }
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
        for block_start_row_idx in (0..input_row_count).step_by(block_size) {
            for block_start_col_idx in (0..input_col_count).step_by(block_size) {
                // Determine the row/cell ends (some blocks get chopped off)
                let row_end = (block_start_row_idx + block_size).min(input_row_count);
                let col_end = (block_start_col_idx + block_size).min(input_row_count);

                // Transpose the block
                for cell_row_idx in block_start_row_idx..row_end {
                    for cell_col_idx in block_start_col_idx..col_end {
                        new_matrix[cell_col_idx][cell_row_idx] = matrix[cell_row_idx][cell_col_idx];
                    }
                }
            }
        }
        new_matrix
    }
}

fn transpose_1d_row_major_matrix_poor_cache_locality(
    mut flat_matrix_row_major_1d: Vec<i32>,
    col_count: usize,
) -> Vec<i32> {
    let row_count = flat_matrix_row_major_1d.len() / col_count;

    if row_count < 1 || col_count < 1 {
        panic!("Matrix must be a 2d matrix with at least 1 element.");
    }

    if row_count == col_count {
        // This is an inplace swap so excluding the diagonals we only need to swap
        // on a half and it will auto swap the other half as a result.

        // for each row
        for idx_row in 0..row_count {
            for idx_col in idx_row + 1..col_count {
                let source_idx = idx_row * col_count + idx_col;
                let target_idx = idx_col * col_count + idx_row;
                flat_matrix_row_major_1d.swap(source_idx, target_idx);
            }
        }
        flat_matrix_row_major_1d
    } else {
        // Make new rows vector based on amount of columns
        let mut new_flat_matrix_row_major_1d = Vec::with_capacity(col_count * row_count);
        for idx_row in 0..row_count {
            for idx_col in 0..col_count {
                let source_idx = idx_row * col_count + idx_col;
                let target_idx = idx_col * col_count + idx_row;
                new_flat_matrix_row_major_1d
                    .insert(target_idx, flat_matrix_row_major_1d[source_idx]);
            }
        }
        new_flat_matrix_row_major_1d
    }
}

fn print_matrix_1d_vec_matrix(flat_row_major_matrix: &Vec<i32>, col_count: usize) {
    print!(" {:>4} |", "idx");
    for col_idx in 0..col_count {
        print!(" {:>4} |", col_idx);
    }
    print!("\n");
    for _ in 0..=col_count {
        print!("------|");
    }
    print!("\n");
    for row_idx in 0..(flat_row_major_matrix.len() / col_count) {
        print!(" {:>4} |", row_idx);
        for col_idx in 0..col_count {
            print!(
                " {:>4} |",
                get_row_major(flat_row_major_matrix, row_idx, col_idx, col_count)
            );
        }
        print!("\n");
    }
    println!("\n");
}

fn print_matrix_2d_vec_matrix(matrix: &Vec<Vec<i32>>) {
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
fn transpose_2d_vec_poor_cache_locality<T: Clone>(mut matrix: Vec<Vec<T>>) -> Vec<Vec<T>> {
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
