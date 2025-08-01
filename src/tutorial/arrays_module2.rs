// Array Module2 - Performance tradeoffs benchmark
use std::time::{Duration, Instant};
// Static Array simulation, we'll use Vec but pre-allocate
struct ArrayBench {
    data: Vec<i32>,
    capacity: usize,
    size: usize,
}

// Simple singly linked list
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

struct LinkedList {
    head: Option<Box<Node>>,
    size: usize,
}

pub fn arrays_module2_performance_tradeoffs_benchmark() {
    let mut stack_size = 8 * 1024 * 1024;
    if cfg!(debug_assertions) {
        stack_size *= 3;
        println!(">> Consider running with --release build, lists are about 1.5 times");
        println!("    faster, and the system requires about a 3rd of the stack memory");
    }
    std::thread::Builder::new()
        // It needs 64MB to do up to a milion.
        .stack_size(stack_size)
        .spawn(|| {
            arrays_module2_performance_tradeoffs_benchmark_run();
        })
        .unwrap()
        .join()
        .unwrap();
}

fn arrays_module2_performance_tradeoffs_benchmark_run() {
    println!("Front insert performance tradeoffs of array vs linked lists, showing the knee...");
    let arr = ArrayBench {
        data: vec![0],
        capacity: 1,
        size: 1,
    };
    let node = Node {
        value: 1,
        next: None,
    };
    if let Some(next) = node.next {
        println!("Some next node was found: {}", next.value);
    }
    let _record = format!(
        "Node Value: {}, Array Capacity: {}",
        node.value, arr.capacity
    );
    println!("Warming up: {:?}\n", _record);
    let _start = Instant::now();

    println!("Data Size | Array Time (µs) | LinkedList Time (µs) | Ratio");
    println!("----------|-----------------|----------------------|------");
    let mut is_knee_found = false;
    for size in [
        100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 2000, 3000, 4000, 5000, 6000, 7000,
        8000, 9000, 10_000, 20_000, 30_000, 40_000, 50_000, 60_000, 70_000, 80_000, 90_000,
        100_000,
    ] {
        let start = Instant::now();
        create_array(size);
        let arr_time = start.elapsed();
        let start = Instant::now();
        create_linked_list(size);
        let lst_time = start.elapsed();
        print_output_result(size, arr_time, lst_time, &mut is_knee_found);
    }
}

fn create_array(element_count: usize) -> ArrayBench {
    let mut arr = ArrayBench {
        data: Vec::with_capacity(element_count),
        capacity: element_count,
        size: 0,
    };

    for i in 1..=(element_count as i32) {
        arr.data.insert(0, i);
        arr.size += 1;
    }
    arr
}

fn create_linked_list(element_count: usize) -> LinkedList {
    let mut list = LinkedList {
        head: None,
        size: 0,
    };

    for i in 1..=(element_count as i32) {
        let new_node = Node {
            value: i,
            next: list.head,
        };
        list.head = Some(Box::new(new_node));
        list.size += 1;
    }
    list
}

fn print_output_result(
    element_count: usize,
    arr_time: Duration,
    lst_time: Duration,
    is_knee_found: &mut bool,
) {
    let array_uq = arr_time.as_micros() as f64;
    let linkedlist_uq = lst_time.as_micros() as f64;
    let ratio = array_uq / linkedlist_uq;
    let mut knee = "";
    if ratio > 1.0 && !*is_knee_found {
        knee = " <-- knee";
        *is_knee_found = true;
    }
    println!(
        "{:<9} | {:>15.3} | {:>20.3} | {:>6.1}x{}",
        element_count, array_uq, linkedlist_uq, ratio, knee
    );
}
