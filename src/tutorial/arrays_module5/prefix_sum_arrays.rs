use crate::tutorial::common_util::{
    create_array, print_header, print_output_row_ratio_compare_result,
};
use std::time::Instant;
pub fn arrays_module5_fundamental_patterns_in_optimization_prefix_sum_arrays() {
    println!("==> Prefix-Sum Arrays");
    let columns = [
        "Data Size",
        "Naive Time (µs)",
        "Prefix Array Time (µs)",
        "      Ratio", // Ratio values getting a bit big so hacking a bit of space
    ];
    print_header(&columns);
    let size_interval = 1000;
    let size_count = 18;
    // How many itterations to double from size_interval
    let lower_threshold = 16_000;
    // starting at
    let arr_sizes: Vec<usize> = (0..size_count)
        .map(|i| size_interval << i)
        .filter(|element| *element >= lower_threshold)
        .collect();
    for size in arr_sizes {
        let arr = create_array(size);
        let prefix_sum_arr = make_prefix_sum_array(&arr); // Note this precomputes O(n) but O(1) amortised
        let start = Instant::now();
        range_sum_naive(&arr, 1, size - 1);
        let time_naive = start.elapsed();
        let start = Instant::now();
        range_sum_prefix_sum_arr(&prefix_sum_arr, 1, size - 1);
        let time = start.elapsed();
        print_output_row_ratio_compare_result(&columns, size, vec![(time_naive, time)]);
    }
    print!("\n\n");
}

pub fn make_prefix_sum_array(arr: &[i32]) -> Vec<i32> {
    let size = arr.len();
    if size == 0 {
        return Vec::new();
    }

    let mut prefix_sum_array = Vec::with_capacity(size);
    prefix_sum_array.push(arr[0]);
    for i in 1..size {
        prefix_sum_array.push(prefix_sum_array[i - 1] + arr[i]);
    }
    prefix_sum_array
}

// Naive range sum - results in O(n) time complexity
pub fn range_sum_naive(arr: &[i32], l: usize, r: usize) -> i32 {
    let mut total = 0;
    for i in l..=r {
        total += arr[i];
    }
    total
}

// O(1) implementation using prefix_arr
pub fn range_sum_prefix_sum_arr(prefix_arr: &[i32], l: usize, r: usize) -> i32 {
    if l == 0 {
        prefix_arr[r]
    } else {
        prefix_arr[r] - prefix_arr[l - 1]
    }
}

#[test]
fn test_range_sum_naive() {
    let arr = vec![2, 4, 6, 8, 10];

    let cases = vec![
        (0, 0, 2),  // sum of [2] = 2
        (0, 2, 12), // sum of [2,4,6] = 12
        (1, 3, 18), // sum of [4,6,8] = 18
        (2, 4, 24), // sum of [6,8,10] = 24
        (0, 4, 30), // sum of entire array = 30
    ];

    for (l, r, expected) in cases {
        let result = range_sum_naive(&arr, l, r);
        assert_eq!(result, expected, "Failed for range ({}, {})", l, r);
    }
}

#[test]
fn test_range_sum_prefix_sum_arr() {
    let arr = vec![2, 4, 6, 8, 10];
    let prefix_arr = make_prefix_sum_array(&arr);

    let cases = vec![
        (0, 0, 2),  // sum of [2] = 2
        (0, 2, 12), // sum of [2,4,6] = 12
        (1, 3, 18), // sum of [4,6,8] = 18
        (2, 4, 24), // sum of [6,8,10] = 24
        (0, 4, 30), // sum of entire array = 30
    ];

    for (l, r, expected) in cases {
        let result = range_sum_prefix_sum_arr(&prefix_arr, l, r);
        assert_eq!(result, expected, "Failed for range ({}, {})", l, r);
    }
}

#[test]
fn test_prefix_array_sums() {
    let cases = vec![
        // (input, expected_prefix)
        (vec![], vec![]),
        (vec![5], vec![5]),
        (vec![1, 2, 3], vec![1, 3, 6]),
        (vec![2, 4, 6, 8, 10], vec![2, 6, 12, 20, 30]),
        (vec![10, -2, 3, -1], vec![10, 8, 11, 10]),
        (vec![-1, -2, -3], vec![-1, -3, -6]),
    ];

    for (input, expected) in cases {
        let output = make_prefix_sum_array(&input);
        assert_eq!(output, expected, "Failed on input: {:?}", input);
    }
}
