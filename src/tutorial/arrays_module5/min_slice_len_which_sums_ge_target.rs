use crate::tutorial::{
    arrays_module5::prefix_sum_arrays::{make_prefix_sum_array, range_sum_prefix_sum_arr},
    common_util::{
        create_array, median_duration_index_u128, print_header,
        print_output_row_ratio_compare_result,
    },
};
use std::{
    i32,
    time::{Duration, Instant},
};

// Variable Sliding Window Implementation
// Examples of O(n^3) - naive, O(n^2) - using prefix sums, and
//   O(n) using a special sliding window with prefix sums
pub fn min_slice_len_which_sums_ge_target() {
    println!(
        "==> Sliding Window (Variable) - min sliding window that's sum over array is ≥ target"
    );
    println!("--> Naive vs Prefix Sum vs Var Sliding Window");
    let columns = [
        "Data Size",
        "Naive (µs)",
        "Prefix Sum (µs)",
        "      Ratio", // Ratio values getting a bit big so hacking a bit of space
        "Prefix Sum (µs)",
        "Var Sliding Window Prefix Sum (µs)",
        "      Ratio", // Ratio values getting a bit big so hacking a bit of space
    ];
    print_header(&columns);
    let start_size = 8;
    let number_of_doubles = 9;
    // starting at
    let arr_sizes: Vec<usize> = (0..number_of_doubles).map(|i| start_size << i).collect();

    for size in &arr_sizes {
        let mut arr_time_naive: Vec<Duration> = Vec::with_capacity(10);
        let mut arr_time_using_prefix_sum_array: Vec<Duration> = Vec::with_capacity(1000);
        let mut arr_time_using_prefix_sum_array_and_variable_sliding_window: Vec<Duration> =
            Vec::with_capacity(1000);
        let arr = create_array(*size);
        let arr_prefix_sum = make_prefix_sum_array(&arr);
        let target = i32::MAX; // Worst case scenario for naive, but never stresses variable sliding window.
        // let target = 8_000_000; // Picked to be more fair.
        for _ in 0..10 {
            let start = Instant::now();
            min_slice_len_which_sums_ge_target_naive(&arr, target);
            arr_time_naive.push(start.elapsed());
        }

        for _ in 0..1000 {
            let start = Instant::now();
            min_slice_len_which_sums_ge_target_using_prefix_array_for_sum(&arr_prefix_sum, target);
            arr_time_using_prefix_sum_array.push(start.elapsed());
        }

        for _ in 0..1000 {
            let start = Instant::now();
            min_slice_len_which_sums_ge_target_using_variable_sliding_window_and_prefix_array_for_sum(
                &arr_prefix_sum,
                target,
            );
            arr_time_using_prefix_sum_array_and_variable_sliding_window.push(start.elapsed());
        }

        let time_naive = arr_time_naive[median_duration_index_u128(&arr_time_naive)];

        let time_using_prefix_sum_array = Duration::from_nanos(
            (arr_time_using_prefix_sum_array
                .iter()
                .map(|d| d.as_nanos())
                .sum::<u128>()
                / arr_time_using_prefix_sum_array.len() as u128) as u64,
        );

        let time_using_prefix_sum_array_and_variable_sliding_window = Duration::from_nanos(
            (arr_time_using_prefix_sum_array_and_variable_sliding_window
                .iter()
                .map(|d| d.as_nanos())
                .sum::<u128>()
                / arr_time_using_prefix_sum_array_and_variable_sliding_window.len() as u128)
                as u64,
        );
        print_output_row_ratio_compare_result(
            &columns,
            *size,
            vec![
                (time_naive, time_using_prefix_sum_array),
                (
                    time_using_prefix_sum_array,
                    time_using_prefix_sum_array_and_variable_sliding_window,
                ),
            ],
        );
    }

    println!(
        "\nNaive approach too slow at this point, removing it from benchmark, and upping difficulty..."
    );

    println!("--> Prefix Sum vs Variable Sliding Window");
    let columns = [
        "Data Size",
        "Prefix Sum (µs)",
        "Var Sliding Window Prefix Sum (µs)",
        "      Ratio", // Ratio values getting a bit big so hacking a bit of space
    ];
    print_header(&columns);
    let start_size = arr_sizes[arr_sizes.len() - 1];
    let number_of_doubles = 5;
    // starting at
    let arr_sizes: Vec<usize> = (0..number_of_doubles).map(|i| start_size << i).collect();

    for size in &arr_sizes {
        let mut arr_time_using_prefix_sum_array: Vec<Duration> = Vec::with_capacity(10);
        let mut arr_time_using_prefix_sum_array_and_variable_sliding_window: Vec<Duration> =
            Vec::with_capacity(1000);
        let arr = create_array(*size);
        let arr_prefix_sum = make_prefix_sum_array(&arr);
        let target = i32::MAX; // Worst case scenario for naive, but never stresses variable sliding window.

        for _ in 0..10 {
            let start = Instant::now();
            min_slice_len_which_sums_ge_target_using_prefix_array_for_sum(&arr_prefix_sum, target);
            arr_time_using_prefix_sum_array.push(start.elapsed());
        }

        for _ in 0..1000 {
            let start = Instant::now();
            min_slice_len_which_sums_ge_target_using_variable_sliding_window_and_prefix_array_for_sum(
                &arr_prefix_sum,
                target,
            );
            arr_time_using_prefix_sum_array_and_variable_sliding_window.push(start.elapsed());
        }
        let time_using_prefix_sum_array = arr_time_using_prefix_sum_array
            [median_duration_index_u128(&arr_time_using_prefix_sum_array)];

        let time_using_prefix_sum_array_and_variable_sliding_window = Duration::from_nanos(
            (arr_time_using_prefix_sum_array_and_variable_sliding_window
                .iter()
                .map(|d| d.as_nanos())
                .sum::<u128>()
                / arr_time_using_prefix_sum_array_and_variable_sliding_window.len() as u128)
                as u64,
        );

        print_output_row_ratio_compare_result(
            &columns,
            *size,
            vec![(
                time_using_prefix_sum_array,
                time_using_prefix_sum_array_and_variable_sliding_window,
            )],
        );
    }

    println!("\nPrefix Sum on its own is too slow, removing...");
    println!("--> Variable Sliding Window");
    let columns = ["Data Size", "Var Sliding Window Prefix Sum (µs)"];
    print_header(&columns);

    let start_size = arr_sizes[arr_sizes.len() - 1];
    let number_of_doubles = 9;
    // starting at
    let arr_sizes: Vec<usize> = (0..number_of_doubles).map(|i| start_size << i).collect();

    for size in arr_sizes {
        let mut arr_time_using_prefix_sum_array_and_variable_sliding_window: Vec<Duration> =
            Vec::with_capacity(1000);
        let arr = create_array(size);
        let arr_prefix_sum = make_prefix_sum_array(&arr);
        let target = i32::MAX; // Worst case scenario for naive, but never stresses variable sliding window.

        for _ in 0..1000 {
            let start = Instant::now();
            min_slice_len_which_sums_ge_target_using_variable_sliding_window_and_prefix_array_for_sum(
                &arr_prefix_sum,
                target,
            );
            arr_time_using_prefix_sum_array_and_variable_sliding_window.push(start.elapsed());
        }

        let time_using_prefix_sum_array_and_variable_sliding_window = Duration::from_nanos(
            (arr_time_using_prefix_sum_array_and_variable_sliding_window
                .iter()
                .map(|d| d.as_nanos())
                .sum::<u128>()
                / arr_time_using_prefix_sum_array_and_variable_sliding_window.len() as u128)
                as u64,
        );

        println!(
            "  {0:>1$} | {2:3$} ",
            size,
            columns[0].chars().count() - 1,
            time_using_prefix_sum_array_and_variable_sliding_window.as_micros(),
            columns[1].chars().count() - 1
        );
    }

    print!("\n\n");
}

// O(n^3)
fn min_slice_len_which_sums_ge_target_naive(arr: &[i32], target: i32) -> Option<usize> {
    if arr.len() == 0 {
        return None;
    }
    // For each length to be tested starting at the smallest length
    for slice_len in 1..=arr.len() {
        let slice_len_right_offset = slice_len - 1;
        // For each starting position less than the slice length
        for left in 0..arr.len() - slice_len_right_offset {
            let right = left + slice_len_right_offset;
            // Sum left through right
            let mut sum = 0;
            for i in left..=right {
                sum += arr[i];
            }
            // Test
            if sum >= target {
                return Some(slice_len);
            }
        }
    }
    None
}

// Prefix Array version
fn min_slice_len_which_sums_ge_target_using_prefix_array_for_sum(
    arr_prefix_array: &[i32],
    target: i32,
) -> Option<usize> {
    if arr_prefix_array.len() == 0 {
        return None;
    }
    // For each length to be tested starting at the smallest length
    for slice_len in 1..=arr_prefix_array.len() {
        let slice_len_right_offset = slice_len - 1;
        // For each starting position less than the slice length
        for left in 0..arr_prefix_array.len() - slice_len_right_offset {
            let right = left + slice_len_right_offset;
            // Sum left through right
            let sum = range_sum_prefix_sum_arr(&arr_prefix_array, left, right);
            // Test
            if sum >= target {
                return Some(slice_len);
            }
        }
    }
    None
}

///--- O(n) time complexity implementation.
// This enum corresponds to the 3 things that
// min_slice_len_which_sums_ge_target_using_variable_sliding_window_and_prefix_array_for_sum
// does while it is processing
enum SearchMode {
    ExpandRightToQualify, // Step 1: expand right pointer until qualifying window found
    ContractLeftToMinimize, // Step 2: contract left pointer to minimize window
    SlideBothPointers,    // Step 3: slide both left and right forward until next qualifying window
}
// The key insight here is to realize you are looking for a qualifying window,
// while at the same time eliminating as many elements i.e. previously checked
// window space as posible.
//
// 1) find the furthest right that makes arr[left..=right] qualify.
// 2) Then contract left until arr[left..=right] no longer qualifies
//      and if smaller record right - length +1 as your current min_window_size
//      and subtract 1 from left.
//
// 3) repeat: add 1 to right and left until arr[left..=right] qualifies
//    again or r would go out of bounds.
// 3.1) If not going out of bounds repeat from 2)
// 3.2) If r was going to go out of bounds return min_window_size.
//
// Aparently you can exceed this implementation's performance more by
// not taking a prefix array itself as the argument but instead implementing a
// prefix sum inside it while sliding, aparently it's more cache friendly to the
// processor, however in this case we just wanted to go from O(n^3) to O(n)
// which we did achieve.
fn min_slice_len_which_sums_ge_target_using_variable_sliding_window_and_prefix_array_for_sum(
    arr_prefix_array: &[i32],
    target: i32,
) -> Option<usize> {
    if arr_prefix_array.len() == 0 {
        return None;
    }
    let mut l = 0;
    let mut r = 0;

    // Are we in 2) or are we in 1/3)?
    let mut search_mode = SearchMode::ExpandRightToQualify;

    let mut result = None;
    loop {
        match search_mode {
            SearchMode::ExpandRightToQualify => {
                // Mode 1)
                // Find the qualifying right
                while r < arr_prefix_array.len()
                    && range_sum_prefix_sum_arr(arr_prefix_array, l, r) < target
                {
                    r += 1; // Expand right side
                }
                if r == arr_prefix_array.len() {
                    // we reached the end without finding any qualifying slices
                    // so we break the loop with result = None
                    break;
                }
                search_mode = SearchMode::ContractLeftToMinimize;
            }
            SearchMode::ContractLeftToMinimize => {
                // Mode 2)
                // Find the contracted left.
                while l <= r && range_sum_prefix_sum_arr(arr_prefix_array, l, r) >= target {
                    l += 1; // Contract left side
                }

                // Record our minimum window size that qualified.
                // extra -1 on l is because the while moved it beyond where it qualfied.
                result = Some(1 + r - (l - 1));
                // if r was at the end of the array we are effectively done
                // and can return the result.
                if r == (arr_prefix_array.len() - 1) {
                    // we have reached the overall end of our variable window
                    // scroll run and the best answer is now present in result.
                    break;
                }
                // l moves past r when the sliding window lenght is 1, to counter
                // act this in this scenario when l moves r moves as well.
                if l > r {
                    r = l;
                }
                search_mode = SearchMode::SlideBothPointers;
            }
            SearchMode::SlideBothPointers => {
                // 3) find a window to the right that can potentially match a
                //  smaller result
                while r < arr_prefix_array.len()
                    && range_sum_prefix_sum_arr(arr_prefix_array, l, r) < target
                {
                    // Slide the whole fixed window now.
                    r += 1;
                    l += 1;
                }
                if r == arr_prefix_array.len() {
                    // We couldn't find a window on the right so we are done
                    break;
                } else {
                    // We found a window so we can try contracting again.
                    search_mode = SearchMode::ContractLeftToMinimize;
                }
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_array_using_prefix_array_for_sum_with_variable_sliding_window() {
        assert_eq!(
            min_slice_len_which_sums_ge_target_using_variable_sliding_window_and_prefix_array_for_sum(
                &make_prefix_sum_array(&[]),
                5
            ),
            None
        );
    }

    #[test]
    fn single_element_less_than_target_using_prefix_array_for_sum_with_variable_sliding_window() {
        assert_eq!(
            min_slice_len_which_sums_ge_target_using_variable_sliding_window_and_prefix_array_for_sum(
                &make_prefix_sum_array(&[3]),
                5
            ),
            None
        );
    }

    #[test]
    fn single_element_equal_to_target_using_prefix_array_for_sum_with_variable_sliding_window() {
        assert_eq!(
            min_slice_len_which_sums_ge_target_using_variable_sliding_window_and_prefix_array_for_sum(
                &make_prefix_sum_array(&[5]),
                5
            ),
            Some(1)
        );
    }

    #[test]
    fn single_element_greater_than_target_using_prefix_array_for_sum_with_variable_sliding_window()
    {
        assert_eq!(
            min_slice_len_which_sums_ge_target_using_variable_sliding_window_and_prefix_array_for_sum(
                &make_prefix_sum_array(&[10]),
                5
            ),
            Some(1)
        );
    }

    #[test]
    fn multiple_elements_no_subarray_meets_target_using_prefix_array_for_sum_with_variable_sliding_window()
     {
        assert_eq!(
            min_slice_len_which_sums_ge_target_using_variable_sliding_window_and_prefix_array_for_sum(
                &make_prefix_sum_array(&[1, 1, 1, 1]),
                10
            ),
            None
        );
    }

    #[test]
    fn multiple_elements_with_valid_subarray_using_prefix_array_for_sum_with_variable_sliding_window()
     {
        assert_eq!(
            min_slice_len_which_sums_ge_target_using_variable_sliding_window_and_prefix_array_for_sum(
                &make_prefix_sum_array(&[2, 3, 1, 2, 4, 3]),
                7
            ),
            Some(2)
        );
    }

    #[test]
    fn target_zero_using_prefix_array_for_sum_with_variable_sliding_window() {
        assert_eq!(
            min_slice_len_which_sums_ge_target_using_variable_sliding_window_and_prefix_array_for_sum(
                &make_prefix_sum_array(&[1, 2, 3]),
                0
            ),
            Some(1)
        );
    }

    ////
    #[test]
    fn empty_array_using_prefix_array_for_sum() {
        assert_eq!(
            min_slice_len_which_sums_ge_target_using_prefix_array_for_sum(
                &make_prefix_sum_array(&[]),
                5
            ),
            None
        );
    }

    #[test]
    fn single_element_less_than_target_using_prefix_array_for_sum() {
        assert_eq!(
            min_slice_len_which_sums_ge_target_using_prefix_array_for_sum(
                &make_prefix_sum_array(&[3]),
                5
            ),
            None
        );
    }

    #[test]
    fn single_element_equal_to_target_using_prefix_array_for_sum() {
        assert_eq!(
            min_slice_len_which_sums_ge_target_using_prefix_array_for_sum(
                &make_prefix_sum_array(&[5]),
                5
            ),
            Some(1)
        );
    }

    #[test]
    fn single_element_greater_than_target_using_prefix_array_for_sum() {
        assert_eq!(
            min_slice_len_which_sums_ge_target_using_prefix_array_for_sum(
                &make_prefix_sum_array(&[10]),
                5
            ),
            Some(1)
        );
    }

    #[test]
    fn multiple_elements_no_subarray_meets_target_using_prefix_array_for_sum() {
        assert_eq!(
            min_slice_len_which_sums_ge_target_using_prefix_array_for_sum(
                &make_prefix_sum_array(&[1, 1, 1, 1]),
                10
            ),
            None
        );
    }

    #[test]
    fn multiple_elements_with_valid_subarray_using_prefix_array_for_sum() {
        assert_eq!(
            min_slice_len_which_sums_ge_target_using_prefix_array_for_sum(
                &make_prefix_sum_array(&[2, 3, 1, 2, 4, 3]),
                7
            ),
            Some(2)
        );
    }

    #[test]
    fn target_zero_using_prefix_array_for_sum() {
        assert_eq!(
            min_slice_len_which_sums_ge_target_using_prefix_array_for_sum(
                &make_prefix_sum_array(&make_prefix_sum_array(&[1, 2, 3])),
                0
            ),
            Some(1)
        );
    }

    ////
    #[test]
    fn empty_array_naive() {
        assert_eq!(min_slice_len_which_sums_ge_target_naive(&[], 5), None);
    }

    #[test]
    fn single_element_less_than_target_naive() {
        assert_eq!(min_slice_len_which_sums_ge_target_naive(&[3], 5), None);
    }

    #[test]
    fn single_element_equal_to_target_naive() {
        assert_eq!(min_slice_len_which_sums_ge_target_naive(&[5], 5), Some(1));
    }

    #[test]
    fn single_element_greater_than_target_naive() {
        assert_eq!(min_slice_len_which_sums_ge_target_naive(&[10], 5), Some(1));
    }

    #[test]
    fn multiple_elements_no_subarray_meets_target_naive() {
        assert_eq!(
            min_slice_len_which_sums_ge_target_naive(&[1, 1, 1, 1], 10),
            None
        );
    }

    #[test]
    fn multiple_elements_with_valid_subarray_naive() {
        assert_eq!(
            min_slice_len_which_sums_ge_target_naive(&[2, 3, 1, 2, 4, 3], 7),
            Some(2)
        );
    }

    #[test]
    fn target_zero_naive() {
        assert_eq!(
            min_slice_len_which_sums_ge_target_naive(&[1, 2, 3], 0),
            Some(1)
        );
    }
}
