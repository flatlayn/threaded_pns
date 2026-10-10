//! Variant 4: Divisibility Division + Deferred Print (Thread Pool)

use primes::{timestamp, Config};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Arc;
use std::thread::{self, JoinHandle};

struct Job {
    n: u64,
    start: u64,
    end: u64,
    step: u64,
    cancel: Arc<AtomicBool>,
    response_tx: Sender<bool>,
}

const SHUTDOWN: u64 = 0;

fn main() {
    let config = Config::load();
    let num_threads = config.threads;
    let max_num = config.max_number;

    println!("=== Variant 4: Divisibility Division + Deferred Print (Thread Pool) ===");
    println!("Threads: {}, Max: {}", num_threads, max_num);
    println!("[START] {}", timestamp());

    let (job_tx, job_rx): (Sender<Job>, Receiver<Job>) = mpsc::channel();
    let job_rx = Arc::new(std::sync::Mutex::new(job_rx));

    let workers: Vec<JoinHandle<()>> = (0..num_threads)
        .map(|_| {
            let rx = Arc::clone(&job_rx);
            thread::spawn(move || worker_loop(rx))
        })
        .collect();

    let mut primes: Vec<u64> = Vec::new();

    for n in 2..=max_num {
        if is_prime_pooled(n, num_threads, &job_tx) {
            primes.push(n);
        }
    }

    // Shutdown
    for _ in 0..num_threads {
        let (tx, _) = mpsc::channel();
        let _ = job_tx.send(Job {
            n: SHUTDOWN, start: 0, end: 0, step: 1,
            cancel: Arc::new(AtomicBool::new(false)), response_tx: tx,
        });
    }
    for w in workers { let _ = w.join(); }

    println!("\nPrimes found ({} total):", primes.len());
    for p in &primes {
        println!("  {}", p);
    }

    println!("\n[END] {}", timestamp());
}

fn worker_loop(job_rx: Arc<std::sync::Mutex<Receiver<Job>>>) {
    loop {
        let job = {
            let rx = job_rx.lock().unwrap();
            rx.recv()
        };

        match job {
            Ok(job) if job.n == SHUTDOWN => return,
            Ok(job) => {
                let mut found = false;
                let mut d = job.start;
                while d <= job.end {
                    if d % 64 == 0 && job.cancel.load(Ordering::Relaxed) {
                        break;
                    }
                    if job.n % d == 0 {
                        found = true;
                        job.cancel.store(true, Ordering::Relaxed);
                        break;
                    }
                    d += job.step;
                }
                let _ = job.response_tx.send(found);
            }
            Err(_) => return,
        }
    }
}

fn is_prime_pooled(n: u64, num_threads: usize, job_tx: &Sender<Job>) -> bool {
    if n < 2 { return false; }
    if n == 2 { return true; }
    if n % 2 == 0 { return false; }

    let limit = (n as f64).sqrt() as u64;

    // Small: check directly with odd divisors
    if limit < 20 {
        return !(3..=limit).step_by(2).any(|d| n % d == 0);
    }

    let (response_tx, response_rx) = mpsc::channel();
    let cancel = Arc::new(AtomicBool::new(false));

    // Odd divisors: 3, 5, 7, ...
    let first_odd = 3u64;
    let last_odd = if limit % 2 == 0 { limit - 1 } else { limit };
    let odd_count = (last_odd - first_odd) / 2 + 1;

    let chunk_size = odd_count / num_threads as u64;
    let remainder = odd_count % num_threads as u64;
    let mut jobs_sent = 0;

    for tid in 0..num_threads {
        let offset = tid as u64 * chunk_size + (tid as u64).min(remainder);
        let extra = if (tid as u64) < remainder { 1 } else { 0 };
        let count = chunk_size + extra;

        if count == 0 { break; }

        let start = first_odd + offset * 2;
        let end = start + (count - 1) * 2;

        if start > last_odd { break; }

        let _ = job_tx.send(Job {
            n,
            start,
            end: end.min(last_odd),
            step: 2,
            cancel: Arc::clone(&cancel),
            response_tx: response_tx.clone(),
        });
        jobs_sent += 1;
    }

    drop(response_tx);

    for _ in 0..jobs_sent {
        if response_rx.recv() == Ok(true) {
            return false;
        }
    }
    true
}
