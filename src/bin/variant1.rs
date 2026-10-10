//! Variant 1: Range Division + Immediate Print

use primes::{is_prime, timestamp, Config};
use std::thread;

fn main() {
    // Load configuration (exits with clear error if config is invalid)
    let config = Config::load();
    let num_threads = config.threads;
    let max_num = config.max_number;

    println!("=== Variant 1: Range Division + Immediate Print ===");
    println!("Threads: {}, Max: {}", num_threads, max_num);
    println!("[START] {}", timestamp());

    // Use scoped threads so we can share references without Arc
    thread::scope(|s| {
        let range_size = max_num - 1;
        let chunk_size = range_size / num_threads as u64;
        let remainder = range_size % num_threads as u64;

        for thread_id in 0..num_threads {
            let start = 2 + thread_id as u64 * chunk_size
                + (thread_id as u64).min(remainder);
            let extra = if (thread_id as u64) < remainder { 1 } else { 0 };
            let end = start + chunk_size - 1 + extra;

            if start > max_num {
                continue;
            }
            let end = end.min(max_num);

            s.spawn(move || {
                for n in start..=end {
                    if is_prime(n) {
                        println!(
                            "[Thread {}] Found prime: {} at {}",
                            thread_id,
                            n,
                            timestamp()
                        );
                    }
                }
            });
        }
    });

    println!("[END] {}", timestamp());
}
