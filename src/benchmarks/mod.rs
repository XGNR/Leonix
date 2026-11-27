use std::hint::black_box;
use std::time::{Duration, Instant};

pub struct BenchmarkResult {
    pub label: &'static str,
    pub elapsed: Duration,
    pub operations: u64,
    pub checksum: u64,
}

pub fn run_cpu(iterations: u64) -> BenchmarkResult {
    let started = Instant::now();
    let mut value = 0x9E37_79B9_7F4A_7C15_u64;

    for _ in 0..iterations {
        value ^= value << 13;
        value ^= value >> 7;
        value ^= value << 17;
        value = value.wrapping_add(0xA076_1D64_78BD_642F);
    }

    let checksum = black_box(value);
    BenchmarkResult {
        label: "CPU integer workload",
        elapsed: started.elapsed(),
        operations: iterations,
        checksum,
    }
}

pub fn run_memory(elements: u64) -> BenchmarkResult {
    let started = Instant::now();
    let count = elements as usize;
    let mut values = vec![0_u64; count];

    for (index, value) in values.iter_mut().enumerate() {
        *value = (index as u64)
            .wrapping_mul(0x9E37_79B9)
            .rotate_left((index % 63) as u32);
    }

    let checksum = black_box(values.iter().fold(0_u64, |sum, value| sum.wrapping_add(*value)));

    BenchmarkResult {
        label: "Memory fill and scan",
        elapsed: started.elapsed(),
        operations: elements,
        checksum,
    }
}
