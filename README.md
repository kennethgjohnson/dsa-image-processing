# DSA Image Processing

A Rust project for image processing. It serves as my tutorial project vehicle for learning algorithms and data structures for now. I have several objectives to learn about for arrays, with my goal being to at the end unifying some of the knowledge gained into a image processing project - not too many features, just enough to get a good feel.

I have included the Array topics goals on the end of this Readme.md under Addendum A.

Note, the final module talks about a Putting It All Together: “Array Toolkit” Project - I still have to see if this is something I will just incorporate as the actual image processor itself or if this will be an extra lib it uses etc.

## Getting Started

### Prerequisites
- Rust (https://www.rust-lang.org/tools/install)

### Build and Run
```sh
cargo build --release
cargo run
```


## Project Structure

- `src/main.rs`: Main entry point and CLI argument parsing for tutorial modules
- `src/tutorial/`: Contains tutorial modules (arrays_module1, arrays_module2, arrays_module3, etc.)
- `Cargo.toml`: Rust project configuration and dependencies
- `target/`: Build artifacts (auto-generated)

Each tutorial module is implemented in its own file under `src/tutorial/` and can be invoked via the CLI as described above.

## Usage

This project runs different tutorial modules based on a command line argument:

- `1`: Arrays Module 1 - Memory Access Patterns
- `2`: Arrays Module 2 - Performance Tradeoffs Benchmark
- `3`: Arrays Module 3 - Dynamic Arrays and Amortized Analysis

To run a specific module, use:
```sh
cargo run --release -- <module_number>
```
For example, to run Module 2:
```sh
cargo run --release -- 2
```
If no valid module number is provided, a help message will be displayed.

## Addendum A
1. **Module 1 – Memory & Access Patterns**
    
2. **Module 2 – Performance Trade-offs**
    
3. **Module 3 – Dynamic Arrays & Amortized Analysis**
    
4. **Module 4 – Core Array Algorithms**
    
5. **Module 5 – Sliding Windows & Prefix-Sum Tricks**
    
6. **Module 6 – Multi-Dimensional Arrays & Cache**
    
7. **Module 7 – Putting It All Together: “Array Toolkit” Project**
    

Each module has
- **Concepts to Read/Watch**
- **Guided Exercises**
- **Mini-Challenge**

### Module 1: Memory & Access Patterns

**Goal:** Understand why `arr[i]` is O(1), how array data sits in RAM, and the basics of pointer arithmetic.

- **Read/Watch**
    - Contiguous memory, row‐ vs column-major (C vs Fortran)
    - Pointer arithmetic in C/C++ (or whatever low-level language you choose)

- **Guided Exercises**
    1. Allocate an array statically and print the addresses of each element.
    2. Write a function `element_at(arr, i)` using pointer arithmetic only (no `[]`).

- **Mini-Challenge:**
    - Given two pointers `p` and `q` into the same array, write code to compute their **index difference** without using indices.


### Module 2: Performance Trade-offs

**Goal:** Quantify array strengths/weaknesses versus linked structures.

- **Read/Watch**
    - O(1) random access vs O(n) insertion/deletion
    - Cache lines, spatial locality

- **Guided Exercises**
    1. Benchmark inserting 1 million ints at front of a static array vs a singly linked list.
    2. Print elapsed times and observe behavior as data size grows.

- **Mini-Challenge:**
    - Using your benchmark harness, plot (or tabulate) time vs n for both structures and explain the “knee” where arrays win.


### Module 3: Dynamic Arrays & Amortized Analysis

**Goal:** Build your own `Vector`/`ArrayList` and see why append is amortized O(1).

- **Read/Watch**
    - Growth strategies: ×2 doubling, golden-ratio, +k increments
    - Amortized cost concept

- **Guided Exercises**
    1. Implement a simple `struct Vector { int *data; size, capacity; }` with `push_back()`.
    2. Instrument it to count element-moves on each `push_back`.

- **Mini-Challenge:**
    - Compare three growth strategies (×2, ×1.5, +1000) by total copies for 1 million pushes—report which is best and why.


### Module 4: Core Array Algorithms

**Goal:** Master in-place array transformations.

- **Read/Watch**
    - Reverse array in-place
    - Rotate (left/right) by k steps
    - Partitioning (e.g. Dutch National Flag)

- **Guided Exercises**
    1. Write in-place `reverse(arr)`.
    2. Write `rotate_right(arr, k)` in O(n) time, O(1) space.
    3. Solve the “move zeros to end” problem in one pass.

- **Mini-Challenge:**
    - Given an array of 0s, 1s, 2s, sort it in a single pass (Dutch National Flag problem).


### Module 5: Sliding Windows & Prefix-Sum Tricks

**Goal:** Use arrays to solve subarray-sum and windowed problems efficiently.

- **Read/Watch**
    - Prefix-sum arrays
    - Two-pointer/sliding-window pattern

- **Guided Exercises**
    1. Compute prefix-sum array and use it to answer any subarray‐sum query in O(1).
    2. Find the maximum sum subarray of size k using a sliding window in O(n).

- **Mini-Challenge:**
    - Given an array of positive ints, find the smallest subarray length whose sum ≥ S in O(n).


### Module 6: Multi-Dimensional Arrays & Cache

**Goal:** Work with 2D/3D arrays and understand cache impacts.

- **Read/Watch**
    - Row-major vs column-major access patterns
    - Loop-order optimizations

- **Guided Exercises**
    1. Multiply two matrices; measure performance with row-major vs column-major inner loops.
    2. Implement a “transpose in-place” for a square matrix.

- **Mini-Challenge:**
    - Given a large 2D grid, implement an efficient “fill” (flood-fill) using your own stack/queue on a flat array.


### Module 7: “Array Toolkit” Capstone Project

**Goal:** Consolidate everything by building a reusable small library and CLI app.

**Project spec:**  
Build a command‐line “array_utils” program in your language of choice that offers:
- `reverse`, `rotate`, `partition` commands
- `benchmark` mode to compare static vs dynamic array ops
- `matrix-mul` with both naive and cache-optimized loops
- `sliding-window` demo for max-sum, min-length tasks

**Steps:**
1. **Project setup:** initialize repo, CLI parsing (e.g. getopt).
2. **Implement core modules:** one feature per module above.
3. **Write automated tests:** ensure correctness (unit tests for every utility).
4. **Add benchmarks & reports:** harness to log timings and print CSV/table.
5. **Documentation & README:** explain concepts, usage examples, and performance insights.