mod basic_matrix_operations;
mod warm_up_2d_arrays_and_flattening_them;

use basic_matrix_operations::basic_matrix_operations;
use warm_up_2d_arrays_and_flattening_them::warm_up_2d_arrays_and_flattening_them;

pub fn arrays_module6_multi_dimensional_arrays_and_cache() {
    println!("Image Processing DSA - Module 6: Multi-Dimensional Arrays & Cache");

    // 1) Warm-Up - 2D Arrays
    // 1a) 2d Arrays in rust.
    // 1b) flatten to 1D form: Row-Major
    // 1c) flatten to 1D form: Col-Major
    warm_up_2d_arrays_and_flattening_them();

    // 2) Basic Matrix Operations
    // 2a1) Row Sum
    // 2a2) Col Sum
    // 2b) Transpose
    // 2c) Rotate 90 degrees clockwise
    basic_matrix_operations();

    // 3) Matrix Multiplication
    // 3a) Naive Triple-loop (Investigate inefficiencies with innerlooop jump around)
    // 3b) Optimize: precompute transpose(B) and use it to get cache-friendly row access.
    // 3c) Loop Tiling / Blocking in Matrix Multiplication
    // Note: Experement with blocksizes 8,16,32,64
    // 3d) Compare the performance of all 3 approaches using The performance comparison
    // suggestion of all three matrix multiplication approaches on matrices of size
    // 500 and 1000

    // 4) 2D Prefix Sums
    // 4a) Implement the Prefix sum
    // 4b) Implement Submatrix sums using inclusion-exclusion.

    // 5) Cache-Friendly vs. Cache-Unfriendly Traversals
    // 5a) Row-major sums
    // 5b) Col-major sums
    // 5c) Evaluate Claim: row-major is faster on large matrices.

    // 6) Advanced Patterns
    // 6a) Spiral Traversal

    // 6b) Maximum submatrix sum (Kadane's 2D Extension)

    // 7) Project: Mini Image Processor
    // 7a) Setup the basic
    // 7b) Brightness Adjustment
    // 7c) Transpose image (x/y swap)
    // 7d) Rotate 90 Degrees
    // 7e) Blur (replace each pixel with average of it's neighbors)
    // 7f) Edge detection (difference with neighbors)
}
