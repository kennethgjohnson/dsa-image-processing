mod challenge_count_sub_arrays_sum_k;
mod challenge_max_sub_element_slice_len_with_at_most_k_distinct_elements;
mod max_sum_for_k_slice;
mod min_slice_len_which_sums_ge_target;
mod prefix_sum_arrays;
mod suffix_sum_arrays;

use challenge_count_sub_arrays_sum_k::challenge_count_sub_arrays_sum_k;
use challenge_max_sub_element_slice_len_with_at_most_k_distinct_elements::challenge_max_sub_element_slice_len_with_at_most_k_distinct_elements;
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
}
