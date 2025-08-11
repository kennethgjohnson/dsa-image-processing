// These exersices make extensive use of prefix_sum_arrays.rs

use super::prefix_sum_arrays::{make_prefix_sum_array, range_sum_naive, range_sum_prefix_sum_arr};
use crate::tutorial::common_util::{
    create_array, median_duration_index_u128, print_header, print_output_row_ratio_compare_result,
};
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

pub fn challenge_count_sub_arrays_sum_k() {
    println!("==> Mini challenge: count the number of sub arrays in an array that total to k");
    let columns = [
        "Data Size",
        "Naive Time (µs)",
        "Prefix Sum ArrayTime (µs)",
        "      Ratio", // Ratio values getting a bit big so hacking a bit of space
        "Prefix Sum ArrayTime (µs)",
        "Hashmap Prefix Array Time (µs)",
        "Ratio", // Ratio values getting a bit big so hacking a bit of space
    ];
    print_header(&columns);
    let size_interval = 1;
    let size_count = 12;
    let lower_threshold = 16;
    // starting at
    let arr_sizes: Vec<usize> = (0..size_count)
        .map(|i| size_interval << i)
        .filter(|element| *element >= lower_threshold)
        .collect();
    for size in arr_sizes {
        let mut arr_time_naive: Vec<Duration> = Vec::with_capacity(5);
        let mut arr_time_prefix_sum_array: Vec<Duration> = Vec::with_capacity(1000);
        let mut arr_time_prefix_sum_array_with_hashmap: Vec<Duration> = Vec::with_capacity(1000);

        let arr = create_array(size);

        for _ in 0..5 {
            let start = Instant::now();
            count_sub_arrays_sum_k_using_naive_approach(&arr, 123_456_789);
            arr_time_naive.push(start.elapsed());
        }

        for _ in 0..1000 {
            let start = Instant::now();
            count_sub_arrays_sum_k_using_prefix_sum_array(&arr, 123_456_789);
            arr_time_prefix_sum_array.push(start.elapsed());
        }

        for _ in 0..1000 {
            let start = Instant::now();
            count_sub_arrays_sum_k_using_prefix_and_hashmap(&arr, 123_456_789);
            arr_time_prefix_sum_array_with_hashmap.push(start.elapsed());
        }

        let time_naive = arr_time_naive[median_duration_index_u128(&arr_time_naive)];
        let time_prefix_sum_array = Duration::from_nanos(
            (arr_time_prefix_sum_array
                .iter()
                .map(|d| d.as_nanos())
                .sum::<u128>()
                / arr_time_prefix_sum_array.len() as u128) as u64,
        );
        let time_prefix_sum_array_with_hashmap = Duration::from_nanos(
            (arr_time_prefix_sum_array_with_hashmap
                .iter()
                .map(|d| d.as_nanos())
                .sum::<u128>()
                / arr_time_prefix_sum_array_with_hashmap.len() as u128) as u64,
        );
        print_output_row_ratio_compare_result(
            &columns,
            size,
            vec![
                (time_naive, time_prefix_sum_array),
                (time_prefix_sum_array, time_prefix_sum_array_with_hashmap),
            ],
        );
    }
    print!("\n\n");
}

// Naive implementation of checking how many sub arrays there are in
// arr that sum to the total of k - I.e. linear combinations of elements
// that total to k
// This is the naive implementation so its going to be a brute force
// every time you check. O(n^3) implementation.
fn count_sub_arrays_sum_k_using_naive_approach(arr: &[i32], k: i32) -> i32 {
    let mut count = 0;
    for l in 0..arr.len() {
        for r in l..arr.len() {
            let naive_sum_value = range_sum_naive(arr, l, r);
            if naive_sum_value == k {
                count += 1;
            }
        }
    }
    count
}

// Here we do the same as the naive implementation except we create a
// prefix_sum_arry and reuse it so we have O(1) checks on the sum_value
// Theoretically it brings O(n^3) down to O(n^2)
fn count_sub_arrays_sum_k_using_prefix_sum_array(arr: &[i32], k: i32) -> i32 {
    let prefix_sum_array = make_prefix_sum_array(&arr);
    let mut count = 0;
    for l in 0..arr.len() {
        for r in l..arr.len() {
            let prefix_array_sum_value = range_sum_prefix_sum_arr(&prefix_sum_array, l, r);
            if prefix_array_sum_value == k {
                count += 1;
            }
        }
    }
    count
}

// THIS IS THE IMPLEMENTATION OF count_subarrays_sum_k TO KNOW:
// Here we do a hashmap implementation directly with the logic we used to build
// a prefixthe same as the naive implementation except we create a prefix array
// bringing the overall time down to O(n)
//
// This part was stupid hard to understand at least for me:
// arr_value is technically arr[j] in this discussion
// 1) prefix_sum[j] is the sum of arr[0]..arr[j]
// 2) prefix_sum[i] is some sub array of arr[0]..arr[j] that precedes the position j
// 3) prefix_sum[j] - prefix_sum[i-1] = the sum of arr[i..=j]
// Note: we saw in range_sum_prefix_sum_arr how this subtraction of the part
// preceeding the start of prefix_sum[i] from prefix_sum[j] would convert
// prefix_sum[j] into sum of arr[i..=j] aka
// "To get the sum from i to j, take the sum up to j, and subtract what came before i."
// 4) We are trying to count the cases where sum(arr[i..j])==k
// This means we are effectively trying to count the number of cases as we go where
// prefix_sum[j] - prefix_sum[i-1] == k
// **So to put it another way, is if you have prefix_sum[j], then if such a set
// existed before it would have been seen at prefix_sum[i-1] == prefix_sum[j] - k**
// 5) This is the key insight, that we can know if such a set existed at a previous time
// if we added prefix_sum[j] continually to a hash map while it is running, then the
// entries in the hashmap effectively become prefix_sum[i] or prefix_sum[i-1]
// 6) So effectively if the encountering of these totals in the hash map are incremented
// the hash map can be inspected for (prefix_sum[j] - k) from step 4) for the existance
// of such a preceding set, and it can be added to the total of matches that add up to k
// that has been seen.
fn count_sub_arrays_sum_k_using_prefix_and_hashmap(arr: &[i32], k: i32) -> i32 {
    if arr.len() == 0 {
        return 0;
    }
    let mut count = 0;
    let mut prefix_sum = 0;
    let mut hash_map: HashMap<i32, i32> = HashMap::new();
    // This insertion of 0 with value 1 confused me for a long time.
    // Similar to the prefix_sum_arrays condition when l=0 pivots straight to
    // pre_sub_arr[r] to return the arr[0..r] total this basically needs the 0
    // in place for scenarios where the arr[0..j] = k and there is no arr[i-1]
    // because its from the start of the array, under those conditions the
    // prefix_sum_i_minus_1 doesn't really exist and therefore evaluates to 0.
    // This hardcoding ensures that when we go looking in the hashmap that we find
    // it with 1 so that 1 can be added to the count.
    hash_map.insert(0, 1);
    for element_value in arr {
        prefix_sum += element_value; // prefix_sum[j] is captured.
        let prefix_sum_i_minus_1 = prefix_sum - k;
        if let Some(prefix_sum_i_minus_1_seen_count) = hash_map.get(&prefix_sum_i_minus_1) {
            // We have seen it before
            count += prefix_sum_i_minus_1_seen_count;
        }
        *hash_map.entry(prefix_sum).or_insert(0) += 1; // increment prefix_sum_i_minus_1 seen count.
    }
    count
}

#[test]
fn test_count_subarrays_sum_k_with_naive_approach() {
    let cases = vec![
        (vec![], 0, 0),                        // Empty array
        (vec![1], 1, 1),                       // Single match
        (vec![1], 2, 0),                       // No match
        (vec![1, 1, 1], 2, 2),                 // [1,1] x2
        (vec![1, 2, 3], 3, 2),                 // [1,2], [3]
        (vec![3, 4, 7, 2, -3, 1, 4, 2], 7, 4), // Multiple subarrays
        (vec![1, -1, 0], 0, 3),                // [1,-1], [0], [1,-1,0]
    ];

    for (arr, k, expected) in cases {
        let result = count_sub_arrays_sum_k_using_naive_approach(&arr, k);
        assert_eq!(result, expected, "Failed on arr: {:?}, k: {}", arr, k);
    }
}

#[test]
fn test_count_subarrays_sum_k_with_prefix_sum_array() {
    let cases = vec![
        (vec![], 0, 0),                        // Empty array
        (vec![1], 1, 1),                       // Single match
        (vec![1], 2, 0),                       // No match
        (vec![1, 1, 1], 2, 2),                 // [1,1] x2
        (vec![1, 2, 3], 3, 2),                 // [1,2], [3]
        (vec![3, 4, 7, 2, -3, 1, 4, 2], 7, 4), // Multiple subarrays
        (vec![1, -1, 0], 0, 3),                // [1,-1], [0], [1,-1,0]
    ];

    for (arr, k, expected) in cases {
        let result = count_sub_arrays_sum_k_using_prefix_sum_array(&arr, k);
        assert_eq!(result, expected, "Failed on arr: {:?}, k: {}", arr, k);
    }
}

#[test]
fn test_count_subarrays_sum_k_with_prefix_and_hashmap() {
    let cases = vec![
        (vec![], 0, 0),                        // Empty array
        (vec![1], 1, 1),                       // Single match
        (vec![1], 2, 0),                       // No match
        (vec![1, 1, 1], 2, 2),                 // [1,1] x2
        (vec![1, 2, 3], 3, 2),                 // [1,2], [3]
        (vec![3, 4, 7, 2, -3, 1, 4, 2], 7, 4), // Multiple subarrays
        (vec![1, -1, 0], 0, 3),                // [1,-1], [0], [1,-1,0]
    ];

    for (arr, k, expected) in cases {
        let result = count_sub_arrays_sum_k_using_prefix_and_hashmap(&arr, k);
        assert_eq!(result, expected, "Failed on arr: {:?}, k: {}", arr, k);
    }
}
