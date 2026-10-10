//! Variant 2: Range Division + Deferred Print

use primes::{is_prime, timestamp, Config};
use std::thread;

fn main() {
    let config = Config::load();
    let num_threads = config.threads;
    let max_num = config.max_number;

    println!("=== Variant 2: Range Division + Deferred Print ===");
    println!("Threads: {}, Max: {}", num_threads, max_num);
    println!("[START] {}", timestamp());

    // Collect all primes from all threads
    let mut all_primes: Vec<u64> = thread::scope(|s| {
        let range_size = max_num - 1;
        let chunk_size = range_size / num_threads as u64;
        let remainder = range_size % num_threads as u64;

        let handles: Vec<_> = (0..num_threads)
            .filter_map(|thread_id| {
                let start = 2 + thread_id as u64 * chunk_size
                    + (thread_id as u64).min(remainder);
                let extra = if (thread_id as u64) < remainder { 1 } else { 0 };
                let end = start + chunk_size - 1 + extra;

                if start > max_num {
                    return None;
                }
                let end = end.min(max_num);

                Some(s.spawn(move || {
                    // Collect primes in this thread's range
                    let mut primes = Vec::new();
                    for n in start..=end {
                        if is_prime(n) {
                            primes.push(n);
                        }
                    }
                    primes
                }))
            })
            .collect();

        // Collect results from all threads
        handles
            .into_iter()
            .flat_map(|h| h.join().unwrap())
            .collect()
    });

    // Sort for clean output (threads may return in any order)
    all_primes.sort_unstable();

    // Deferred print: all at once
    println!("\nPrimes found ({} total):", all_primes.len());
    for p in &all_primes {
        println!("  {}", p);
    }

    println!("\n[END] {}", timestamp());
}
