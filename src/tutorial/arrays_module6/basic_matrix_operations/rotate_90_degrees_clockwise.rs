use crate::tutorial::common_util::{make_matrix, print_matrix_2d_vec_matrix};

pub fn rotate_90_degrees_clockwise() {
    println!("---> Rotate 90 Degrees Clockwise (Rect)");
    let col_count = 10;
    let row_count = 5;
    let matrix = make_matrix(row_count, col_count, 0);
    println!("Before Matrix 2d Vec<Vec<T>>:");
    print_matrix_2d_vec_matrix(&matrix);
    let rotated_matrix = rotate_2d_vec(matrix, 3);
    println!("After Rotate 2d Vec<Vec<T>>:");
    print_matrix_2d_vec_matrix(&rotated_matrix);
}

fn rotate_2d_vec<T: Copy>(matrix: Vec<Vec<T>>, block_size: usize) -> Vec<Vec<T>> {
    let row_count = matrix.len();

    if row_count < 1 || matrix[0].len() < 1 {
        panic!("Matrix must be a 2d matrix with at least 1 element.");
    }
    let col_count = matrix[0].len();

    // Make new rows vector based on amount of columns
    let mut new_matrix = vec![vec![matrix[0][0]; row_count]; col_count];

    for block_start_row_idx in (0..row_count).step_by(block_size) {
        for block_start_col_idx in (0..col_count).step_by(block_size) {
            // Determine the row/cell ends (some blocks get chopped off)
            let row_end = (block_start_row_idx + block_size).min(row_count);
            let col_end = (block_start_col_idx + block_size).min(col_count);

            // Transpose the block
            for cell_row_idx in block_start_row_idx..row_end {
                for cell_col_idx in block_start_col_idx..col_end {
                    new_matrix[cell_col_idx][row_count - cell_row_idx - 1] =
                        matrix[cell_row_idx][cell_col_idx];
                }
            }
        }
    }
    new_matrix
}
