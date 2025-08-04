use std::{
    ptr,
    time::{Duration, Instant},
};

use crate::tutorial::common_util::create_array;

pub fn arrays_module4_core_array_algorithms_two_pointer_technique() {
    println!("==> Two-Pointer Technique - Array reversal O(n)");
    println!("Data Size | Our Time (µs) | Rust Native Time (µs) | Ratio");
    println!("----------|---------------|-----------------------|------");
    for size in [
        1_000, 2_000, 4_000, 8_000, 16_000, 32_000, 64_000, 128_000, 256_000, 512_000, 1_024_000,
    ] {
        let mut arr = create_array(size);
        let mut arr2 = create_array(size);
        let start = Instant::now();
        two_pointer_array_reversal(&mut arr);
        let time_two_point_array_reversal = start.elapsed();
        let start = Instant::now();
        arr2.reverse();
        let time_rust_reverse = start.elapsed();
        print_output_two_pointer_result_our_vs_rust(
            size,
            time_two_point_array_reversal,
            time_rust_reverse,
        )
    }
    print!("\n\n\n");
}

// I tried multiple implemnetation of two_pointer_array_reversal to try match rust's own
// .reverse() but they must have some sort of special compiling options for that function
// since even using the code from the function itself aka SOLUTION 6 only yields
// 50% of the performance of their function. In the end I reverted to SOLUTION 3
// as that is the most true to what I was trying to build before getting sucked into
// trying to match the performance of .reverse
// use std::mem;
// use std::ops::Range;
// use std::slice;
// #[inline]
fn two_pointer_array_reversal(arr: &mut Vec<i32>) {
    if arr.len() > 1 {
        // SOL 1/2
        // let mut left_idx = 0;
        // let mut right_idx = arr.len() - 1;
        // while left_idx < right_idx {
        //     // Solution 1
        //     // arr.swap(left_idx, right_idx); // Doing bounds checking so slower than reverse.
        //     // Solution 2
        //     // No bounds checking swaping in unsafe code
        //     unsafe {
        //         // // Solution 2a - manual swapping (still slower than rust native)
        //         // // Swap in place
        //         // // remember left
        //         // let temp_val = *arr.get_unchecked_mut(left_idx);
        //         // // swap right to left
        //         // *arr.get_unchecked_mut(left_idx) = *arr.get_unchecked_mut(right_idx);
        //         // // swap old left to right via temp_val
        //         // *arr.get_unchecked_mut(right_idx) = temp_val;
        //         // Solution 2b - use pointer swaping - still 3x slower than rust's reverse.
        //         ptr::swap(
        //             arr.get_unchecked_mut(left_idx),
        //             arr.get_unchecked_mut(right_idx),
        //         );
        //     }

        //     left_idx += 1;
        //     right_idx -= 1;
        // }
        // SOLUTION 3 - still slower than rust reverse by 2-3x
        let ptr = arr.as_mut_ptr();
        let mut left_idx = 0;
        let mut right_idx = arr.len() - 1;
        while left_idx < right_idx {
            unsafe {
                ptr::swap(ptr.add(left_idx), ptr.add(right_idx));
            }
            left_idx += 1;
            right_idx -= 1;
        }
        // SOLUTION 4 - still 3X slower than .reverse
        // unsafe {
        //     let ptr = arr.as_mut_ptr();
        //     for i in 0..arr.len() / 2 {
        //         ptr::swap(ptr.add(i), ptr.add(arr.len() - 1 - i));
        //     }
        // }
        // // SOLUTION 5 - start advice from a LLM lol to give hints to LLVM and use memory swaping
        // // Yea its still 3X-4X slower...
        // let half = arr.len() / 2;
        // // Tell LLVM that [0..half) and [len-half..len) don't overlap
        // let (a, b) = arr.split_at_mut(half);
        // let b = &mut b[..half];
        // for i in 0..half {
        //     // This looks a lot like revswap
        //     core::mem::swap(&mut a[i], &mut b[half - 1 - i]);
        // }
    }
    // SOLUTION 6 - EVEN RUST's own code is 2x .... they must have some compiling black magic.
    // let half_len = arr.len() / 2;
    // let Range { start, end } = arr.as_mut_ptr_range();

    // // These slices will skip the middle item for an odd length,
    // // since that one doesn't need to move.
    // let (front_half, back_half) =
    //         // SAFETY: Both are subparts of the original slice, so the memory
    //         // range is valid, and they don't overlap because they're each only
    //         // half (or less) of the original slice.
    //         unsafe {
    //             (
    //                 slice::from_raw_parts_mut(start, half_len),
    //                 slice::from_raw_parts_mut(end.sub(half_len), half_len),
    //             )
    //         };

    // // Introducing a function boundary here means that the two halves
    // // get `noalias` markers, allowing better optimization as LLVM
    // // knows that they're disjoint, unlike in the original slice.
    // revswap(front_half, back_half, half_len);

    // #[inline]
    // const fn revswap<T>(a: &mut [T], b: &mut [T], n: usize) {
    //     debug_assert!(a.len() == n);
    //     debug_assert!(b.len() == n);

    //     // Because this function is first compiled in isolation,
    //     // this check tells LLVM that the indexing below is
    //     // in-bounds. Then after inlining -- once the actual
    //     // lengths of the slices are known -- it's removed.
    //     let (a, _) = a.split_at_mut(n);
    //     let (b, _) = b.split_at_mut(n);

    //     let mut i = 0;
    //     while i < n {
    //         mem::swap(&mut a[i], &mut b[n - 1 - i]);
    //         i += 1;
    //     }
    // }
}

fn print_output_two_pointer_result_our_vs_rust(
    element_count: usize,
    our_time: Duration,
    rust_time: Duration,
) {
    let our_uq = our_time.as_micros() as f64;
    let rust_uq = rust_time.as_micros() as f64;
    let ratio = (our_time.as_nanos() / rust_time.as_nanos()) as f64;

    println!(
        "{:<9} | {:>13} | {:>21} | {:>4.1}x",
        element_count, our_uq, rust_uq, ratio
    );
}

#[test]
fn test_reverse() {
    // Multiple elements
    let mut arr = create_array(1000);
    two_pointer_array_reversal(&mut arr);
    let mut expected_result = create_array(1000);
    expected_result.reverse();
    assert_eq!(arr, expected_result);

    // Empty
    let mut arr = create_array(0);
    two_pointer_array_reversal(&mut arr);
    let mut expected_result = create_array(0);
    expected_result.reverse();
    assert_eq!(arr, expected_result);

    // Single element
    let mut arr = create_array(1);
    two_pointer_array_reversal(&mut arr);
    let mut expected_result = create_array(1);
    expected_result.reverse();
    assert_eq!(arr, expected_result);
}
