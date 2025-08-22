use crate::tutorial::common_util::make_matrix;

pub fn warm_up_2d_arrays_and_flattening_them() {
    // 1) Warm-Up - 2D Arrays
    // 1a) 2d Arrays in rust.
    println!("==> 1) Warm-Up - 2D Arrays aka Matrices\n");
    println!("--> Arrays");
    let array = make_array();
    print!(" {:>4} |", "idx");
    for col_idx in 0..array[0].len() {
        print!(" {:>4} |", col_idx);
    }
    print!("\n");
    for _ in 0..=array[0].len() {
        print!("------|");
    }
    print!("\n");
    for row_idx in 0..array.len() {
        let row = &array[row_idx];
        print!(" {:>4} |", row_idx);
        row.iter().for_each(|column_value| {
            print!(" {:>4} |", column_value);
        });
        print!("\n");
    }
    let row_idx = 1;
    let col_idx = 1;
    println!("&array[1][1] {:?}", &array[row_idx][col_idx]);

    println!("\n");
    ////////////////////////////
    println!("--> Vectors");
    let matrix = make_matrix(10, 10, 0);
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
        let row = &matrix[row_idx];
        print!(" {:>4} |", row_idx);
        row.iter().for_each(|column_value| {
            print!(" {:>4} |", column_value);
        });
        print!("\n");
    }
    let row_idx = 5;
    let col_idx = 5;
    println!("&matrix[5][5] {:?}", &matrix[row_idx][col_idx]);

    println!("\n");
    //////////////////////////
    println!("--> Sub Slice");
    let matrix = make_matrix(10, 10, 0);
    let row_range_start = 3;
    let row_range_end = 7;
    let col_range_start = 2;
    let col_range_end = 6;
    println!("So effectively we would like to get matrix[3..7][2..6]");
    // Note we are flattening the columns into a slice &[i32] stored in a vector of rows
    let sub_matrix_slice: Vec<&[i32]> = matrix[row_range_start..row_range_end]
        .iter()
        .map(|columns| &columns[col_range_start..col_range_end])
        .collect();

    print!(" {:>4} |", "idx");
    for col_idx in 0..sub_matrix_slice[0].len() {
        print!(" {:>4} |", (col_range_start + col_idx));
    }
    print!("\n");
    for _ in 0..=sub_matrix_slice[0].len() {
        print!("------|");
    }
    print!("\n");
    for row_idx in 0..sub_matrix_slice.len() {
        let row = &sub_matrix_slice[row_idx];
        print!(" {:>4} |", row_range_start + row_idx);
        row.iter().for_each(|column_value| {
            print!(" {:>4} |", column_value);
        });
        print!("\n");
    }
    println!("\n");
    // That took way to much work, would have hoped rust had some sugar syntax
    // for this but oh well I asume its the usually close to what it actually
    // does language approach.
    //////////////////////////////////////////////////////////////////

    // 1b) flatten to 1D form: Row-Major
    let column_count = 10;
    let matrix = make_matrix(10, 10, 0);
    let row_major_1d_flatten = row_major_1d_flatten_i32(matrix);
    println!("1d Row Major: {:?}\n", row_major_1d_flatten);
    println!("Let's traverse it with for each row, for each column");
    let row_count = row_major_1d_flatten.len() / column_count;
    print!(" {:>4} |", "idx");
    for col_idx in 0..column_count {
        print!(" {:>4} |", col_idx);
    }
    print!("\n");
    for _ in 0..=column_count {
        print!("------|");
    }
    print!("\n");
    // just print out the rows
    for row_idx in 0..row_count {
        print!(" {:>4} |", row_idx);
        for col_idx in 0..column_count {
            print!(
                " {:>4} |",
                row_major_1d_flatten[row_idx * column_count + col_idx]
            );
        }
        print!("\n");
    }
    println!("\n");
    print!("Let's traverse it with for each row, get a slice of columns, and ");
    print!("print the columns as if we are back in a 2d array.\n");
    print!(" {:>4} |", "idx");
    for col_idx in 0..column_count {
        print!(" {:>4} |", col_idx);
    }
    print!("\n");
    for _ in 0..=column_count {
        print!("------|");
    }
    print!("\n");
    // just print out the rows
    for row_idx in 0..row_count {
        print!(" {:>4} |", row_idx);
        let row_start = row_idx * column_count;
        let row_end = row_start + column_count;
        let row = &row_major_1d_flatten[row_start..row_end];
        row.iter().for_each(|column_value| {
            print!(" {:>4} |", column_value);
        });
        print!("\n");
    }
    print!("\n\n");
    println!(
        "Row Major get at flat[5,5]: {:?}",
        get_row_major(&row_major_1d_flatten, 5, 5, column_count)
    );
    print!("\n\n");
    ////////////////////////////////////

    // 1c) flatten to 1D form: Col-Major
    let column_count = 10;
    let matrix = make_matrix(10, 10, 0);
    let col_major_1d_flatten = col_major_1d_flatten_i32(matrix);
    println!("1d Col Major: {:?}\n", col_major_1d_flatten);
    let row_count = col_major_1d_flatten.len() / column_count;
    print!(" {:>4} |", "idx");
    for col_idx in 0..column_count {
        print!(" {:>4} |", col_idx);
    }
    print!("\n");
    for _ in 0..=column_count {
        print!("------|");
    }
    print!("\n");
    // just print out the rows
    for row_idx in 0..row_count {
        print!(" {:>4} |", row_idx);
        for col_idx in 0..column_count {
            print!(
                " {:>4} |",
                col_major_1d_flatten[col_idx * row_count + row_idx]
            );
        }
        print!("\n");
    }
    println!("\n");
    println!(
        "Col Major get at flat[5,5]: {:?}",
        get_col_major(&col_major_1d_flatten, 5, 5, row_count)
    );
    print!("\n\n");
}

fn make_array() -> [[i32; 3]; 3] {
    let array = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    array
}

fn row_major_1d_flatten_i32(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    flatten_row_major(&matrix)
}

fn col_major_1d_flatten_i32(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    flatten_col_major(&matrix)
}

/// Flatten a 2D vector into 1D row-major order.
pub fn flatten_row_major<T: Clone>(matrix: &Vec<Vec<T>>) -> Vec<T> {
    let row_count = matrix.len();
    let col_count = matrix[0].len();
    let mut result = Vec::with_capacity(row_count * col_count);
    for row_idx in 0..row_count {
        for col_idx in 0..col_count {
            let value = matrix[row_idx][col_idx].clone();
            result.push(value);
        }
    }
    result
}

/// Flatten a 2D vector into 1D column-major order.
pub fn flatten_col_major<T: Clone>(matrix: &Vec<Vec<T>>) -> Vec<T> {
    let row_count = matrix.len();
    let col_count = matrix[0].len();
    let mut result = Vec::with_capacity(row_count * col_count);
    for col_idx in 0..col_count {
        for row_idx in 0..row_count {
            let value = matrix[row_idx][col_idx].clone();
            result.push(value);
        }
    }
    result
}

/// Access element in row-major flattened array.
/// rows = number of rows in the original matrix
/// cols = number of columns in the original matrix
pub fn get_row_major<T>(flat: &Vec<T>, row: usize, col: usize, cols: usize) -> &T {
    &flat[row * cols + col]
}

/// Access element in column-major flattened array.
pub fn get_col_major<T>(flat: &Vec<T>, row: usize, col: usize, rows: usize) -> &T {
    &flat[col * rows + row]
}
