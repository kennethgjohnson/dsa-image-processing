use crate::tutorial::common_util::{
    print_header_one_metric_result, print_output_one_metric_result,
};
use std::time::Instant;

pub fn arrays_module4_core_array_algorithms_dutch_national_flag_three_way_partitioning() {
    println!("==> Duch National Flag - move 3 types of elements into 3 ordered boundries O(n)");
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
        let mut arr = create_array_with_zeros_ones_twos(size);
        let start = Instant::now();
        dutch_flag(&mut arr);
        let time = start.elapsed();
        print_output_one_metric_result(size, time);
    }
    print!("\n\n\n");
}

fn create_array_with_zeros_ones_twos(element_count: usize) -> Vec<i32> {
    let mut arr = if element_count == 0 {
        Vec::new()
    } else {
        Vec::with_capacity(element_count)
    };
    for i in 1..=(element_count as i32) {
        arr.push(match i {
            i if i % 3 == 1 => 0,
            i if i % 3 == 2 => 1,
            _ => 2,
        });
    }
    arr
}

fn dutch_flag(arr: &mut [i32]) {
    // three  boundry zones(with a 4th working zone):
    // 0..low -> Confirmed 0's (this boundry is moving up from the start)
    // low..mid -> Confirmed 1's (this boundry is moving up by the processing)
    // mid..high -> Unknowns in progress of being tested.
    // high..len -> Confirmed 2's (this boundry is moving down from the end)
    let (mut low, mut mid, mut high) = (0, 0, arr.len() - 1);
    while mid <= high {
        // mid is our working on zone, so therefore once mid==high all zones sorted
        match arr[mid] {
            0 => {
                // We have a 0 at the mid point so move it into the low boundry
                // and move the low boundry and mid point on. Note: mid can be
                // moved on since the value from low move up potentially would be
                // a 0.
                arr.swap(mid, low);
                low += 1;
                mid += 1;
            }
            1 => {
                // we are checking at mid and found a mid so move on the mid boundry
                mid += 1;
            }
            2 => {
                // we found a high in the mid position so we need to swap it with
                // the high boundry, however note that a mid could have been moved
                // into mid position with the swap so we do not move the mid boundry
                // check up.
                arr.swap(mid, high);
                high -= 1; // high boundry moving down from the end, the others move up.
            }
            _ => panic!("We are only implementing for 3 distinct values in this throw away."),
        }
    }
}

#[test]
fn test_dutch_flag() {
    let mut arr = [2, 0, 2, 1, 1, 0];
    dutch_flag(&mut arr);
    assert_eq!(arr, [0, 0, 1, 1, 2, 2]);
}
