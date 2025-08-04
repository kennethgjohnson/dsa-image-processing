use super::common_util::{print_header_one_metric_result, print_output_one_metric_result};
use std::time::Instant;

pub fn arrays_module4_core_array_algorithms_move_zeros_to_end() {
    // - insight is that you don't move the zeros, rather you move the non zeros to the fron and overwrite the tail with new zeros.
    println!("==> In place move zeros to end O(n)");
    print_header_one_metric_result();
    let size_interval = 1000;
    let size_count = 18;
    let threshold = 16_000;
    // starting at
    let arr_sizes: Vec<usize> = (0..size_count)
        .map(|i| size_interval << i)
        .filter(|element| *element >= threshold)
        .collect();
    for size in arr_sizes {
        let mut arr = create_array_with_zeros(size);
        let start = Instant::now();
        move_zeros_to_end(&mut arr);
        let time = start.elapsed();
        print_output_one_metric_result(size, time);
    }
    print!("\n\n\n");
}

fn create_array_with_zeros(element_count: usize) -> Vec<i32> {
    let mut arr = if element_count == 0 {
        Vec::new()
    } else {
        Vec::with_capacity(element_count)
    };
    for i in 1..=(element_count as i32) {
        if i % 3 == 0 {
            arr.push(0);
        } else {
            arr.push(i);
        }
    }
    arr
}

fn move_zeros_to_end(arr: &mut [i32]) {
    // Move non 0's forward
    let mut write_pos = 0;
    for read_pos in 0..arr.len() {
        if arr[read_pos] != 0 {
            // Move to write pos
            arr[write_pos] = arr[read_pos];
            // Move write pos forward.
            write_pos += 1;
        }
    }

    // Fill remainder from write pos to end with 0's
    // Insight: no need to care about the 0's at all, rather we were making sure
    // the non zero ordering is maintained.
    for i in write_pos..arr.len() {
        arr[i] = 0;
    }
}

#[test]
fn test_move_zeros_to_end_flag() {
    let mut arr = vec![0, 1, 0, 3, 12];
    move_zeros_to_end(&mut arr);
    assert_eq!(arr, vec![1, 3, 12, 0, 0]);
}
