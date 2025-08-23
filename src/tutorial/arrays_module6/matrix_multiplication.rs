use std::time::{Duration, Instant};

use crate::tutorial::{
    arrays_module6::{
        basic_matrix_operations::transpose::transpose_2d_vec_loop_tiling_aka_block_tiling,
        warm_up_2d_arrays_and_flattening_them::{flatten_col_major, flatten_row_major},
    },
    common_util::{
        make_matrix, print_header, print_matrix_1d_vec_matrix, print_matrix_2d_vec_matrix,
        print_output_row_ratio_compare_result_nanos,
    },
};

pub fn matrix_multiplication() {
    println!("==> 3) Matrix Multiplication\n");
    // 3) Matrix Multiplication
    // 3a) Naive Triple-loop (Investigate inefficiencies with innerlooop jump around)
    matrix_multiplication_triple_loop();
    // 3b) Optimize: precompute transpose(B) and use it to get cache-friendly row access.
    matrix_multiplication_triple_loop_precompute_transpose_b();
    // 3c) Loop Tiling / Blocking in Matrix Multiplication
    matrix_multiplication_loop_tiling_blocking();

    // 3c) Loop Tiling / Blocking in Matrix Multiplication (1d row-major A and B)
    matrix_multiplication_loop_tiling_blocking_flat_row_major();
    // 3c) Loop Tiling / Blocking in Matrix Multiplication (1d row-major A and 1d col-major B)
    matrix_multiplication_loop_tiling_blocking_flat_row_major_a_with_col_major_b();

    // 3d) Compare the performance of all 3 approaches using The performance comparison
    // suggestion of all three matrix multiplication approaches
    matrix_multiplication_performance_compare();
}

fn matrix_multiplication_triple_loop() {
    println!("--> Vec_2d Tripple Loop");
    let a_row_count = 5;
    // must match
    let a_col_count = 10;
    let b_row_count = 10;
    //
    let b_col_count = 6;

    let matrix_a_2d_vec = make_matrix(a_row_count, a_col_count, 0);
    let matrix_b_2d_vec = make_matrix(b_row_count, b_col_count, 0);

    println!("[A] = ");
    print_matrix_2d_vec_matrix(&matrix_a_2d_vec);
    println!("[B] = ");
    print_matrix_2d_vec_matrix(&matrix_b_2d_vec);
    println!("Performing [A]*[B] = [C]");
    let matrix_c_2d_vec = matrix_multiply_triple_loop_vec_2d(&matrix_a_2d_vec, &matrix_b_2d_vec);
    println!("[C] = ");
    print_matrix_2d_vec_matrix(&matrix_c_2d_vec);
}

fn matrix_multiply_triple_loop_vec_2d(
    matrix_a: &Vec<Vec<i32>>,
    matrix_b: &Vec<Vec<i32>>,
) -> Vec<Vec<i32>> {
    let a_col_count = matrix_a[0].len();
    let b_row_count = matrix_b.len();
    if a_col_count != b_row_count {
        panic!("Can not multiply matrices [A]'s columns do not match [B]'s rows.")
    }
    let a_row_count = matrix_a.len();
    let b_col_count = matrix_b[0].len();

    // a_col_count == b_row_count == k
    let k = a_col_count;
    let mut matrix_c = vec![vec![0; b_col_count]; a_row_count];

    for a_row_idx in 0..a_row_count {
        for b_col_idx in 0..b_col_count {
            let mut total = 0;
            for k_idx in 0..k {
                total += matrix_a[a_row_idx][k_idx] * matrix_b[k_idx][b_col_idx];
            }
            matrix_c[a_row_idx][b_col_idx] = total;
        }
    }
    matrix_c
}

fn matrix_multiplication_triple_loop_precompute_transpose_b() {
    println!("--> Vec_2d Tripple Loop Pre-compute transpose [B]");
    let a_row_count = 5;
    // must match
    let a_col_count = 10;
    let b_row_count = 10;
    //
    let b_col_count = 6;

    let matrix_a_2d_vec = make_matrix(a_row_count, a_col_count, 0);
    let matrix_b_2d_vec = make_matrix(b_row_count, b_col_count, 0);

    println!("[A] = ");
    print_matrix_2d_vec_matrix(&matrix_a_2d_vec);
    println!("[B] = ");
    print_matrix_2d_vec_matrix(&matrix_b_2d_vec);
    println!("Transposing B");
    let matrix_b_2d_vec_transposed =
        transpose_2d_vec_loop_tiling_aka_block_tiling(matrix_b_2d_vec, 32);
    println!("[B transposed] = ");
    print_matrix_2d_vec_matrix(&matrix_b_2d_vec_transposed);
    println!("Performing [A]*[B] = [C] using transposed precompute of [B]");
    let matrix_c_2d_vec = matrix_multiply_triple_loop_vec_2d_with_transposed_b(
        &matrix_a_2d_vec,
        &matrix_b_2d_vec_transposed,
    );
    println!("[C] = ");
    print_matrix_2d_vec_matrix(&matrix_c_2d_vec);
}

fn matrix_multiply_triple_loop_vec_2d_with_transposed_b(
    matrix_a: &Vec<Vec<i32>>,
    matrix_b_transposed: &Vec<Vec<i32>>,
) -> Vec<Vec<i32>> {
    let a_col_count = matrix_a[0].len();
    let b_row_count = matrix_b_transposed[0].len();
    if a_col_count != b_row_count {
        panic!("Can not multiply matrices [A]'s columns do not match [B]'s rows.")
    }
    let a_row_count = matrix_a.len();
    let b_col_count = matrix_b_transposed.len();

    // a_col_count == b_row_count == k
    let k = a_col_count;
    let mut matrix_c = vec![vec![0; b_col_count]; a_row_count];

    for a_row_idx in 0..a_row_count {
        let a_row = &matrix_a[a_row_idx];
        for b_col_idx in 0..b_col_count {
            // Since b is transposed, the row is a the column.
            let b_column = &matrix_b_transposed[b_col_idx];
            for k_idx in 0..k {
                matrix_c[a_row_idx][b_col_idx] += a_row[k_idx] * b_column[k_idx];
            }
        }
    }
    matrix_c
}

fn matrix_multiplication_loop_tiling_blocking() {
    println!("--> Vec_2d Loop Tiling / Blocking");
    let a_row_count = 5;
    // must match
    let a_col_count = 10;
    let b_row_count = 10;
    //
    let b_col_count = 6;

    let matrix_a_2d_vec = make_matrix(a_row_count, a_col_count, 0);
    let matrix_b_2d_vec = make_matrix(b_row_count, b_col_count, 0);

    println!("[A] = ");
    print_matrix_2d_vec_matrix(&matrix_a_2d_vec);
    println!("[B] = ");
    print_matrix_2d_vec_matrix(&matrix_b_2d_vec);
    println!("Performing [A]*[B] = [C] using loop tiling");
    let matrix_c_2d_vec = matrix_multiply_loop_tiling(&matrix_a_2d_vec, &matrix_b_2d_vec, 32);
    println!("[C] = ");
    print_matrix_2d_vec_matrix(&matrix_c_2d_vec);
}

// Just got this straight out of a article, need to ponder this to understand it.
// A good way would be to make a matrix A/B/C visuallizer highlighting the parts
// being worked on to see it visually: I think the main thing tripping me up here
// is understanding the for block_k and for k_in_block loops.
fn matrix_multiply_loop_tiling(
    matrix_a: &Vec<Vec<i32>>,
    matrix_b: &Vec<Vec<i32>>,
    block_size: usize,
) -> Vec<Vec<i32>> {
    let a_col_count = matrix_a[0].len();
    let b_row_count = matrix_b.len();
    if a_col_count != b_row_count {
        panic!("Can not multiply matrices [A]'s columns do not match [B]'s rows.")
    }
    let a_row_count = matrix_a.len();
    let b_col_count = matrix_b[0].len();

    // a_col_count == b_row_count == k
    let k = a_col_count;
    let mut matrix_c = vec![vec![0; b_col_count]; a_row_count];

    // Iterate over blocks of rows in C (and rows in A)
    for block_row_c in (0..a_row_count).step_by(block_size) {
        // Iterate over blocks of columns in C (and columns in B)
        for block_col_c in (0..b_col_count).step_by(block_size) {
            // Iterate over blocks along the inner dimension (columns of A / rows of B)
            for block_k in (0..k).step_by(block_size) {
                // Iterate over rows inside the current C block
                for row_in_c_block in block_row_c..(block_row_c + block_size).min(a_row_count) {
                    // Iterate over columns inside the current C block
                    for col_in_c_block in block_col_c..(block_col_c + block_size).min(b_col_count) {
                        // Iterate along the inner dimension inside the block
                        for k_in_block in block_k..(block_k + block_size).min(a_col_count) {
                            // Multiply and accumulate the corresponding elements
                            matrix_c[row_in_c_block][col_in_c_block] += matrix_a[row_in_c_block]
                                [k_in_block]
                                * matrix_b[k_in_block][col_in_c_block];
                        }
                    }
                }
            }
        }
    }
    matrix_c
}

fn matrix_multiplication_loop_tiling_blocking_flat_row_major() {
    println!("--> 1drow major Loop Tiling / Blocking");
    let a_row_count = 5;
    // must match
    let a_col_count = 10;
    let b_row_count = 10;
    //
    let b_col_count = 6;

    let matrix_a_2d_vec = make_matrix(a_row_count, a_col_count, 0);
    let matrix_b_2d_vec = make_matrix(b_row_count, b_col_count, 0);
    let matrix_a_flat = flatten_row_major(&matrix_a_2d_vec);
    let matrix_b_flat = flatten_row_major(&matrix_b_2d_vec);

    println!("[A] = ");
    print_matrix_1d_vec_matrix(&matrix_a_flat, a_col_count);
    println!("[B] = ");
    print_matrix_1d_vec_matrix(&matrix_b_flat, b_col_count);
    println!("Performing [A]*[B] = [C] using loop tiling");
    let matrix_c_flat = matrix_multiply_loop_tiling_flat_row_major(
        &matrix_a_flat,
        &matrix_b_flat,
        a_row_count,
        a_col_count,
        b_col_count,
        32,
    );
    println!("[C] = ");
    print_matrix_1d_vec_matrix(&matrix_c_flat, b_col_count);
}

// Just got this straight out of a article, need to ponder this to understand it.
/// Matrix multiplication with loop tiling (blocking),
/// using flat row-major Vec<i32> for storage.
fn matrix_multiply_loop_tiling_flat_row_major(
    matrix_a: &Vec<i32>, // row-major [a_row_count × a_col_count]
    matrix_b: &Vec<i32>, // row-major [b_row_count × b_col_count]
    a_row_count: usize,
    a_col_count: usize,
    b_col_count: usize,
    block_size: usize,
) -> Vec<i32> {
    let b_row_count = a_col_count;
    if matrix_b.len() != b_row_count * b_col_count {
        panic!("Matrix B dimensions do not match multiplication requirements.");
    }

    // Output matrix C [a_row_count × b_col_count], row-major
    let mut matrix_c = vec![0; a_row_count * b_col_count];

    // Iterate over blocks of rows in C (and rows in A)
    for block_row_c in (0..a_row_count).step_by(block_size) {
        // Iterate over blocks of columns in C (and columns in B)
        for block_col_c in (0..b_col_count).step_by(block_size) {
            // Iterate over blocks along the inner dimension (columns of A / rows of B)
            for block_k in (0..a_col_count).step_by(block_size) {
                // Rows inside the current C block
                for row_in_c_block in block_row_c..(block_row_c + block_size).min(a_row_count) {
                    // Cols inside the current C block
                    for col_in_c_block in block_col_c..(block_col_c + block_size).min(b_col_count) {
                        let mut sum = matrix_c[row_in_c_block * b_col_count + col_in_c_block];
                        // Inner dimension inside the block
                        for k_in_block in block_k..(block_k + block_size).min(a_col_count) {
                            let a_val = matrix_a[row_in_c_block * a_col_count + k_in_block];
                            let b_val = matrix_b[k_in_block * b_col_count + col_in_c_block];
                            sum += a_val * b_val;
                        }
                        matrix_c[row_in_c_block * b_col_count + col_in_c_block] = sum;
                    }
                }
            }
        }
    }

    matrix_c
}

fn matrix_multiplication_loop_tiling_blocking_flat_row_major_a_with_col_major_b() {
    println!("--> 1drow major Loop Tiling / Blocking but with b being 1dcol major");
    let a_row_count = 5;
    // must match
    let a_col_count = 10;
    let b_row_count = 10;
    //
    let b_col_count = 6;

    let matrix_a_2d_vec = make_matrix(a_row_count, a_col_count, 0);
    let matrix_b_2d_vec = make_matrix(b_row_count, b_col_count, 0);
    let matrix_a_flat = flatten_row_major(&matrix_a_2d_vec);
    let matrix_b_flat = flatten_col_major(&matrix_b_2d_vec);

    println!("[A] = ");
    print_matrix_1d_vec_matrix(&matrix_a_flat, a_col_count);
    println!("[B] = ");
    print_matrix_1d_vec_matrix(&matrix_b_flat, b_col_count);
    println!("Performing [A]*[B] = [C] using loop tiling");
    let matrix_c_flat = matrix_multiply_loop_tiling_flat_row_major_a_col_major_b(
        &matrix_a_flat,
        &matrix_b_flat,
        a_row_count,
        a_col_count,
        b_col_count,
        32,
    );
    println!("[C] = ");
    print_matrix_1d_vec_matrix(&matrix_c_flat, b_col_count);
}

/// Matrix multiplication with loop tiling (blocking),
/// using flat row-major Vec<i32> for A and C,
/// and flat column-major Vec<i32> for B.
fn matrix_multiply_loop_tiling_flat_row_major_a_col_major_b(
    matrix_a: &Vec<i32>, // row-major [a_row_count × a_col_count]
    matrix_b: &Vec<i32>, // column-major [b_row_count × b_col_count]
    a_row_count: usize,
    a_col_count: usize,
    b_col_count: usize,
    block_size: usize,
) -> Vec<i32> {
    let b_row_count = a_col_count;
    if matrix_b.len() != b_row_count * b_col_count {
        panic!("Matrix B dimensions do not match multiplication requirements.");
    }

    // Output matrix C [a_row_count × b_col_count], row-major
    let mut matrix_c = vec![0; a_row_count * b_col_count];

    // Iterate over blocks of rows in C (and rows in A)
    for block_row_c in (0..a_row_count).step_by(block_size) {
        // Iterate over blocks of columns in C (and columns in B)
        for block_col_c in (0..b_col_count).step_by(block_size) {
            // Iterate over blocks along the inner dimension (columns of A / rows of B)
            for block_k in (0..a_col_count).step_by(block_size) {
                // Rows inside the current C block
                for row_in_c_block in block_row_c..(block_row_c + block_size).min(a_row_count) {
                    // Cols inside the current C block
                    for col_in_c_block in block_col_c..(block_col_c + block_size).min(b_col_count) {
                        let mut sum = matrix_c[row_in_c_block * b_col_count + col_in_c_block];
                        // Inner dimension inside the block
                        for k_in_block in block_k..(block_k + block_size).min(a_col_count) {
                            let a_val = matrix_a[row_in_c_block * a_col_count + k_in_block];
                            // <-- difference: column-major indexing
                            let b_val = matrix_b[col_in_c_block * b_row_count + k_in_block];
                            sum += a_val * b_val;
                        }
                        matrix_c[row_in_c_block * b_col_count + col_in_c_block] = sum;
                    }
                }
            }
        }
    }

    matrix_c
}

fn matrix_multiplication_performance_compare() {
    //
    println!("---> Matrix Multiplication Benchmarks (Vec<Vec<i32>> Loop Tiles)");
    let columns = [
        "Data Size",
        "Triple Loop (ns)",
        "Transpose [B] (ns)",
        "Ratio", // Ratio values getting a bit big so hacking a bit of space
        "Triple Loop (ns)",
        "Loop Tile (32^2) (ns)",
        "Ratio", // Ratio values getting a bit big so hacking a bit of space
        "Triple Loop (ns)",
        "Loop Tile (64^2) (ns)",
        "Ratio", // Ratio values getting a bit big so hacking a bit of space
        "Triple Loop (ns)",
        "Loop Tile (128^2) (ns)",
        "Ratio", // Ratio values getting a bit big so hacking a bit of space
    ];
    print_header(&columns);
    let start_size = 8;
    let number_of_doubles = 7;
    // starting at
    let arr_sizes: Vec<usize> = (0..number_of_doubles).map(|i| start_size << i).collect();

    for size in &arr_sizes {
        let mut arr_time_tl: Vec<Duration> = Vec::with_capacity(10);
        let mut arr_time_tl_tp_b: Vec<Duration> = Vec::with_capacity(10);
        let mut arr_time_block_32_tile_loop: Vec<Duration> = Vec::with_capacity(10);
        let mut arr_time_block_64_tile_loop: Vec<Duration> = Vec::with_capacity(10);
        let mut arr_time_block_128_tile_loop: Vec<Duration> = Vec::with_capacity(10);
        let matrix_a = make_matrix(*size / 2, *size, 0);
        let matrix_b = make_matrix(*size, *size / 2, 0);
        let matrix_b_transposed = transpose_2d_vec_loop_tiling_aka_block_tiling(matrix_b, 32);
        let matrix_b = make_matrix(*size, *size / 2, 0);

        for _ in 0..10 {
            let start = Instant::now();
            let _matrix_c = matrix_multiply_triple_loop_vec_2d(&matrix_a, &matrix_b);
            std::hint::black_box(_matrix_c);
            arr_time_tl.push(start.elapsed());
        }
        for _ in 0..10 {
            let start = Instant::now();
            let _matrix_c = matrix_multiply_triple_loop_vec_2d_with_transposed_b(
                &matrix_a,
                &matrix_b_transposed,
            );
            std::hint::black_box(_matrix_c);
            arr_time_tl_tp_b.push(start.elapsed());
        }

        for _ in 0..10 {
            let start = Instant::now();

            let _matrix_c = matrix_multiply_loop_tiling(&matrix_a, &matrix_b, 32);
            std::hint::black_box(_matrix_c);
            arr_time_block_32_tile_loop.push(start.elapsed());
        }

        for _ in 0..10 {
            let start = Instant::now();

            let _matrix_c = matrix_multiply_loop_tiling(&matrix_a, &matrix_b, 64);
            std::hint::black_box(_matrix_c);
            arr_time_block_64_tile_loop.push(start.elapsed());
        }

        for _ in 0..10 {
            let start = Instant::now();

            let _matrix_c = matrix_multiply_loop_tiling(&matrix_a, &matrix_b, 128);
            std::hint::black_box(_matrix_c);
            arr_time_block_128_tile_loop.push(start.elapsed());
        }

        let time_tl = Duration::from_nanos(
            (arr_time_tl.iter().map(|d| d.as_nanos()).sum::<u128>() / arr_time_tl.len() as u128)
                as u64,
        );

        let time_tl_tp_b = Duration::from_nanos(
            (arr_time_tl_tp_b.iter().map(|d| d.as_nanos()).sum::<u128>()
                / arr_time_tl_tp_b.len() as u128) as u64,
        );

        let time_loop_tile_32 = Duration::from_nanos(
            (arr_time_block_32_tile_loop
                .iter()
                .map(|d| d.as_nanos())
                .sum::<u128>()
                / arr_time_block_32_tile_loop.len() as u128) as u64,
        );

        let time_loop_tile_64 = Duration::from_nanos(
            (arr_time_block_64_tile_loop
                .iter()
                .map(|d| d.as_nanos())
                .sum::<u128>()
                / arr_time_block_64_tile_loop.len() as u128) as u64,
        );

        let time_loop_tile_128 = Duration::from_nanos(
            (arr_time_block_128_tile_loop
                .iter()
                .map(|d| d.as_nanos())
                .sum::<u128>()
                / arr_time_block_128_tile_loop.len() as u128) as u64,
        );

        print_output_row_ratio_compare_result_nanos(
            &columns,
            (*size / 2) * *size / 2,
            vec![
                (time_tl, time_tl_tp_b),
                (time_tl, time_loop_tile_32),
                (time_tl, time_loop_tile_64),
                (time_tl, time_loop_tile_128),
            ],
        );
    }
    println!("\nNow let's try with a flattened 1d Vec<i32> Row Major A and B into C...");
    println!("---> Matrix Multiplication Benchmarks (Vec<i32> row major Loop Tiles)");
    let columns = [
        "Data Size",
        "Triple Loop (ns)",
        "Loop Tile (32^2) (ns)",
        "Ratio", // Ratio values getting a bit big so hacking a bit of space
        "Triple Loop (ns)",
        "Loop Tile (64^2) (ns)",
        "Ratio", // Ratio values getting a bit big so hacking a bit of space
        "Triple Loop (ns)",
        "Loop Tile (128^2) (ns)",
        "Ratio", // Ratio values getting a bit big so hacking a bit of space
    ];
    print_header(&columns);
    let start_size = 8;
    let number_of_doubles = 8;
    // starting at
    let arr_sizes: Vec<usize> = (0..number_of_doubles).map(|i| start_size << i).collect();

    for size in &arr_sizes {
        let mut arr_time_tl: Vec<Duration> = Vec::with_capacity(10);
        let mut arr_time_block_32_tile_loop: Vec<Duration> = Vec::with_capacity(10);
        let mut arr_time_block_64_tile_loop: Vec<Duration> = Vec::with_capacity(10);
        let mut arr_time_block_128_tile_loop: Vec<Duration> = Vec::with_capacity(10);
        let a_row_count = *size / 2;
        let b_col_count = a_row_count;
        let a_col_count = *size;
        let b_row_count = *size;
        let matrix_a = make_matrix(a_row_count, a_col_count, 0);
        let matrix_b = make_matrix(b_row_count, b_col_count, 0);
        let matrix_a_flat = flatten_row_major(&matrix_a);
        let matrix_b_flat = flatten_row_major(&matrix_b);

        for _ in 0..10 {
            let start = Instant::now();
            let _matrix_c = matrix_multiply_triple_loop_vec_2d(&matrix_a, &matrix_b);
            std::hint::black_box(_matrix_c);
            arr_time_tl.push(start.elapsed());
        }

        for _ in 0..10 {
            let start = Instant::now();

            let _matrix_c = matrix_multiply_loop_tiling_flat_row_major(
                &matrix_a_flat,
                &matrix_b_flat,
                a_row_count,
                a_col_count,
                b_col_count,
                32,
            );
            std::hint::black_box(_matrix_c);
            arr_time_block_32_tile_loop.push(start.elapsed());
        }

        for _ in 0..10 {
            let start = Instant::now();

            let _matrix_c = matrix_multiply_loop_tiling_flat_row_major(
                &matrix_a_flat,
                &matrix_b_flat,
                a_row_count,
                a_col_count,
                b_col_count,
                64,
            );
            std::hint::black_box(_matrix_c);
            arr_time_block_64_tile_loop.push(start.elapsed());
        }

        for _ in 0..10 {
            let start = Instant::now();

            let _matrix_c = matrix_multiply_loop_tiling_flat_row_major(
                &matrix_a_flat,
                &matrix_b_flat,
                a_row_count,
                a_col_count,
                b_col_count,
                128,
            );
            std::hint::black_box(_matrix_c);
            arr_time_block_128_tile_loop.push(start.elapsed());
        }

        let time_tl = Duration::from_nanos(
            (arr_time_tl.iter().map(|d| d.as_nanos()).sum::<u128>() / arr_time_tl.len() as u128)
                as u64,
        );

        let time_loop_tile_32 = Duration::from_nanos(
            (arr_time_block_32_tile_loop
                .iter()
                .map(|d| d.as_nanos())
                .sum::<u128>()
                / arr_time_block_32_tile_loop.len() as u128) as u64,
        );

        let time_loop_tile_64 = Duration::from_nanos(
            (arr_time_block_64_tile_loop
                .iter()
                .map(|d| d.as_nanos())
                .sum::<u128>()
                / arr_time_block_64_tile_loop.len() as u128) as u64,
        );

        let time_loop_tile_128 = Duration::from_nanos(
            (arr_time_block_128_tile_loop
                .iter()
                .map(|d| d.as_nanos())
                .sum::<u128>()
                / arr_time_block_128_tile_loop.len() as u128) as u64,
        );

        print_output_row_ratio_compare_result_nanos(
            &columns,
            (*size / 2) * *size / 2,
            vec![
                (time_tl, time_loop_tile_32),
                (time_tl, time_loop_tile_64),
                (time_tl, time_loop_tile_128),
            ],
        );
    }
    println!(
        "\nThese benchmarks are madening, I don't get whats the point of 
blocks with loop tiling if it only operates faster for matrixes of 64x64..
doesn't make sense there must be something wrong with these algo's I'll 
come back later..., WTF screw it let me try making B col major:"
    );

    println!(
        "---> Matrix Multiplication Benchmarks (Vec<i32> row major * Vec<i32> col major Loop Tiles)"
    );
    let columns = [
        "Data Size",
        "Triple Loop (ns)",
        "Loop Tile (32^2) (ns)",
        "Ratio", // Ratio values getting a bit big so hacking a bit of space
        "Triple Loop (ns)",
        "Loop Tile (64^2) (ns)",
        "Ratio", // Ratio values getting a bit big so hacking a bit of space
        "Triple Loop (ns)",
        "Loop Tile (128^2) (ns)",
        "Ratio", // Ratio values getting a bit big so hacking a bit of space
    ];
    print_header(&columns);
    let start_size = 8;
    let number_of_doubles = 8;
    // starting at
    let arr_sizes: Vec<usize> = (0..number_of_doubles).map(|i| start_size << i).collect();

    for size in &arr_sizes {
        let mut arr_time_tl: Vec<Duration> = Vec::with_capacity(10);
        let mut arr_time_block_32_tile_loop: Vec<Duration> = Vec::with_capacity(10);
        let mut arr_time_block_64_tile_loop: Vec<Duration> = Vec::with_capacity(10);
        let mut arr_time_block_128_tile_loop: Vec<Duration> = Vec::with_capacity(10);
        let a_row_count = *size / 2;
        let b_col_count = a_row_count;
        let a_col_count = *size;
        let b_row_count = *size;
        let matrix_a = make_matrix(a_row_count, a_col_count, 0);
        let matrix_b = make_matrix(b_row_count, b_col_count, 0);
        let matrix_a_flat_row_major = flatten_row_major(&matrix_a);
        let matrix_b_flat_col_major = flatten_col_major(&matrix_b);

        for _ in 0..10 {
            let start = Instant::now();
            let _matrix_c = matrix_multiply_triple_loop_vec_2d(&matrix_a, &matrix_b);
            std::hint::black_box(_matrix_c);
            arr_time_tl.push(start.elapsed());
        }

        for _ in 0..10 {
            let start = Instant::now();
            let _matrix_c = matrix_multiply_loop_tiling_flat_row_major_a_col_major_b(
                &matrix_a_flat_row_major,
                &matrix_b_flat_col_major,
                a_row_count,
                a_col_count,
                b_col_count,
                32,
            );
            std::hint::black_box(_matrix_c);
            arr_time_block_32_tile_loop.push(start.elapsed());
        }

        for _ in 0..10 {
            let start = Instant::now();
            let _matrix_c = matrix_multiply_loop_tiling_flat_row_major_a_col_major_b(
                &matrix_a_flat_row_major,
                &matrix_b_flat_col_major,
                a_row_count,
                a_col_count,
                b_col_count,
                64,
            );
            std::hint::black_box(_matrix_c);
            arr_time_block_64_tile_loop.push(start.elapsed());
        }

        for _ in 0..10 {
            let start = Instant::now();

            let _matrix_c = matrix_multiply_loop_tiling_flat_row_major_a_col_major_b(
                &matrix_a_flat_row_major,
                &matrix_b_flat_col_major,
                a_row_count,
                a_col_count,
                b_col_count,
                128,
            );
            std::hint::black_box(_matrix_c);
            arr_time_block_128_tile_loop.push(start.elapsed());
        }

        let time_tl = Duration::from_nanos(
            (arr_time_tl.iter().map(|d| d.as_nanos()).sum::<u128>() / arr_time_tl.len() as u128)
                as u64,
        );

        let time_loop_tile_32 = Duration::from_nanos(
            (arr_time_block_32_tile_loop
                .iter()
                .map(|d| d.as_nanos())
                .sum::<u128>()
                / arr_time_block_32_tile_loop.len() as u128) as u64,
        );

        let time_loop_tile_64 = Duration::from_nanos(
            (arr_time_block_64_tile_loop
                .iter()
                .map(|d| d.as_nanos())
                .sum::<u128>()
                / arr_time_block_64_tile_loop.len() as u128) as u64,
        );

        let time_loop_tile_128 = Duration::from_nanos(
            (arr_time_block_128_tile_loop
                .iter()
                .map(|d| d.as_nanos())
                .sum::<u128>()
                / arr_time_block_128_tile_loop.len() as u128) as u64,
        );

        print_output_row_ratio_compare_result_nanos(
            &columns,
            (*size / 2) * *size / 2,
            vec![
                (time_tl, time_loop_tile_32),
                (time_tl, time_loop_tile_64),
                (time_tl, time_loop_tile_128),
            ],
        );
    }

    println!(
        "\nFinally results I can live with, here we see that 1d block tiling, combined
        with a 1d flattened col-major B matrix is up to 6.5x faster on big data structures
        this was the value on my system for the last row I'm not sure how much it scales higher but
        my long running output looked like this last time so it definitely gets better at large sizes
        than triple loops:
---> Matrix Multiplication Benchmarks (Vec<i32> row major * Vec<i32> col major Loop Tiles)
 Data Size | Triple Loop (ns) | Loop Tile (32^2) (ns) | Ratio | Triple Loop (ns) | Loop Tile (64^2) (ns) | Ratio | Triple Loop (ns) | Loop Tile (128^2) (ns) | Ratio
-----------|------------------|-----------------------|-------|------------------|-----------------------|-------|------------------|------------------------|-------
 16        |              820 |                   290 |  2.8x |              820 |                   210 |  3.9x |              820 |                    200 |  4.1x
 64        |             1240 |                   640 |  1.9x |             1240 |                   550 |  2.3x |             1240 |                    580 |  2.1x
 256       |             6290 |                  3260 |  1.9x |             6290 |                  2730 |  2.3x |             6290 |                   2720 |  2.3x
 1024      |            43230 |                 21160 |  2.0x |            43230 |                 16220 |  2.7x |            43230 |                  16250 |  2.7x
 4096      |           350230 |                164970 |  2.1x |           350230 |                128690 |  2.7x |           350230 |                 110700 |  3.2x
 16384     |          2842350 |               1347350 |  2.1x |          2842350 |               1059120 |  2.7x |          2842350 |                 906640 |  3.1x
 65536     |         28879930 |              10617850 |  2.7x |         28879930 |               8423040 |  3.4x |         28879930 |                7373110 |  3.9x
 262144    |        267693800 |              85644070 |  3.1x |        267693800 |              69101560 |  3.9x |        267693800 |               61035090 |  4.4x
 1048576   |       4076774170 |             723973150 |  5.6x |       4076774170 |             569111340 |  7.2x |       4076774170 |              583272550 |  7.0x
 4194304   |      78664407060 |            5731472960 | 13.7x |      78664407060 |            5978903970 | 13.2x |      78664407060 |             5549987740 | 14.2x"
    );
}
