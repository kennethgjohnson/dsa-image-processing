use std::collections::HashMap;
use std::hash::Hash;
use std::time::{Duration, Instant};

use crate::tutorial::common_util::{
    create_array, median_duration_index_u128, print_header, print_output_row_ratio_compare_result,
};

// Given a string and integer k, return the length of the longest substring with at most k distinct characters.
pub fn challenge_max_sub_element_slice_len_with_at_most_k_distinct_elements() {
    println!("==> Mini challenge: - Longest Substring with At Most K Distinct Elements");
    println!("--> Very Naive vs Naive with hashmap vs Var Sliding Window with hashmap");
    let columns = [
        "Data Size",
        "Very Naive (µs)",
        "Naive with hashmap (µs)",
        "      Ratio", // Ratio values getting a bit big so hacking a bit of space
        "Naive with hashmap (µs)",
        "Var Sliding Window with Hashmap (µs)",
        "      Ratio", // Ratio values getting a bit big so hacking a bit of space
    ];
    print_header(&columns);
    let start_size = 8;
    let number_of_doubles = 7;
    // starting at
    let arr_sizes: Vec<usize> = (0..number_of_doubles).map(|i| start_size << i).collect();

    for size in &arr_sizes {
        let mut arr_time_naive: Vec<Duration> = Vec::with_capacity(10);
        let mut arr_time_naive_hashmap: Vec<Duration> = Vec::with_capacity(100);
        let mut arr_time_sliding_window_hashmap: Vec<Duration> = Vec::with_capacity(1000);
        let arr = create_array(*size);
        // sliding window hashmap's worst case is k=1 while very naive is k=n so we pick the middle.
        let k = size / 2;
        for _ in 0..10 {
            let start = Instant::now();
            max_sub_element_slice_len_with_at_most_k_distinct_elements_naive_very(&arr, k);
            arr_time_naive.push(start.elapsed());
        }

        for _ in 0..100 {
            let start = Instant::now();
            max_sub_element_slice_len_with_at_most_k_distinct_elements_naive_with_hashmap(&arr, k);
            arr_time_naive_hashmap.push(start.elapsed());
        }

        for _ in 0..1000 {
            let start = Instant::now();
            max_sub_element_slice_len_with_at_most_k_distinct_elements_using_sliding_window_and_hashmap(
                &arr, k,
            );
            arr_time_sliding_window_hashmap.push(start.elapsed());
        }

        let time_naive = arr_time_naive[median_duration_index_u128(&arr_time_naive)];

        let time_naive_hashmap = Duration::from_nanos(
            (arr_time_naive_hashmap
                .iter()
                .map(|d| d.as_nanos())
                .sum::<u128>()
                / arr_time_naive_hashmap.len() as u128) as u64,
        );

        let time_sliding_window_hashmap = Duration::from_nanos(
            (arr_time_sliding_window_hashmap
                .iter()
                .map(|d| d.as_nanos())
                .sum::<u128>()
                / arr_time_sliding_window_hashmap.len() as u128) as u64,
        );
        print_output_row_ratio_compare_result(
            &columns,
            *size,
            vec![
                (time_naive, time_naive_hashmap),
                (time_naive_hashmap, time_sliding_window_hashmap),
            ],
        );
    }

    println!("\nVery Naive approach too slow at this point removing it from benchmark...");

    println!("--> Naive with hashmap vs Var Sliding Window with hashmap");
    let columns = [
        "Data Size",
        "Naive with hashmap (µs)",
        "Var Sliding Window with Hashmap (µs)",
        "      Ratio", // Ratio values getting a bit big so hacking a bit of space
    ];
    print_header(&columns);
    let start_size = arr_sizes[arr_sizes.len() - 1];
    let number_of_doubles = 5;
    // starting at
    let arr_sizes: Vec<usize> = (0..number_of_doubles).map(|i| start_size << i).collect();

    for size in &arr_sizes {
        let mut arr_time_naive_hashmap: Vec<Duration> = Vec::with_capacity(10);
        let mut arr_time_sliding_window_hashmap: Vec<Duration> = Vec::with_capacity(1000);
        let arr = create_array(*size);
        // sliding window hashmap's worst case is k=1 while naive is k=n so we pick the middle.
        let k = size / 2;

        for _ in 0..10 {
            let start = Instant::now();
            max_sub_element_slice_len_with_at_most_k_distinct_elements_naive_with_hashmap(&arr, k);
            arr_time_naive_hashmap.push(start.elapsed());
        }

        for _ in 0..1000 {
            let start = Instant::now();
            max_sub_element_slice_len_with_at_most_k_distinct_elements_using_sliding_window_and_hashmap(
                &arr, k,
            );
            arr_time_sliding_window_hashmap.push(start.elapsed());
        }
        let time_naive_hashmap =
            arr_time_naive_hashmap[median_duration_index_u128(&arr_time_naive_hashmap)];

        let time_sliding_window_hashmap = Duration::from_nanos(
            (arr_time_sliding_window_hashmap
                .iter()
                .map(|d| d.as_nanos())
                .sum::<u128>()
                / arr_time_sliding_window_hashmap.len() as u128) as u64,
        );

        print_output_row_ratio_compare_result(
            &columns,
            *size,
            vec![(time_naive_hashmap, time_sliding_window_hashmap)],
        );
    }

    println!(
        "\nNaive is too slow so removing it also, and increasing dificulty for Var implementation to worst case scenario..."
    );
    println!("--> Variable Sliding Window");
    let columns = ["Data Size", "Var Sliding Window with Hashmap (µs)"];
    print_header(&columns);

    let start_size = arr_sizes[arr_sizes.len() - 1];
    let number_of_doubles = 7;
    // starting at
    let arr_sizes: Vec<usize> = (0..number_of_doubles).map(|i| start_size << i).collect();

    for size in arr_sizes {
        let mut arr_time_sliding_window_hashmap: Vec<Duration> = Vec::with_capacity(100);
        let arr = create_array(size);
        let k = 1; // Worst case scenario for variable sliding window.

        for _ in 0..100 {
            let start = Instant::now();
            max_sub_element_slice_len_with_at_most_k_distinct_elements_using_sliding_window_and_hashmap(
                &arr, k,
            );
            arr_time_sliding_window_hashmap.push(start.elapsed());
        }

        let time_sliding_window_hashmap = Duration::from_nanos(
            (arr_time_sliding_window_hashmap
                .iter()
                .map(|d| d.as_nanos())
                .sum::<u128>()
                / arr_time_sliding_window_hashmap.len() as u128) as u64,
        );

        println!(
            "  {0:>1$} | {2:3$} ",
            size,
            columns[0].chars().count() - 1,
            time_sliding_window_hashmap.as_micros(),
            columns[1].chars().count() - 1
        );
    }

    print!("\n\n");
}

fn max_sub_element_slice_len_with_at_most_k_distinct_elements_naive_very<T>(
    arr: &[T],
    k: usize,
) -> Option<usize>
where
    T: PartialEq,
{
    if k == 0 || arr.is_empty() {
        return None;
    }

    let mut max_len = 0;

    for l in 0..arr.len() {
        'right: for r in l..arr.len() {
            // Count distinct elements in arr[i..=j]
            let mut distinct = Vec::new();
            for i in l..=r {
                let element_value = &arr[i];
                if !distinct.contains(&element_value) {
                    distinct.push(element_value);
                }
                if distinct.len() > k {
                    // no need to count further and technically we are exceeding k
                    // so break up to moving right onward.
                    break 'right;
                }
            }

            if distinct.len() <= k {
                let curr_len = r - l + 1;
                if curr_len > max_len {
                    max_len = curr_len;
                }
            } else {
                // exceeded k distinct, no need to check longer subarrays starting at i
                break;
            }
        }
    }

    if max_len == 0 { None } else { Some(max_len) }
}

fn max_sub_element_slice_len_with_at_most_k_distinct_elements_naive_with_hashmap<T>(
    arr: &[T],
    k: usize,
) -> Option<usize>
where
    T: Eq + Hash,
{
    if k == 0 || arr.is_empty() {
        return None;
    }

    let mut max_len = 0;
    let mut map_distinct_entries_and_counts = HashMap::<&T, usize>::new();
    // Moving left
    for l in 0..arr.len() {
        map_distinct_entries_and_counts.clear();
        // Check right
        for r in l..arr.len() {
            *map_distinct_entries_and_counts.entry(&arr[r]).or_insert(0) += 1;

            if map_distinct_entries_and_counts.len() <= k {
                // We have a potential winner
                max_len = max_len.max(r - l + 1)
            } else {
                // We no longer have a winner no need to to check further right
                // without moving the left.
                break;
            }
        }
    }

    if max_len == 0 { None } else { Some(max_len) }
}

enum SearchMode {
    ExpandWindowRight,
    SlideWindowRight,
}
fn max_sub_element_slice_len_with_at_most_k_distinct_elements_using_sliding_window_and_hashmap<T>(
    arr: &[T],
    k: usize,
) -> Option<usize>
where
    T: Eq + Hash,
{
    if k == 0 || arr.is_empty() {
        return None;
    }
    let mut l = 0;
    let mut r = 0;
    let mut max_len = 0;
    let mut map_distinct_entries_and_counts = HashMap::<&T, usize>::new();
    let mut search_mode = SearchMode::ExpandWindowRight;
    loop {
        // if the right side of the window reaches the end then the work is over.
        if r == arr.len() {
            break;
        }
        match search_mode {
            SearchMode::ExpandWindowRight => {
                // Expand Right happens when looking for 1st qualifying entry
                // or expanding to find the limits of the next qualifying entry.
                // Expand r.
                r += 1;
                // Add new arr[r-1] character to distinct map
                *map_distinct_entries_and_counts
                    .entry(&arr[r - 1])
                    .or_insert(0) += 1;
                if map_distinct_entries_and_counts.len() <= k {
                    max_len = max_len.max(r - l)
                } else {
                    // distinct count is bigger than k, sliding the whole window right
                    // until distinct count is no longer bigger than k
                    search_mode = SearchMode::SlideWindowRight;
                }
            }
            SearchMode::SlideWindowRight => {
                // We are sliding the whole window right now looking for the next
                // qualifying window of l..r size which is +1 bigger than previous
                // qualifying max_len

                // First remove arr[l] from the map since its element is about to drop
                // off the window.
                let map_entry = map_distinct_entries_and_counts
                    .get_mut(&arr[l])
                    .expect("Key not found");
                *map_entry -= 1; // Key should always be there.
                if *map_entry == 0 {
                    // the count has fallen to 0 remove it.
                    map_distinct_entries_and_counts.remove(&arr[l]);
                }

                // Move the window
                r += 1;
                l += 1;

                // Add new arr[r-1] character to distinct map
                *map_distinct_entries_and_counts
                    .entry(&arr[r - 1])
                    .or_insert(0) += 1;

                if map_distinct_entries_and_counts.len() <= k {
                    // distinct count is no longer bigger than k
                    // record the new max and grow window to the right.
                    max_len = max_len.max(r - l);
                    search_mode = SearchMode::ExpandWindowRight;
                }
            }
        }
    }
    if max_len == 0 { None } else { Some(max_len) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_array_sliding_window_and_hashmap() {
        let arr: Vec<char> = vec![];
        assert_eq!(
            max_sub_element_slice_len_with_at_most_k_distinct_elements_using_sliding_window_and_hashmap(
                &arr, 2
            ),
            None
        );
    }

    #[test]
    fn test_k_zero_sliding_window_and_hashmap() {
        let arr = vec![1, 2, 3];
        assert_eq!(
            max_sub_element_slice_len_with_at_most_k_distinct_elements_using_sliding_window_and_hashmap(
                &arr, 0
            ),
            None
        );
    }

    #[test]
    fn test_single_element_array_sliding_window_and_hashmap() {
        let arr = vec!['a'];
        assert_eq!(
            max_sub_element_slice_len_with_at_most_k_distinct_elements_using_sliding_window_and_hashmap(
                &arr, 1
            ),
            Some(1)
        );
        assert_eq!(
            max_sub_element_slice_len_with_at_most_k_distinct_elements_using_sliding_window_and_hashmap(
                &arr, 2
            ),
            Some(1)
        );
    }

    #[test]
    fn test_all_same_elements_sliding_window_and_hashmap() {
        let arr = vec![5, 5, 5, 5];
        assert_eq!(
            max_sub_element_slice_len_with_at_most_k_distinct_elements_using_sliding_window_and_hashmap(
                &arr, 1
            ),
            Some(4)
        );
        assert_eq!(
            max_sub_element_slice_len_with_at_most_k_distinct_elements_using_sliding_window_and_hashmap(
                &arr, 2
            ),
            Some(4)
        );
    }

    #[test]
    fn test_simple_case_sliding_window_and_hashmap() {
        let arr = vec!['a', 'b', 'c', 'a', 'b', 'c'];
        assert_eq!(
            max_sub_element_slice_len_with_at_most_k_distinct_elements_using_sliding_window_and_hashmap(
                &arr, 2
            ),
            Some(2)
        );

        assert_eq!(
            max_sub_element_slice_len_with_at_most_k_distinct_elements_using_sliding_window_and_hashmap(
                &arr, 3
            ),
            Some(6)
        );
    }

    #[test]
    fn test_mixed_types_sliding_window_and_hashmap() {
        let arr = vec![1, 2, 1, 2, 3];
        assert_eq!(
            max_sub_element_slice_len_with_at_most_k_distinct_elements_using_sliding_window_and_hashmap(
                &arr, 2
            ),
            Some(4)
        );
        assert_eq!(
            max_sub_element_slice_len_with_at_most_k_distinct_elements_using_sliding_window_and_hashmap(
                &arr, 1
            ),
            Some(1)
        );
        assert_eq!(
            max_sub_element_slice_len_with_at_most_k_distinct_elements_using_sliding_window_and_hashmap(
                &['a', 'b', 'a', 'c', 'b'],
                2
            ),
            Some(3) // 'a', 'b', 'a'
        );
    }

    #[test]
    fn test_large_k_sliding_window_and_hashmap() {
        let arr = vec![1, 2, 3, 4, 5];
        assert_eq!(
            max_sub_element_slice_len_with_at_most_k_distinct_elements_using_sliding_window_and_hashmap(
                &arr, 10
            ),
            Some(5)
        );
    }

    #[test]
    fn test_k_equals_array_length_sliding_window_and_hashmap() {
        let arr = vec!['x', 'y', 'z'];
        assert_eq!(
            max_sub_element_slice_len_with_at_most_k_distinct_elements_using_sliding_window_and_hashmap(
                &arr, 3
            ),
            Some(3)
        );
    }

    #[test]
    fn test_no_valid_subarray_sliding_window_and_hashmap() {
        let arr = vec![1, 2, 3];
        assert_eq!(
            max_sub_element_slice_len_with_at_most_k_distinct_elements_using_sliding_window_and_hashmap(
                &arr, 0
            ),
            None
        );
    }

    //////////
    #[test]
    fn test_empty_array_naive_with_hashmap() {
        let arr: Vec<char> = vec![];
        assert_eq!(
            max_sub_element_slice_len_with_at_most_k_distinct_elements_naive_with_hashmap(&arr, 2),
            None
        );
    }

    #[test]
    fn test_k_zero_naive_with_hashmap() {
        let arr = vec![1, 2, 3];
        assert_eq!(
            max_sub_element_slice_len_with_at_most_k_distinct_elements_naive_with_hashmap(&arr, 0),
            None
        );
    }

    #[test]
    fn test_single_element_array_naive_with_hashmap() {
        let arr = vec!['a'];
        assert_eq!(
            max_sub_element_slice_len_with_at_most_k_distinct_elements_naive_with_hashmap(&arr, 1),
            Some(1)
        );
        assert_eq!(
            max_sub_element_slice_len_with_at_most_k_distinct_elements_naive_with_hashmap(&arr, 2),
            Some(1)
        );
    }

    #[test]
    fn test_all_same_elements_naive_with_hashmap() {
        let arr = vec![5, 5, 5, 5];
        assert_eq!(
            max_sub_element_slice_len_with_at_most_k_distinct_elements_naive_with_hashmap(&arr, 1),
            Some(4)
        );
        assert_eq!(
            max_sub_element_slice_len_with_at_most_k_distinct_elements_naive_with_hashmap(&arr, 2),
            Some(4)
        );
    }

    #[test]
    fn test_simple_case_naive_with_hashmap() {
        let arr = vec!['a', 'b', 'c', 'a', 'b', 'c'];
        assert_eq!(
            max_sub_element_slice_len_with_at_most_k_distinct_elements_naive_with_hashmap(&arr, 2),
            Some(2)
        );

        assert_eq!(
            max_sub_element_slice_len_with_at_most_k_distinct_elements_naive_with_hashmap(&arr, 3),
            Some(6)
        );
    }

    #[test]
    fn test_mixed_types_naive_with_hashmap() {
        let arr = vec![1, 2, 1, 2, 3];
        assert_eq!(
            max_sub_element_slice_len_with_at_most_k_distinct_elements_naive_with_hashmap(&arr, 2),
            Some(4)
        );
        assert_eq!(
            max_sub_element_slice_len_with_at_most_k_distinct_elements_naive_with_hashmap(&arr, 1),
            Some(1)
        );
        assert_eq!(
            max_sub_element_slice_len_with_at_most_k_distinct_elements_naive_with_hashmap(
                &['a', 'b', 'a', 'c', 'b'],
                2
            ),
            Some(3) // 'a', 'b', 'a'
        );
    }

    #[test]
    fn test_large_k_naive_with_hashmap() {
        let arr = vec![1, 2, 3, 4, 5];
        assert_eq!(
            max_sub_element_slice_len_with_at_most_k_distinct_elements_naive_with_hashmap(&arr, 10),
            Some(5)
        );
    }

    #[test]
    fn test_k_equals_array_length_naive_with_hashmap() {
        let arr = vec!['x', 'y', 'z'];
        assert_eq!(
            max_sub_element_slice_len_with_at_most_k_distinct_elements_naive_with_hashmap(&arr, 3),
            Some(3)
        );
    }

    #[test]
    fn test_no_valid_subarray_naive_with_hashmap() {
        let arr = vec![1, 2, 3];
        assert_eq!(
            max_sub_element_slice_len_with_at_most_k_distinct_elements_naive_with_hashmap(&arr, 0),
            None
        );
    }

    //////////
    #[test]
    fn test_empty_array_naive_very() {
        let arr: Vec<char> = vec![];
        assert_eq!(
            max_sub_element_slice_len_with_at_most_k_distinct_elements_naive_very(&arr, 2),
            None
        );
    }

    #[test]
    fn test_k_zero_naive_very() {
        let arr = vec![1, 2, 3];
        assert_eq!(
            max_sub_element_slice_len_with_at_most_k_distinct_elements_naive_very(&arr, 0),
            None
        );
    }

    #[test]
    fn test_single_element_array_naive_very() {
        let arr = vec!['a'];
        assert_eq!(
            max_sub_element_slice_len_with_at_most_k_distinct_elements_naive_very(&arr, 1),
            Some(1)
        );
        assert_eq!(
            max_sub_element_slice_len_with_at_most_k_distinct_elements_naive_very(&arr, 2),
            Some(1)
        );
    }

    #[test]
    fn test_all_same_elements_naive_very() {
        let arr = vec![5, 5, 5, 5];
        assert_eq!(
            max_sub_element_slice_len_with_at_most_k_distinct_elements_naive_very(&arr, 1),
            Some(4)
        );
        assert_eq!(
            max_sub_element_slice_len_with_at_most_k_distinct_elements_naive_very(&arr, 2),
            Some(4)
        );
    }

    #[test]
    fn test_simple_case_naive_very() {
        let arr = vec!['a', 'b', 'c', 'a', 'b', 'c'];
        assert_eq!(
            max_sub_element_slice_len_with_at_most_k_distinct_elements_naive_very(&arr, 2),
            Some(2)
        );

        assert_eq!(
            max_sub_element_slice_len_with_at_most_k_distinct_elements_naive_very(&arr, 3),
            Some(6)
        );
    }

    #[test]
    fn test_mixed_types_naive_very() {
        let arr = vec![1, 2, 1, 2, 3];
        assert_eq!(
            max_sub_element_slice_len_with_at_most_k_distinct_elements_naive_very(&arr, 2),
            Some(4)
        );
        assert_eq!(
            max_sub_element_slice_len_with_at_most_k_distinct_elements_naive_very(&arr, 1),
            Some(1)
        );
        assert_eq!(
            max_sub_element_slice_len_with_at_most_k_distinct_elements_naive_very(
                &['a', 'b', 'a', 'c', 'b'],
                2
            ),
            Some(3) // 'a', 'b', 'a'
        );
    }

    #[test]
    fn test_large_k_naive_very() {
        let arr = vec![1, 2, 3, 4, 5];
        assert_eq!(
            max_sub_element_slice_len_with_at_most_k_distinct_elements_naive_very(&arr, 10),
            Some(5)
        );
    }

    #[test]
    fn test_k_equals_array_length_naive_very() {
        let arr = vec!['x', 'y', 'z'];
        assert_eq!(
            max_sub_element_slice_len_with_at_most_k_distinct_elements_naive_very(&arr, 3),
            Some(3)
        );
    }

    #[test]
    fn test_no_valid_subarray_naive_very() {
        let arr = vec![1, 2, 3];
        assert_eq!(
            max_sub_element_slice_len_with_at_most_k_distinct_elements_naive_very(&arr, 0),
            None
        );
    }
}
