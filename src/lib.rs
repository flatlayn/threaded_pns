use serde::Deserialize;
use std::fs;
use std::io::ErrorKind;
use std::process;

/// Configuration loaded from config.toml
#[derive(Debug, Deserialize)]
pub struct Config {
    pub threads: usize,
    pub max_number: u64,
}

impl Config {
    pub fn load() -> Self {
        Self::load_from_file("config.toml")
    }

    pub fn load_from_file(path: &str) -> Self {
        // Step 1: Read the file
        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(e) => {
                let msg = match e.kind() {
                    ErrorKind::NotFound => {
                        format!("Configuration file '{}' not found.\n\
                                 Create it with:\n\n\
                                 threads = 4\n\
                                 max_number = 1000\n", path)
                    }
                    ErrorKind::PermissionDenied => {
                        format!("Permission denied reading '{}'.\n\
                                 Check file permissions with: ls -la {}", path, path)
                    }
                    _ => format!("Failed to read '{}': {}", path, e),
                };
                eprintln!("Error: {}", msg);
                process::exit(1);
            }
        };

        // Step 2: Parse TOML
        let config: Config = match toml::from_str(&content) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Error: Invalid TOML in '{}':\n  {}\n\n\
                           Expected format:\n\
                           threads = 4\n\
                           max_number = 1000", path, e);
                process::exit(1);
            }
        };

        // Step 3: Validate values
        if config.threads == 0 {
            eprintln!("Error: 'threads' must be at least 1 (got 0)");
            process::exit(1);
        }
        if config.max_number < 2 {
            eprintln!("Error: 'max_number' must be at least 2 (got {})", config.max_number);
            process::exit(1);
        }

        config
    }
}

/// Check if n is prime using trial division (odd divisors only after 2).
pub fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    if n == 2 { return true; }
    if n % 2 == 0 { return false; }

    let limit = (n as f64).sqrt() as u64;
    // Only check odd divisors: 3, 5, 7, ...
    for d in (3..=limit).step_by(2) {
        if n % d == 0 { return false; }
    }
    true
}

/// Check if n is divisible by any number in range [start, end].
pub fn has_divisor_in_range(n: u64, start: u64, end: u64) -> bool {
    for d in start..=end {
        if d > 1 && d < n && n % d == 0 {
            return true;
        }
    }
    false
}

/// Format current timestamp with milliseconds
pub fn timestamp() -> String {
    chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        assert!(!is_prime(0));
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(!is_prime(4));
        assert!(is_prime(5));
        assert!(is_prime(97));
        assert!(!is_prime(100));
    }
}
