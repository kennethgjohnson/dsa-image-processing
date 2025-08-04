mod tutorial;
use std::env;
use tutorial::{
    arrays_module1::arrays_module1_memory_access_patterns,
    arrays_module2::arrays_module2_performance_tradeoffs_benchmark,
    arrays_module3::arrays_module3_dynamic_arrays_and_amortized_analysis,
    arrays_module4::arrays_module4_core_array_algorithms,
};

fn main() {
    match env::args().nth(1).as_deref() {
        Some("1") => arrays_module1_memory_access_patterns(),
        Some("2") => arrays_module2_performance_tradeoffs_benchmark(),
        Some("3") => arrays_module3_dynamic_arrays_and_amortized_analysis(),
        Some("4") => arrays_module4_core_array_algorithms(),
        _ => println!(
            "No tutorial module specified or it is invalid. Specify which tutorial module you wish to run by specifying a number following the command i.e. cargo run --release -- 2"
        ),
    }
}
