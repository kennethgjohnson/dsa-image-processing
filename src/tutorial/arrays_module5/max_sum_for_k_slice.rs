use crate::tutorial::{
    arrays_module5::prefix_sum_arrays::{make_prefix_sum_array, range_sum_prefix_sum_arr},
    common_util::{
        create_array, median_duration_index_u128, print_header,
        print_output_row_ratio_compare_result,
    },
};
use std::time::{Duration, Instant};

// Fixed Sliding Window
pub fn max_sum_for_k_slice() {
    println!("==> Sliding Window (Fixed) - max sum in fixed sliding window over array");
    let columns = [
        "Data Size",
        "Naive Time (µs)",
        "Prefix Sum ArrayTime (µs)",
        "      Ratio", // Ratio values getting a bit big so hacking a bit of space
    ];
    print_header(&columns);
    let size_interval = 1;
    let size_count = 17;
    let lower_threshold = 16;
    // starting at
    let arr_sizes: Vec<usize> = (0..size_count)
        .map(|i| size_interval << i)
        .filter(|element| *element >= lower_threshold)
        .collect();
    for size in arr_sizes {
        let mut arr_time_naive: Vec<Duration> = Vec::with_capacity(10);
        let mut arr_time_using_prefix_sum_array: Vec<Duration> = Vec::with_capacity(1000);
        let arr = create_array(size);
        let arr_prefix = make_prefix_sum_array(&arr);
        let k = (arr.len() / 2).max(1); // k = n/2 = worst case
        for _ in 0..10 {
            let start = Instant::now();
            max_sum_fixed_window_naive(&arr, k);
            arr_time_naive.push(start.elapsed());
        }
        for _ in 0..1000 {
            let start = Instant::now();
            max_sum_fixed_window_using_prefix_sum(&arr_prefix, k);
            arr_time_using_prefix_sum_array.push(start.elapsed());
        }

        let time_naive = arr_time_naive[median_duration_index_u128(&arr_time_naive)];
        let time_using_prefix_sum_array = Duration::from_nanos(
            (arr_time_using_prefix_sum_array
                .iter()
                .map(|d| d.as_nanos())
                .sum::<u128>()
                / arr_time_using_prefix_sum_array.len() as u128) as u64,
        );
        print_output_row_ratio_compare_result(
            &columns,
            size,
            vec![(time_naive, time_using_prefix_sum_array)],
        );
    }
    print!("\n\n");
}

// prefix sum approach O(n)
fn max_sum_fixed_window_using_prefix_sum(arr_prefix_sum: &[i32], k: usize) -> i32 {
    if k == 0 {
        panic!("window size must be > 0")
    }
    if arr_prefix_sum.len() < k {
        panic!("window size must be <= array length")
    }

    // Starting it off at the minimum posible i32 since we will be comparing
    // against if for the max
    let mut max_sum: i32 = i32::MIN;
    for left in 0..=(arr_prefix_sum.len() - k) {
        let right = left + k - 1;
        let sum = range_sum_prefix_sum_arr(&arr_prefix_sum, left, right);
        max_sum = max_sum.max(sum);
    }
    max_sum
}

// Naive approach - O(k*(n-k+1))
// O(k*(n-k+1)) -> O(k*(n-k))
// -> O(kn - k^2) downwards parabola quadratic
// -> Find the worst case scenario:
// -> T(k) = kn - k^2
// -> T(k) = -k^2 + kn
// In a quadratic ax² + bx + c, the maximum/minimum occurs at:
// x = −b / (2a)
// In our quadratic
// a = -1, b = n, c = 0
// k = -n / -2
// k = n/2
// -> at k = n/2 the upside down parabola reaches it's maximum value.
// -> T(n/2) = (n/2) * (n - n/2)
// T(n/2) = (n/2) * (1n - (1/2)n)
// T(n/2) = (n/2) * (1/2)n
// T(n/2) = n/2 * n/2
// T(n/2) = n^2 / 4
// So worst case scenario at n/2 the function is growing at n^2 / 4, big O simplification O(n^2)
fn max_sum_fixed_window_naive(arr: &[i32], k: usize) -> i32 {
    if k == 0 {
        panic!("window size must be > 0")
    }
    let arr_len = arr.len();
    if arr.len() < k {
        panic!("window size must be <= array length")
    }
    let mut left: usize = 0;
    let mut right: usize = left + k - 1;

    // Starting it off at the minimum posible i32 since we will be comparing
    // against if for the max
    let mut max_sum: i32 = i32::MIN;
    for _ in 0..=(arr_len - k) {
        // let sum: i32 = arr[left..=right].iter().sum();
        let mut sum = 0;
        for i in left..=right {
            sum += arr[i];
        }
        if sum > max_sum {
            max_sum = sum;
        }
        left += 1;
        right += 1;
    }
    max_sum
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_sum_fixed_window_prefix_sum_basic() {
        let arr = vec![1, 3, 5, 2, 8, 1, 5];
        let k = 3;
        // Possible windows: [1,3,5]=9, [3,5,2]=10, [5,2,8]=15, [2,8,1]=11, [8,1,5]=14
        assert_eq!(max_sum_fixed_window_naive(&arr, k), 15);
    }

    #[test]
    fn test_max_sum_fixed_window_prefix_sum_exact_length() {
        let arr = vec![4, 4, 4];
        let k = 3;
        // Only one window possible
        assert_eq!(max_sum_fixed_window_naive(&arr, k), 12);
    }

    #[test]
    fn test_max_sum_fixed_window_prefix_sum_k_equals_1() {
        let arr = vec![7, 2, 5, 10];
        let k = 1;
        assert_eq!(max_sum_fixed_window_naive(&arr, k), 10);
    }

    #[test]
    fn test_max_sum_fixed_window_prefix_sum_k_equals_len() {
        let arr = vec![1, 2, 3, 4, 5];
        let k = arr.len();
        assert_eq!(max_sum_fixed_window_naive(&arr, k), 15);
    }

    #[test]
    #[should_panic(expected = "window size must be <= array length")]
    fn test_max_sum_fixed_window_prefix_sum_k_too_large() {
        let arr = vec![1, 2];
        let k = 3;
        max_sum_fixed_window_naive(&arr, k);
    }

    #[test]
    #[should_panic(expected = "window size must be > 0")]
    fn test_max_sum_fixed_window_prefix_sum_zero_k() {
        let arr = vec![1, 2, 3];
        max_sum_fixed_window_naive(&arr, 0);
    }

    //////
    ///
    ///
    ///
    #[test]
    fn test_max_sum_fixed_window_naive_basic() {
        let arr = vec![1, 3, 5, 2, 8, 1, 5];
        let k = 3;
        // Possible windows: [1,3,5]=9, [3,5,2]=10, [5,2,8]=15, [2,8,1]=11, [8,1,5]=14
        assert_eq!(max_sum_fixed_window_naive(&arr, k), 15);
    }

    #[test]
    fn test_max_sum_fixed_window_naive_exact_length() {
        let arr = vec![4, 4, 4];
        let k = 3;
        // Only one window possible
        assert_eq!(max_sum_fixed_window_naive(&arr, k), 12);
    }

    #[test]
    fn test_max_sum_fixed_window_naive_k_equals_1() {
        let arr = vec![7, 2, 5, 10];
        let k = 1;
        assert_eq!(max_sum_fixed_window_naive(&arr, k), 10);
    }

    #[test]
    fn test_max_sum_fixed_window_naive_k_equals_len() {
        let arr = vec![1, 2, 3, 4, 5];
        let k = arr.len();
        assert_eq!(max_sum_fixed_window_naive(&arr, k), 15);
    }

    #[test]
    #[should_panic(expected = "window size must be <= array length")]
    fn test_max_sum_fixed_window_naive_k_too_large() {
        let arr = vec![1, 2];
        let k = 3;
        max_sum_fixed_window_naive(&arr, k);
    }

    #[test]
    #[should_panic(expected = "window size must be > 0")]
    fn test_max_sum_fixed_window_naive_zero_k() {
        let arr = vec![1, 2, 3];
        max_sum_fixed_window_naive(&arr, 0);
    }
}
