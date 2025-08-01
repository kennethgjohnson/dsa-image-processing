use std::alloc::{Layout, alloc, dealloc, realloc};
use std::{
    ops::{Index, IndexMut},
    time::{Duration, Instant},
};
struct MyVector<T> {
    data: *mut T,    // Pointer to start of allocated capacity.
    size: usize,     // Current number of slots filled.
    capacity: usize, // Current number of slots available.
}

// Implement Drop trait so that memory is freed if the program crashes.
impl<T> Drop for MyVector<T> {
    fn drop(&mut self) {
        if !self.data.is_null() {
            let layout = self.make_memory_layout(self.capacity);
            unsafe {
                dealloc(self.data as *mut u8, layout); // Note: need to always case to u8 when deallocating.
            }
        }
    }
}

// read‑only indexing: &mylist[i] -> &T
impl<T> Index<usize> for MyVector<T> {
    type Output = T;

    fn index(&self, idx: usize) -> &T {
        self.validate_index(idx);
        unsafe { &*self.data.add(idx) }
    }
}

impl<T> MyVector<T> {
    fn validate_index(&self, idx: usize) {
        if self.size <= idx {
            panic!(
                "index out of bounds: the len is {} but the index is {}",
                self.size, idx
            );
        }
    }

    pub fn new() -> Self {
        Self {
            data: std::ptr::null_mut(),
            size: 0,
            capacity: 0,
        }
    }

    fn make_memory_layout(&self, capacity: usize) -> Layout {
        let element_size = size_of::<T>();
        Layout::from_size_align(capacity * element_size, align_of::<T>()).unwrap()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let mut new_vec = MyVector::new();
        new_vec.resize_capacity(capacity);
        new_vec
    }

    fn resize_capacity(&mut self, capacity: usize) {
        if self.data.is_null() {
            // first allocation
            unsafe {
                self.data = alloc(self.make_memory_layout(capacity)).cast::<T>();
            }
        } else {
            // resize to bigger memory
            let current_layout = self.make_memory_layout(self.capacity);
            unsafe {
                self.data = realloc(
                    self.data.cast::<u8>(),
                    current_layout,
                    capacity * size_of::<T>(), // Is size in u8 so needs to be multiplied
                )
                .cast::<T>();
            }
        }
        self.capacity = capacity;
    }

    pub fn len(&self) -> usize {
        self.size
    }

    // Increases Capacity by 1000 when full
    pub fn push_fixed(&mut self, element: T) {
        if self.capacity == self.size {
            // Add Capacity
            self.resize_capacity(self.capacity + 1000);
        }
        self.push_raw(element);
    }

    fn push_raw(&mut self, element: T) {
        unsafe {
            *self.data.add(self.size) = element;
        }
        self.size += 1;
    }

    // Increases size by golden ratio, except its a aproximation of 1.5 since
    // integer arithmatic is supose to be faster... meh.
    pub fn push_golden(&mut self, element: T) {
        if self.capacity == self.size {
            // Add Capacity
            let mut new_cap = self.capacity.max(1).saturating_mul(3).saturating_div(2);
            if new_cap <= self.capacity {
                new_cap = self.capacity + 1;
            }
            self.resize_capacity(new_cap);
        }
        self.push_raw(element);
    }

    // Doubling capacity with each grow needed.
    pub fn push_doubleing(&mut self, element: T) {
        if self.capacity == self.size {
            // Add Capacity
            self.resize_capacity(self.capacity.max(1).saturating_mul(2) as usize);
        }
        self.push_raw(element);
    }
}

// mutable indexing: &mut mylist[i] -> &mut T
impl<T> IndexMut<usize> for MyVector<T> {
    fn index_mut(&mut self, idx: usize) -> &mut T {
        self.validate_index(idx);
        unsafe { &mut *self.data.add(idx) }
    }
}

pub fn arrays_module3_dynamic_arrays_and_amortized_analysis() {
    let _start = Instant::now(); // warming up the timer.
    println!("==>Testing push_fixed");
    let mut my_vector = MyVector::new();
    my_vector.push_fixed(8);
    my_vector.push_fixed(9);
    my_vector.push_fixed(10);

    println!(
        "The values in my_vector are {}, {}, {}",
        my_vector[0], my_vector[1], my_vector[2]
    );
    println!("There are {} elements.", my_vector.len());
    println!();
    let mut my_vector = MyVector::with_capacity(200);
    my_vector.push_fixed(12);
    println!(
        "New vector capacity 200 with 1 element size is {} elements.",
        my_vector.len()
    );

    println!("==>Testing push_golden");
    let mut my_vector = MyVector::new();
    my_vector.push_golden(8);
    my_vector.push_golden(9);
    my_vector.push_golden(10);

    println!(
        "The values in my_vector are {}, {}, {}",
        my_vector[0], my_vector[1], my_vector[2]
    );
    println!("There are {} elements.", my_vector.len());
    println!();
    let mut my_vector = MyVector::with_capacity(200);
    my_vector.push_golden(12);
    println!(
        "New vector capacity 200 with 1 element size is {} elements.",
        my_vector.len()
    );

    println!("==>Testing push_doubling");
    let mut my_vector = MyVector::new();
    my_vector.push_doubleing(8);
    my_vector.push_doubleing(9);
    my_vector.push_doubleing(10);

    println!(
        "The values in my_vector are {}, {}, {}",
        my_vector[0], my_vector[1], my_vector[2]
    );
    println!("There are {} elements.", my_vector.len());
    println!();
    let mut my_vector = MyVector::with_capacity(200);
    my_vector.push_doubleing(12);
    println!(
        "New vector capacity 200 with 1 element size is {} elements.",
        my_vector.len()
    );

    print!("\n\n");
    println!("==>Begining capturing performance info...");

    println!("Data Size |   Fixed (µs) |  Golden (µs) | Doubling (µs) | Fastest F/G/D ");
    println!("----------|--------------|--------------|---------------|---------------");

    for size in [
        1000, 2000, 3000, 4000, 5000, 6000, 7000, 8000, 9000, 10_000, 20_000, 30_000, 40_000,
        50_000, 60_000, 70_000, 80_000, 90_000, 100_000, 200_000, 300_000, 400_000, 500_000,
        600_000, 700_000, 800_000, 900_000, 1_000_000, 2_000_000, 3_000_000, 4_000_000, 5_000_000,
        6_000_000, 7_000_000, 8_000_000, 9_000_000, 10_000_000,
    ] {
        let start = Instant::now();
        create_my_vec_push_fixed(size);
        let fixed_time = start.elapsed();
        let start = Instant::now();
        create_my_vec_push_golden(size);
        let golden_time = start.elapsed();
        let start = Instant::now();
        create_my_vec_push_doubleing(size);
        let doubling_time = start.elapsed();
        print_three_results(size, fixed_time, golden_time, doubling_time);
    }
}

fn create_my_vec_push_fixed(element_count: usize) -> MyVector<i32> {
    let mut my_vec = MyVector::new(); // We not using with capacity so we can test the growth speed.

    for i in 1..=(element_count as i32) {
        my_vec.push_fixed(i);
    }
    my_vec
}

fn create_my_vec_push_golden(element_count: usize) -> MyVector<i32> {
    let mut my_vec = MyVector::new(); // We not using with capacity so we can test the growth speed.

    for i in 1..=(element_count as i32) {
        my_vec.push_golden(i);
    }
    my_vec
}

fn create_my_vec_push_doubleing(element_count: usize) -> MyVector<i32> {
    let mut my_vec = MyVector::new(); // We not using with capacity so we can test the growth speed.

    for i in 1..=(element_count as i32) {
        my_vec.push_doubleing(i);
    }
    my_vec
}

fn print_three_results(n: usize, f: Duration, g: Duration, d: Duration) {
    let f_us = f.as_micros() as f64;
    let g_us = g.as_micros() as f64;
    let d_us = d.as_micros() as f64;

    // figure out the minimum
    let fastest = if f_us <= g_us && f_us <= d_us {
        "F"
    } else if g_us <= f_us && g_us <= d_us {
        "G"
    } else {
        "D"
    };

    println!(
        "{:<9} | {:>12.1} | {:>12.1} | {:>12.1}  |   {}",
        n, f_us, g_us, d_us, fastest
    );
}
