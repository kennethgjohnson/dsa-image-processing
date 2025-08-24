use crate::tutorial::common_util::{make_matrix, print_matrix_2d_vec_matrix};

pub fn prefix_sums_2d() {
    // 4a) Implement the Prefix sum
    // 4b) Implement Submatrix sums using inclusion-exclusion.
    println!("--> Vec_2d Prefix Sum Build");
    let row_count = 5;
    let col_count = 10;

    let matrix_2d_vec = make_matrix(row_count, col_count, 0);
    println!("Source Matrix:");
    print_matrix_2d_vec_matrix(&matrix_2d_vec);
    println!("Building prefix sum 2d matrix from source...");
    let prefix_sum_matrix = make_prefix_sum_matrix(&matrix_2d_vec);
    println!("2d Prefix Sum Matrix:");
    print_matrix_2d_vec_matrix(&prefix_sum_matrix);

    println!("--> Submatrix sums using inclusion-exclusion");
    let result = sub_matrix_sum_using_prefix_matrix(&prefix_sum_matrix, 2, 2, 4, 7);
    println!("The sub matrix sum of 2,2:4,7 = {}", result);
}

// O(n*m) build prefix sum matrix
pub fn make_prefix_sum_matrix(matrix: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let row_size = matrix.len();
    if row_size == 0 {
        // No rows = empty
        return Vec::new();
    }
    let col_size = matrix[0].len();
    if col_size == 0 {
        // No cols = empty
        return Vec::new();
    }
    // 1) Initialize destination matrix
    let mut prefix_matrix = Vec::with_capacity(row_size);

    // 2) Create the first row, and populate it's first element
    let mut first_row = Vec::with_capacity(col_size);
    first_row.push(matrix[0][0]); // Initializing corner.

    // 3) Populate the rest of the first row
    for col_idx in 1..col_size {
        first_row.push(first_row[col_idx - 1] + matrix[0][col_idx]);
    }
    // 4) Add it to the matrix.
    prefix_matrix.push(first_row);

    // 4) Create the rest of the rows populating only the first column.
    for row_idx in 1..row_size {
        let mut new_row = Vec::with_capacity(col_size);
        new_row.push(prefix_matrix[row_idx - 1][0] + matrix[row_idx][0]);
        prefix_matrix.push(new_row);
    }

    // 5) Build out the rest of the prefix cells row by row from row 1 col 1 row by row.
    for row_idx in 1..row_size {
        for col_idx in 1..col_size {
            // area above
            let above = prefix_matrix[row_idx - 1][col_idx];
            // area to the left
            let left = prefix_matrix[row_idx][col_idx - 1];
            let corner_that_gets_double_counted = prefix_matrix[row_idx - 1][col_idx - 1];
            prefix_matrix[row_idx].push(
                matrix[row_idx][col_idx] + above + left - corner_that_gets_double_counted, // fix double-count
            );
        }
    }

    prefix_matrix
}

// O(1) get sub matrix sum of matrix using prefix matrix.
// I left out the checking that r is to the right of l, this is just a tutorial
// for now.
pub fn sub_matrix_sum_using_prefix_matrix(
    prefix_matrix: &Vec<Vec<i32>>,
    l_row: usize,
    l_col: usize,
    r_row: usize,
    r_col: usize,
) -> i32 {
    if l_row == 0 && l_col == 0 {
        // there is no left, top, or top left
        prefix_matrix[r_row][r_col]
    } else if l_row == 0 {
        // there is no top or top left
        // remove left rectangle
        prefix_matrix[r_row][r_col] - prefix_matrix[r_row][l_col - 1]
    } else if l_col == 0 {
        // there is no left and no top left
        // remove top rectangle
        prefix_matrix[r_row][r_col] - prefix_matrix[l_row - 1][r_col]
    } else {
        prefix_matrix[r_row][r_col]
      - prefix_matrix[l_row-1][r_col]      // remove top rectangle
      - prefix_matrix[r_row][l_col-1]      // remove left rectangle
      + prefix_matrix[l_row-1][l_col-1] // add back top-left overlap (double removed)
    }
}
