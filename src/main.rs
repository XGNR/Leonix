mod benchmarks;
mod config;
mod core;
mod monitor;
mod performance;
mod tools;

fn main() {
    if let Err(error) = core::run() {
        eprintln!("\n[LEONIX ERROR] {error}");
        std::process::exit(1);
    }
}
