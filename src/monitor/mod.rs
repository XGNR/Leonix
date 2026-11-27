use std::env;

pub fn print_report() -> Result<(), Box<dyn std::error::Error>> {
    let logical_cores = std::thread::available_parallelism()
        .map(|count| count.get())
        .unwrap_or(1);
    let working_directory = env::current_dir()?;

    println!("\n  🦁 LEONIX SYSTEM REPORT");
    println!("  --------------------------------");
    println!("  Project version : {}", env!("CARGO_PKG_VERSION"));
    println!("  Operating system: {}", env::consts::OS);
    println!("  Architecture    : {}", env::consts::ARCH);
    println!("  Logical cores   : {logical_cores}");
    println!("  Working folder  : {}", working_directory.display());
    println!("  --------------------------------");
    println!("  Memory details  : Not available in the standard-library build");
    println!("  GPU details     : Not available in the standard-library build");
    println!("\n  Tip: run 'leonix bench' to measure local CPU and memory workloads.\n");

    Ok(())
}
