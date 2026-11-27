use std::error::Error;

use crate::benchmarks;
use crate::config;
use crate::tools;

pub fn run(iterations: u64) -> Result<(), Box<dyn Error>> {
    let iterations = config::validate_iterations(iterations)?;
    let memory_elements = iterations.min(config::MAX_MEMORY_ELEMENTS);

    println!("\n  ⚡ LEONIX PERFORMANCE LAB");
    println!("  --------------------------------");
    println!("  CPU workload : {} iterations", tools::format_count(iterations));
    println!("  Memory test  : {} elements", tools::format_count(memory_elements));
    println!("  --------------------------------");

    let cpu = benchmarks::run_cpu(iterations);
    tools::print_result(&cpu);

    let memory = benchmarks::run_memory(memory_elements);
    tools::print_result(&memory);

    println!("\n  Research note: results depend on hardware, power mode,");
    println!("  background tasks, compiler settings, and temperature.\n");

    Ok(())
}
