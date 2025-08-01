mod tutorial;
use std::{
    env, ptr,
    time::{Duration, Instant},
};
use tutorial::{
    arrays_module1::arrays_module1_memory_access_patterns,
    arrays_module2::arrays_module2_performance_tradeoffs_benchmark,
    arrays_module3::arrays_module3_dynamic_arrays_and_amortized_analysis,
};

fn main() {
    match env::args().nth(1).as_deref() {
        Some("1") => arrays_module1_memory_access_patterns(),
        Some("2") => arrays_module2_performance_tradeoffs_benchmark(),
        Some("3") => arrays_module3_dynamic_arrays_and_amortized_analysis(),
        Some("4") => arrays_module4_core_array_algorithms(),
        _ => println!(
            "No tutorial module specified or it is invalid. Specify which tutorial module you wish to run by specifying a number following the command i.e. cargo run --release -- 2"
        ),
    }
}

fn arrays_module4_core_array_algorithms() {
    // We will be going over 3 fundamental paterns and perhaps a variant or two,
    // including benchmarking information where appropriate, for some in-place
    // swapping algorithms/techniques.

    // 1 - Two-Pointer Technique - Array reversal
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

    // 2 - Cyclic Rotation - basically a shift left/right with overflow wrap around.
    // We will implement the rotate_right_naive ,rotate_right, and rotate_left
    // rotate_right_naive will be well naive...
    println!(
        "==> Cyclic Rotation - Array overflowing rotate left or right naive - right naive O(k*n), right O(n), left O(n)"
    );
    println!(
        "Data Size | RNaive Time (µs) | RR Time (µs) |      Ratio      | LR Time (µs) | LR Math to RR Time (µs) | Ratio"
    );
    println!(
        "----------|------------------|--------------|-----------------|--------------|-------------------------|------"
    );
    for size in [
        1_000, 2_000, 4_000, 8_000, 16_000, 32_000, 64_000, 128_000, 256_000, 512_000, 1_024_000,
    ] {
        let mut arr = create_array(size);
        let mut arr2 = create_array(size);
        let mut arr3 = create_array(size);
        let mut arr4 = create_array(size);
        let start = Instant::now();
        rotate_right_naive(&mut arr, 100_000);
        let time_rotate_right_naive = start.elapsed();
        let start = Instant::now();
        rotate_right(&mut arr2, 100_000);
        let time_rotate_right = start.elapsed();
        let start = Instant::now();
        rotate_left(&mut arr3, 100_000);
        let time_rotate_left = start.elapsed();
        let start = Instant::now();
        rotate_left_math_rotate_right(&mut arr4, 100_000);
        let time_rotate_left_math_rotate_right = start.elapsed();
        print_output_rotate_algo_result(
            size,
            time_rotate_right_naive,
            time_rotate_right,
            time_rotate_left,
            time_rotate_left_math_rotate_right,
        );
    }

    // TODO: 3. Move Zeros to End

    // TODO: 4. Dutch National Flag (Three-Way Partitioning)
}

fn create_array(element_count: usize) -> Vec<i32> {
    let mut arr = if element_count == 0 {
        Vec::new()
    } else {
        Vec::with_capacity(element_count)
    };
    for i in 1..=(element_count as i32) {
        arr.push(i)
    }
    arr
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

fn rotate_right_naive<T: Copy>(arr: &mut Vec<T>, rotation: usize) {
    // Move one position rotation times - O(rotation*n) time!
    for _ in 0..rotation {
        let temp = arr[arr.len() - 1]; // save last element
        for i in (1..arr.len()).rev() {
            arr[i] = arr[i - 1]; // shift everything right
        }
        arr[0] = temp; // put saved element at front
    }
}

// Realize that Array is broken into two parts, A and B -> B needs to move left of A for right rotations
// and A needs to move right of B for left rotations.
// So if we are moving left algorithmically we
// reverse A and reverse B, then reverse the whole array.
// And if we are moving right algorithmically we
// reverse the whole array, reverse B and reverse A
// important to note that the actual order of A and B reversals don't matter, only that
// it happens before or after the global reverse of the array.
//
// The amount we are rotating left can also be converted to a right rotation using
// the equivalence that left by rotation == right by (n − rotation mod n)
fn rotate_right<T: Copy>(arr: &mut Vec<T>, rotation: usize) {
    let len = arr.len();
    if len != 0 {
        let rotation = rotation % len;
        arr.reverse();
        arr[..rotation].reverse();
        arr[rotation..].reverse();
    }
}

fn rotate_left<T: Copy>(arr: &mut Vec<T>, rotation: usize) {
    let len = arr.len();
    if len != 0 {
        let rotation = rotation % len;
        // Solution A - mathematical mirror
        arr[rotation..].reverse();
        arr[..rotation].reverse();
        arr.reverse();
    }
}

fn rotate_left_math_rotate_right<T: Copy>(arr: &mut Vec<T>, rotation: usize) {
    let len = arr.len();
    if len != 0 {
        let rotation = rotation % len;
        // Solution B - Mathematically converted to a right rotation amount.
        // left by rotation == right by (n − rotation mod n)
        let right_rotation = (len - rotation) % len;
        rotate_right(arr, right_rotation);
    }
}

fn print_output_rotate_algo_result(
    element_count: usize,
    time_rotate_right_naive: Duration,
    time_rotate_right: Duration,
    time_rotate_left: Duration,
    time_rotate_left_math_rotate_right: Duration,
) {
    let rotate_right_naive_uq = time_rotate_right_naive.as_micros() as f64;
    let rotate_right_uq = time_rotate_right.as_micros() as f64;
    let rotate_left_uq = time_rotate_left.as_micros() as f64;
    let rotate_left_math_rotate_right_uq = time_rotate_left_math_rotate_right.as_micros() as f64;

    let naive_right_to_right_ratio =
        time_rotate_right_naive.as_nanos() as f64 / time_rotate_right.as_nanos().max(1) as f64;

    let rotate_left_math_rotate_right_to_rotate_left_ratio = time_rotate_left.as_nanos() as f64
        / time_rotate_left_math_rotate_right.as_nanos().max(1) as f64;

    println!(
        "{:<9} | {:>16} | {:>12} | {:>14.1}x | {:>12} | {:>23} | {:>4.1}x",
        element_count,
        rotate_right_naive_uq,
        rotate_right_uq,
        naive_right_to_right_ratio,
        rotate_left_uq,
        rotate_left_math_rotate_right_uq,
        rotate_left_math_rotate_right_to_rotate_left_ratio
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

#[test]
fn test_rotate_right_naive() {
    let mut arr = create_array(5);
    rotate_right_naive(&mut arr, 2);
    // after 2 right-shifts [1,2,3,4,5] → [4,5,1,2,3]
    assert_eq!(arr, vec![4, 5, 1, 2, 3]);

    let mut arr = create_array(3);
    rotate_right_naive(&mut arr, 5); // 5 % 3 == 2
    assert_eq!(arr, vec![2, 3, 1]);
}

#[test]
fn test_rotate_right() {
    let mut arr = create_array(5);
    rotate_right(&mut arr, 2);
    // after 2 right-shifts [1,2,3,4,5] → [4,5,1,2,3]
    assert_eq!(arr, vec![4, 5, 1, 2, 3]);

    let mut arr = create_array(3);
    rotate_right(&mut arr, 5); // 5 % 3 == 2
    assert_eq!(arr, vec![2, 3, 1]);

    let mut arr = create_array(0);
    rotate_right(&mut arr, 5); // no dividing by 0 errors.
    assert_eq!(arr, Vec::new());
}

#[test]
fn test_rotate_left() {
    let mut arr = create_array(5);
    rotate_left(&mut arr, 2);
    // after 2 left-shifts [1,2,3,4,5] → [4,5,1,2,3]
    assert_eq!(arr, vec![3, 4, 5, 1, 2]);

    let mut arr = create_array(3);
    rotate_left(&mut arr, 5); // 5 % 3 == 2
    assert_eq!(arr, vec![3, 1, 2]);

    let mut arr = create_array(0);
    rotate_left(&mut arr, 5); // no dividing by 0 errors.
    assert_eq!(arr, Vec::new());
}
