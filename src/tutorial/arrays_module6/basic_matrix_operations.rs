mod rotate_90_degrees_clockwise;
mod row_sum_column;
mod transpose;

use crate::tutorial::arrays_module6::basic_matrix_operations::{
    rotate_90_degrees_clockwise::rotate_90_degrees_clockwise, row_sum_column::row_sum_column_sum,
    transpose::transpose,
};

pub fn basic_matrix_operations() {
    println!("==> 2) Basic Matrix Operations\n");
    // 2) Basic Matrix Operations
    row_sum_column_sum();

    // 2b) Transpose
    transpose();

    // 2c) Rotate 90 degrees clockwise
    rotate_90_degrees_clockwise();
}
