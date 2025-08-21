mod tutorial;
use std::env;
use tutorial::{
    arrays_module1::arrays_module1_memory_access_patterns,
    arrays_module2::arrays_module2_performance_tradeoffs_benchmark,
    arrays_module3::arrays_module3_dynamic_arrays_and_amortized_analysis,
    arrays_module4::arrays_module4_core_array_algorithms,
    arrays_module5::arrays_module5_sliding_windows_and_prefix_sum_tricks,
    arrays_module6::arrays_module6_multi_dimensional_arrays_and_cache,
};

fn main() {
    if let Some(choice) = env::args().nth(1).as_deref() {
        if choice == "*" {
            let valid_choices = ["1", "2", "3", "4", "5", "6"];
            for choice in valid_choices {
                println!("\n=================================================");
                println!("=================================================\n");
                run_choice(choice);
            }
        } else {
            run_choice(choice);
        }
    } else {
        println!(
            "No tutorial module specified or it is invalid. Specify which tutorial module you wish to run by specifying a number following the command i.e. cargo run --release -- 2"
        );
    }
}

fn run_choice(choice: &str) {
    match choice {
        "1" => arrays_module1_memory_access_patterns(),
        "2" => arrays_module2_performance_tradeoffs_benchmark(),
        "3" => arrays_module3_dynamic_arrays_and_amortized_analysis(),
        "4" => arrays_module4_core_array_algorithms(),
        "5" => arrays_module5_sliding_windows_and_prefix_sum_tricks(),
        "6" => arrays_module6_multi_dimensional_arrays_and_cache(),
        _ => {}
    }
}
