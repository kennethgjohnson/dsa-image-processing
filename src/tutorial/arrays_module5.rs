mod count_subarrays_sum_k;
mod max_sum_fixed_sliding_window_over_array;
mod prefix_sum_arrays;
mod suffix_sum_arrays;

use count_subarrays_sum_k::arrays_module5_fundamental_patterns_in_optimization_count_sub_arrays_sum_k;
use max_sum_fixed_sliding_window_over_array::arrays_module5_fundamental_patterns_in_optimization_max_sum_fixed_window;
use prefix_sum_arrays::arrays_module5_fundamental_patterns_in_optimization_prefix_sum_arrays;
use suffix_sum_arrays::arrays_module5_fundamental_patterns_in_optimization_suffix_sum_arrays;

pub fn arrays_module5_sliding_windows_and_prefix_sum_tricks() {
    // We will be going over 2 fundamental paterns that appear in optimization
    // tasks:
    // 1) Prefix Sum Arrays - used for instant sub array sum calculations
    // 2) Sliding Window - contiguous subarrays wanting to optimize something
    //      over a range - Max Sum of Fixed Window (Fixed-sized sliding windows)
    //      and Smallest Subarray ≥ Target (variable-sized sliding windows)

    // 1) Prefix Sum Arrays
    arrays_module5_fundamental_patterns_in_optimization_prefix_sum_arrays();
    arrays_module5_fundamental_patterns_in_optimization_suffix_sum_arrays();

    // Extra Challenge - count_sub_arrays_sum_k - is particularly hard and interesting at least for me...
    arrays_module5_fundamental_patterns_in_optimization_count_sub_arrays_sum_k();

    // 2) Siding Window Max Sum of Fixed Window
    arrays_module5_fundamental_patterns_in_optimization_max_sum_fixed_window();

    // 2) TODO Siding Window: Smallest Subarray ≥ Target (variable-sized sliding window)
}
