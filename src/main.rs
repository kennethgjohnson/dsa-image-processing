mod tutorial;
use std::env;
use tutorial::module1::arrays_module1_memory_access_patterns;
use tutorial::module2::arrays_module2_performance_tradeoffs_benchmark;

fn main() {
    match env::args().nth(1).as_deref() {
        Some("1") => arrays_module1_memory_access_patterns(),
        Some("2") => arrays_module2_performance_tradeoffs_benchmark(),
        _ => println!("No tutorial module specified."),
    }
}
