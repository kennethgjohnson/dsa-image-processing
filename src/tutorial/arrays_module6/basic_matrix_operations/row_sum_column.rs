use crate::tutorial::common_util::{
    make_matrix, print_header, print_output_row_ratio_compare_result_nanos,
};
use std::time::{Duration, Instant};

pub fn row_sum_column_sum() {
    row_sum_column_sum_impl();
    row_sum_column_sum_benchmarks();
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
