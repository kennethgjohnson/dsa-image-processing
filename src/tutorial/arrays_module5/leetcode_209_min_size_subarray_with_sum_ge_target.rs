use crate::tutorial::{
    arrays_module5::{
        min_slice_len_which_sums_ge_target::min_slice_len_which_sums_ge_target_using_variable_sliding_window_and_prefix_array_for_sum,
        prefix_sum_arrays::{make_prefix_sum_array, range_sum_prefix_sum_arr},
    },
    common_util::{create_array, print_header, print_output_row_ratio_compare_result},
};
use std::{
    i32,
    time::{Duration, Instant},
};

// Leetcode 209 Sliding Window (Variable) with prefix array
// Functionally the same as min_slice_len_which_sums_ge_target, but we will
// use this opertunity to build our prefix array while processing the array
// instead of passing it in.
pub fn bonus_leetcode_209_min_size_subarray_with_sum_ge_target() {
    println!(
        "==> Bonus Leetcode 209 Sliding Window (Variable) - min sliding window that's sum over array is ≥ target (inline compute vs pre-compute)"
    );
    let columns = [
        "Data Size",
        "pre-computed (µs)",
        "inline-computed (µs)",
        "      Ratio", // Ratio values getting a bit big so hacking a bit of space
    ];
    print_header(&columns);
    let start_size = 16_384;
    let number_of_doubles = 6;
    // starting at
    let arr_sizes: Vec<usize> = (0..number_of_doubles).map(|i| start_size << i).collect();

    for size in &arr_sizes {
        let mut arr_time_using_prefix_sum_array_pre_computed: Vec<Duration> =
            Vec::with_capacity(1000);
        let mut arr_time_using_prefix_sum_array_inline_computed: Vec<Duration> =
            Vec::with_capacity(1000);
        let arr = create_array(*size);

        let target = i32::MAX; // Worst case scenario for naive, but never stresses variable sliding window.

        for _ in 0..1000 {
            let start = Instant::now();
            let arr_prefix_sum = make_prefix_sum_array(&arr);
            min_slice_len_which_sums_ge_target_using_variable_sliding_window_and_prefix_array_for_sum(
                &arr_prefix_sum,
                target,
            );
            arr_time_using_prefix_sum_array_pre_computed.push(start.elapsed());
        }

        for _ in 0..1000 {
            let start = Instant::now();
            min_sub_array_len(target, &arr);
            arr_time_using_prefix_sum_array_inline_computed.push(start.elapsed());
        }

        let time_using_prefix_sum_array_pre_computed = Duration::from_nanos(
            (arr_time_using_prefix_sum_array_pre_computed
                .iter()
                .map(|d| d.as_nanos())
                .sum::<u128>()
                / arr_time_using_prefix_sum_array_pre_computed.len() as u128) as u64,
        );

        let time_using_prefix_sum_array_inline_computed = Duration::from_nanos(
            (arr_time_using_prefix_sum_array_inline_computed
                .iter()
                .map(|d| d.as_nanos())
                .sum::<u128>()
                / arr_time_using_prefix_sum_array_inline_computed.len() as u128) as u64,
        );

        print_output_row_ratio_compare_result(
            &columns,
            *size,
            vec![(
                time_using_prefix_sum_array_pre_computed,
                time_using_prefix_sum_array_inline_computed,
            )],
        );
    }

    println!("Now with amortised cost over 1000 itterations for the pre-computed version.");
    let columns = [
        "Data Size",
        "pre-computed (µs)",
        "inline-computed (µs)",
        "      Ratio", // Ratio values getting a bit big so hacking a bit of space
    ];
    print_header(&columns);
    let start_size = 16_384;
    let number_of_doubles = 6;
    // starting at
    let arr_sizes: Vec<usize> = (0..number_of_doubles).map(|i| start_size << i).collect();

    for size in &arr_sizes {
        let mut arr_time_using_prefix_sum_array_pre_computed: Vec<Duration> =
            Vec::with_capacity(1000);
        let mut arr_time_using_prefix_sum_array_inline_computed: Vec<Duration> =
            Vec::with_capacity(1000);
        let arr = create_array(*size);

        let start = Instant::now();
        let arr_prefix_sum = make_prefix_sum_array(&arr);
        let fixed_pre_compute_time = start.elapsed();

        let target = i32::MAX; // Worst case scenario for naive, but never stresses variable sliding window.

        for _ in 0..1000 {
            let start = Instant::now();

            min_slice_len_which_sums_ge_target_using_variable_sliding_window_and_prefix_array_for_sum(
                &arr_prefix_sum,
                target,
            );
            arr_time_using_prefix_sum_array_pre_computed.push(start.elapsed());
        }

        for _ in 0..1000 {
            let start = Instant::now();
            min_sub_array_len(target, &arr);
            arr_time_using_prefix_sum_array_inline_computed.push(start.elapsed());
        }

        let time_using_prefix_sum_array_pre_computed = Duration::from_nanos(
            (arr_time_using_prefix_sum_array_pre_computed
                .iter()
                .map(|d| d.as_nanos())
                .sum::<u128>()
                / arr_time_using_prefix_sum_array_pre_computed.len() as u128) as u64,
        );
        let amortised_pre_compute_time = (fixed_pre_compute_time.as_nanos() / 1000) as u64;

        let time_using_prefix_sum_array_pre_computed = Duration::from_nanos(
            amortised_pre_compute_time + time_using_prefix_sum_array_pre_computed.as_nanos() as u64,
        );

        let time_using_prefix_sum_array_inline_computed = Duration::from_nanos(
            (arr_time_using_prefix_sum_array_inline_computed
                .iter()
                .map(|d| d.as_nanos())
                .sum::<u128>()
                / arr_time_using_prefix_sum_array_inline_computed.len() as u128) as u64,
        );

        print_output_row_ratio_compare_result(
            &columns,
            *size,
            vec![(
                time_using_prefix_sum_array_pre_computed,
                time_using_prefix_sum_array_inline_computed,
            )],
        );
    }

    print!("\n\n");

    println!(
        "The benifit of amortizing the cost of prefix arrays are obvious for work that repeats on the same data."
    )
}

enum SearchMode {
    ExpandRight,
    ContractLeft,
    SlideRight,
}

pub fn min_sub_array_len(target: i32, nums: &[i32]) -> i32 {
    // nums is positive integers
    // target is posive integer
    // find length of shortest slice, that's sum is >= target
    // if it doesnt exist return 0
    if nums.len() == 0 {
        return 0;
    }

    let mut arr_prefix = Vec::with_capacity(nums.len());

    let mut l = 0;
    let mut r: usize = 0;
    let mut prefix_total = 0;

    let mut search_mode = SearchMode::ExpandRight;
    let mut result = 0;
    loop {
        match search_mode {
            SearchMode::ExpandRight => {
                // Find the first slice that puts us at or over the target
                r += 1;
                prefix_total += nums[r - 1];
                arr_prefix.push(prefix_total);

                if range_sum_prefix_sum_arr(&arr_prefix, l, r - 1) >= target {
                    // We are at a healthy slice size and can start contracting the left
                    search_mode = SearchMode::ContractLeft;
                    result = r - l;
                } else {
                    if r == nums.len() {
                        // We tried to find a qualifying window but none were
                        // found: our work is done.
                        break;
                    }
                }
            }
            SearchMode::ContractLeft => {
                l += 1;
                if range_sum_prefix_sum_arr(&arr_prefix, l, r - 1) < target {
                    // We found the current minimum slice size so we can now
                    // slide the whole window as a fixed window right until we
                    // find this smaller window which qualifies to our right.
                    if r == nums.len() {
                        // We were at the last token and have just completed
                        // contracting the left: our work is done.
                        break;
                    }
                    search_mode = SearchMode::SlideRight;
                } else {
                    // We contracted from the left and found a smaller window
                    result = r - l;
                }
            }
            SearchMode::SlideRight => {
                l += 1;
                r += 1;
                // expand prefix array
                prefix_total += nums[r - 1];
                arr_prefix.push(prefix_total);
                if range_sum_prefix_sum_arr(&arr_prefix, l, r - 1) >= target {
                    // We are at a healthy slice size and can start contracting the left
                    search_mode = SearchMode::ContractLeft;
                    result = r - l;
                } else {
                    if r == nums.len() {
                        // We are at the last token and could not find a qualifying smaller
                        // window: our work is done
                        break;
                    }
                }
            }
        }
    }
    return result as i32;
}
