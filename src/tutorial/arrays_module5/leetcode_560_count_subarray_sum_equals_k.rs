use crate::tutorial::common_util::{create_array, print_header};
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

// Bonus 2: Leetcode 560: Subarray Sum Equals K
// https://leetcode.com/problems/subarray-sum-equals-k/description/
pub fn bonus_leetcode_560_subarray_sum_equals_k() {
    println!(
        "==> Bonus Leetcode 560 Count Sub Arrays that's sum equals k - use prefix sums algebra and a hashmap O(n)"
    );
    let columns = ["Data Size", "Time (µs)"];
    print_header(&columns);
    let start_size = 1024;
    let number_of_doubles = 6;
    // starting at
    let arr_sizes: Vec<usize> = (0..number_of_doubles).map(|i| start_size << i).collect();

    for size in &arr_sizes {
        let mut arr_time: Vec<Duration> = Vec::with_capacity(1000);
        let arr = create_array(*size);

        for _ in 0..1000 {
            let start = Instant::now();
            subarray_sum(&arr, 123_456_789);
            arr_time.push(start.elapsed());
        }

        let time = Duration::from_nanos(
            (arr_time.iter().map(|d| d.as_nanos()).sum::<u128>() / arr_time.len() as u128) as u64,
        );

        println!(
            "  {0:>1$} | {2:3$} ",
            size,
            columns[0].chars().count() - 1,
            time.as_micros(),
            columns[1].chars().count() - 1
        );
    }

    print!("\n\n");
}

//----
// My Notes
// Naive:
// Visiting each sub array which equals to k would be O(n^2) complexity since you
// are looping once for each size, and once for each starting position of that size
//
// The calculating of their size can be made O(1) using a prefix array.
//
// So the question here is how do we change visiting of each sub array into
// visiting O(n) or less..., the answer is a "sliding window" of 1 which
// calculates the prefix sum since prefix sums allow us to make O(1) assessments
// of previously encountered data's relation to current data in a progression.
//
// So we wish to know if a sub array exists such that sum of (arr[i]..=arr[n]) = k
// Prefix arrays encode the sum of arr[i]..=arr[n] as prefix[n] - prefix[i-1]
// i.e. you chop off the part of the total that is irrelevant.
// so we are effectively looking for k = prefix[n] - prefix[i-1]
//
// So when processing calculating prefix[n] to determine if such a sub array exist
// that it would sum to k means did we see prefix[i-1] before,
// prefix[i-1] can be found with some algebra prefix[i-1] = prefix[n] - k.
//
// So does this prefix array element exist? this would be a O(n) enquiry on your
// existing data if you were to loop over it so we need to bring in a hashmap
// map(prefix[i-1]) = "exists" -> however we want to know more than if it existed
// before, we wish to know how many times has it existed since all instances would
// be combinations that can potentially sum to prefix[n] if k is added.
// Therefore map(prefix[i-1]) does not equal "exists", but instead
// map(prefix[i-1]) = how many times has the value been seen.
//
// Therefore while we process the array for prefix sums, we build a map of how many
// times we have seen a prefix array position sum to the resulting value.

// In addition to this an edge case exists, this is where i-1 is prior to the start
// of the array, this comes into play for prefix sums of 0..n, to handle this we
// need to include a edge case record of prefix sum value 0 being seen 1 times
// representing the prefix[i-1] that falls out of bounds prior to the start of the
// array.

// And if I didn't understand my previous paragraph there is this AI generated "refinement"
// ****
// Edge case (subarrays that start at index 0):
// The algebra uses prefix[i-1], and i-1 can be -1 (the empty prefix before the array).
// By definition prefix[-1] = 0, and there is exactly one such index before scanning.
// So we initialize the hashmap with map[0] = 1 to count that single prior index.
// This makes the lookup `map[prefix[j] - k]` automatically count subarrays starting at 0.
// ****

// And AI suposid "cleanup" version of my notes, but I don't know havent' read
// over this just leaving it here for now:
//  {{{{
// (polished)
// Naive:
// Checking every subarray to see if it sums to k is O(n^2):
//  - Outer loop for start position
//  - Inner loop for end position
//
// Using a prefix-sum array lets you compute any subarray sum in O(1).
//
// Goal: reduce visiting every subarray to O(n).
// Idea: scan once and keep counts of previously seen prefix sums.
// Let prefix[t] = sum(arr[0..t]) and define prefix[-1] = 0 (empty prefix).
//
// sum(arr[i..j]) = prefix[j] - prefix[i-1].
// We want sum(...) = k, so:
//   prefix[i-1] = prefix[j] - k
//
// While scanning index j (computing prefix[j]):
//  - The number of subarrays ending at j with sum k equals
//    map[prefix[j] - k], where map stores counts of prior prefix values.
//  - Then increment map[prefix[j]].
//
// Hashmap definition:
//  - key   = prefix sum value
//  - value = how many times that prefix sum has been seen so far
//
// Edge case (subarrays that start at index 0):
//  - prefix[-1] = 0 is a valid prior prefix index (the empty prefix).
//  - There is exactly one such index before scanning, so initialize map[0] = 1.
//    That lets the formula map[prefix[j] - k] correctly count subarrays that
//    begin at index 0 without special-casing.
//
// Invariant (useful to reason about correctness):
//  - Before processing any elements, the hashmap counts prefix[-1]=0 once:
//    map[0] = 1.
//
// Complexity:
//  - Time: O(n)
//  - Space: O(n) (for the hashmap)
//  }}}}

/// This is identical to the challenge_count_sub_arrays_sum_k problem I just added
/// better notes this time round.
fn subarray_sum(nums: &Vec<i32>, k: i32) -> i32 {
    if nums.len() == 0 {
        return 0;
    }
    let mut count = 0;
    let mut prefix_sum = 0;
    let mut hash_map: HashMap<i32, i32> = HashMap::new();

    hash_map.insert(0, 1);
    for element_value in nums {
        prefix_sum += element_value;
        let prefix_sum_i_minus_1 = prefix_sum - k;
        if let Some(prefix_sum_i_minus_1_seen_count) = hash_map.get(&prefix_sum_i_minus_1) {
            // We have seen it before
            count += prefix_sum_i_minus_1_seen_count;
        }
        *hash_map.entry(prefix_sum).or_insert(0) += 1; // Increment new seen value
    }
    count
}
