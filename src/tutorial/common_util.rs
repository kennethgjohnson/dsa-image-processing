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

pub fn print_header(col_names: &[&str]) {
    let header = String::from(format!(
        " {} \n{}\n",          // Combination
        col_names.join(" | "), // Header Text
        col_names
            .iter()
            .map(|name| "-".repeat(name.chars().count() + 2))
            .collect::<Vec<String>>()
            .join("|")  // Line
    ));
    print!("{}", header);
}

pub fn print_output_row_ratio_compare_result(
    col_names: &[&str],
    size: usize,
    duration_data: Vec<(Duration, Duration)>,
) {
    // print size first
    print!(" {:<1$} ", size, col_names[0].chars().count());
    for i in 0..duration_data.len() {
        let durations = duration_data[i];
        let first_uq = durations.0.as_micros();
        let second_uq = durations.1.as_micros();
        let ratio = { durations.0.as_nanos().max(1) as f64 / durations.1.as_nanos().max(1) as f64 };
        let column_base_idx = i * 3;
        let first_column_width = col_names[column_base_idx + 1].chars().count();
        let second_column_width = col_names[column_base_idx + 2].chars().count();
        let ratio_column_width = col_names[column_base_idx + 3].chars().count();
        print!(
            "| {0:>1$} | {2:3$} | {4:5$.1}x ",
            first_uq,
            first_column_width,
            second_uq,
            second_column_width,
            ratio,
            ratio_column_width - 1 // -1 for the x
        );
    }
    print!("\n")
}

pub fn median_duration_index_u128(arr_durations: &[Duration]) -> usize {
    if arr_durations.is_empty() {
        panic!("No results provided in array")
    }

    // Pair each value with its original index
    let mut indexed: Vec<(usize, u128)> = arr_durations
        .iter()
        .copied()
        .enumerate()
        .map(|(i, dur)| (i, dur.as_nanos()))
        .collect();

    // Sort by value, keeping original indices
    indexed.sort_unstable_by_key(|&(_, val)| val);

    let mid = arr_durations.len() / 2;

    if arr_durations.len() % 2 == 1 {
        indexed[mid].0 // Odd: middle element
    } else {
        // Even: pick the earlier of the two middle indices (or customize as needed)
        let i1 = indexed[mid - 1].0;
        let i2 = indexed[mid].0;
        if arr_durations[i1] <= arr_durations[i2] {
            i1
        } else {
            i2
        }
    }
}
