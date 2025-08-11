mod challenge_count_sub_arrays_sum_k;
mod challenge_max_sub_element_slice_len_with_at_most_k_distinct_elements;
mod leetcode_209_min_size_subarray_with_sum_ge_target;
mod max_sum_for_k_slice;
mod min_slice_len_which_sums_ge_target;
mod prefix_sum_arrays;
mod suffix_sum_arrays;

use core::num;
use std::env::join_paths;

use challenge_count_sub_arrays_sum_k::challenge_count_sub_arrays_sum_k;
use challenge_max_sub_element_slice_len_with_at_most_k_distinct_elements::challenge_max_sub_element_slice_len_with_at_most_k_distinct_elements;
use leetcode_209_min_size_subarray_with_sum_ge_target::bonus_leetcode_209_min_size_subarray_with_sum_ge_target;
use max_sum_for_k_slice::max_sum_for_k_slice;
use min_slice_len_which_sums_ge_target::min_slice_len_which_sums_ge_target;
use prefix_sum_arrays::prefix_sum_arrays;
use suffix_sum_arrays::suffix_sum_arrays;

pub fn arrays_module5_sliding_windows_and_prefix_sum_tricks() {
    // Topic: fundamental patterns in optimization
    // We will be going over 2 fundamental paterns that appear in optimization
    // tasks:
    // 1) Prefix Sum Arrays - used for instant sub array sum calculations
    // 2) Sliding Window - contiguous subarrays wanting to optimize something
    //      over a range - Max Sum of Fixed Window (Fixed-sized sliding windows)
    //      and Smallest Subarray ≥ Target (variable-sized sliding windows)

    // 1) Prefix Sum Arrays
    // Naive O(n)
    // Turning O(n) into O(1)
    prefix_sum_arrays();
    suffix_sum_arrays();

    // Extra Challenge - count_sub_arrays_sum_k - is particularly hard and interesting at least for me...
    // I.e. how to use algebra with a hashmap and prefix arrays to encode a
    //  relation between previous elements
    //  Naive O(n^3)
    //  Prefix Sum O(n^2)
    //  Prefix Sum with Hashmap encoding algebraic relation O(n)
    challenge_count_sub_arrays_sum_k();

    // 2) Sliding Windows
    // Fixed - Window
    // Naive O(n^2)
    // Prefix Sum O(n)
    max_sum_for_k_slice();

    // Variable - Window
    // Naive O(n^3)
    // Prefix Sum O(n^2)
    // Prefix Sum with variable window O(n)
    min_slice_len_which_sums_ge_target();

    // Mini-Challenge: Longest Substring with At Most K Distinct Elements
    challenge_max_sub_element_slice_len_with_at_most_k_distinct_elements();

    // Bonus 1: Leetcode 209: Minimum Size Subarray Sum
    // https://leetcode.com/problems/minimum-size-subarray-sum/description/
    // Same as min_slice_len_which_sums_ge_target.rs, I will use this opertunity
    // to build the prefix array during processing.
    bonus_leetcode_209_min_size_subarray_with_sum_ge_target()

    // Bonus 2: Leetcode 560: Subarray Sum Equals K
    // https://leetcode.com/problems/subarray-sum-equals-k/description/

    // Bonus 3: Leetcode 76: Minimum Window Substring
    //https://leetcode.com/problems/minimum-window-substring/description/
}

enum SearchMode {
    ExpandRight,
    ContractLeft,
    SlideRight,
}

pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
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

fn range_sum_prefix_sum_arr(prefix_arr: &[i32], l: usize, r: usize) -> i32 {
    if l == 0 {
        prefix_arr[r]
    } else {
        prefix_arr[r] - prefix_arr[l - 1]
    }
}
