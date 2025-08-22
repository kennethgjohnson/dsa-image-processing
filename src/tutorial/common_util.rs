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

pub fn print_output_row_ratio_compare_result_micros(
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

pub fn print_output_row_ratio_compare_result_nanos(
    col_names: &[&str],
    size: usize,
    duration_data: Vec<(Duration, Duration)>,
) {
    // print size first
    print!(" {:<1$} ", size, col_names[0].chars().count());
    for i in 0..duration_data.len() {
        let durations = duration_data[i];
        let first_ns = durations.0.as_nanos();
        let second_ns = durations.1.as_nanos();
        let ratio = { durations.0.as_nanos().max(1) as f64 / durations.1.as_nanos().max(1) as f64 };
        let column_base_idx = i * 3;
        let first_column_width = col_names[column_base_idx + 1].chars().count();
        let second_column_width = col_names[column_base_idx + 2].chars().count();
        let ratio_column_width = col_names[column_base_idx + 3].chars().count();
        print!(
            "| {0:>1$} | {2:3$} | {4:5$.1}x ",
            first_ns,
            first_column_width,
            second_ns,
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

// Some deterministic pseudo random string generator: alpha_string_from_seed using XorShift64
// util.rs
struct XorShift64 {
    state: u64,
}

impl XorShift64 {
    fn new(seed: u64) -> Self {
        assert!(seed != 0, "seed must be non-zero");
        Self { state: seed }
    }

    fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.state = x;
        x.wrapping_mul(0x2545F4914F6CDD1D)
    }
}

pub fn alpha_string_from_seed(seed: u64, n: usize) -> String {
    const ALPHABET: &[u8; 52] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let mut rng = XorShift64::new(seed);
    let mut out = String::with_capacity(n);
    for _ in 0..n {
        let idx = (rng.next_u64() % ALPHABET.len() as u64) as usize;
        out.push(ALPHABET[idx] as char);
    }
    out
}

pub fn make_matrix(row_count: usize, col_count: usize, default_value: i32) -> Vec<Vec<i32>> {
    //
    let mut rows: Vec<Vec<i32>> = Vec::with_capacity(row_count as usize);
    let mut cell_number = 0;
    for _ in 0..row_count {
        let mut cols = Vec::with_capacity(col_count as usize);
        for _ in 0..col_count {
            if default_value == 0 {
                cols.push(cell_number);
            } else {
                cols.push(default_value);
            }
            cell_number += 1;
        }
        rows.push(cols);
    }
    rows
}

pub fn print_matrix_1d_vec_matrix(flat_row_major_matrix: &Vec<i32>, col_count: usize) {
    print!(" {:>4} |", "idx");
    for col_idx in 0..col_count {
        print!(" {:>4} |", col_idx);
    }
    print!("\n");
    for _ in 0..=col_count {
        print!("------|");
    }
    print!("\n");
    for row_idx in 0..(flat_row_major_matrix.len() / col_count) {
        print!(" {:>4} |", row_idx);
        for col_idx in 0..col_count {
            print!(
                " {:>4} |",
                get_row_major(flat_row_major_matrix, row_idx, col_idx, col_count)
            );
        }
        print!("\n");
    }
    println!("\n");
}

fn get_row_major<T>(flat: &Vec<T>, row: usize, col: usize, cols: usize) -> &T {
    &flat[row * cols + col]
}

pub fn print_matrix_2d_vec_matrix(matrix: &Vec<Vec<i32>>) {
    print!(" {:>4} |", "idx");
    for col_idx in 0..matrix[0].len() {
        print!(" {:>4} |", col_idx);
    }
    print!("\n");
    for _ in 0..=matrix[0].len() {
        print!("------|");
    }
    print!("\n");
    for row_idx in 0..matrix.len() {
        let row = &*matrix[row_idx];
        print!(" {:>4} |", row_idx);
        row.iter().for_each(|column_value| {
            print!(" {:>4} |", column_value);
        });
        print!("\n");
    }
    println!("\n");
}
