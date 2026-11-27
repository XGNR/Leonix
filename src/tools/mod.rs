use crate::benchmarks::BenchmarkResult;

pub fn format_count(value: u64) -> String {
    let digits = value.to_string();
    let mut output = String::with_capacity(digits.len() + digits.len() / 3);
    for (index, character) in digits.chars().enumerate() {
        if index > 0 && (digits.len() - index) % 3 == 0 {
            output.push(',');
        }
        output.push(character);
    }
    output
}

pub fn print_result(result: &BenchmarkResult) {
    let seconds = result.elapsed.as_secs_f64();
    let rate = if seconds > 0.0 {
        result.operations as f64 / seconds
    } else {
        0.0
    };

    println!("\n  ▶ {}", result.label);
    println!("    Time     : {:.3} ms", result.elapsed.as_secs_f64() * 1_000.0);
    println!("    Work     : {}", format_count(result.operations));
    println!("    Rate     : {:.0} operations/sec", rate);
    println!("    Checksum : 0x{:016X}", result.checksum);
}
