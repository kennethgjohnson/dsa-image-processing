use crate::tutorial::common_util::create_array;
use std::time::{Duration, Instant};
pub fn arrays_module4_core_array_algorithms_cyclical_rotation() {
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
    for size in [1_000, 2_000, 4_000, 8_000, 16_000, 32_000, 64_000, 128_000] {
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
    print!("\n\n\n");
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
