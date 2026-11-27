use std::error::Error;
use std::env;

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut args = env::args().skip(1);

    match args.next().as_deref() {
        Some("info") => crate::monitor::print_report(),
        Some("bench") => {
            let iterations = match args.next() {
                Some(value) => value.parse::<u64>()?,
                None => crate::config::DEFAULT_ITERATIONS,
            };
            crate::performance::run(iterations)
        }
        Some("help") | None => {
            print_help();
            Ok(())
        }
        Some(command) => Err(format!("Unknown command: {command}. Run 'leonix help'.").into()),
    }
}

fn print_help() {
    println!(
        r#"
  🦁 LEONIX — XGNR SYSTEM TOOLKIT

  USAGE:
      leonix <COMMAND>

  COMMANDS:
      info              Show system and runtime information
      bench [COUNT]     Run CPU and memory micro-benchmarks
      help              Show this help screen

  EXAMPLES:
      leonix info
      leonix bench
      leonix bench 10000000

  Notes:
      Benchmarks are local experiments, not gaming FPS predictions.
"#
    );
}
