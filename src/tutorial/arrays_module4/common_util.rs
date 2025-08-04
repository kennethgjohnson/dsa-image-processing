use std::time::Duration;

// Common utility functions for tutorials.
pub fn create_array(element_count: usize) -> Vec<i32> {
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

pub fn print_header_one_metric_result() {
    println!("Data size | Time (µs)");
    println!("----------|----------");
}

pub fn print_output_one_metric_result(element_count: usize, time: Duration) {
    let time_uq = time.as_micros() as f64;
    println!("{:<9} | {:>9} ", element_count, time_uq);
}
