/// Default number of deterministic CPU benchmark iterations.
pub const DEFAULT_ITERATIONS: u64 = 2_000_000;

/// Maximum number of elements used by the memory benchmark.
pub const MAX_MEMORY_ELEMENTS: u64 = 2_000_000;

/// Keep a user-supplied benchmark from accidentally requesting zero work.
pub fn validate_iterations(iterations: u64) -> Result<u64, &'static str> {
    if iterations == 0 {
        Err("iteration count must be greater than zero")
    } else {
        Ok(iterations)
    }
}
