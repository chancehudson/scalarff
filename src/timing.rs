//! A simple module for timing functions and printing information.
//!
use std::sync::RwLock;
use std::time::Duration;
use std::time::Instant;

use colored::Colorize;

pub fn print_separator() {
    println!("||||||||||||||||||||||||||||||||||||||||");
}

static TRANSCRIPT: RwLock<Vec<(String, Duration)>> = RwLock::new(vec![]);

/// Execute a closure and print+store information about the
/// execution. Closure should return a string that will be used
/// to identify the closure in a summary (see `summary_exec`).
pub fn stat_exec(f: &mut dyn Fn() -> String) {
    let now = Instant::now();
    let name = f();
    let elapsed = now.elapsed();
    let time_str = format!("{} ms", elapsed.as_millis()).bold().italic();
    println!(
        "{}",
        format!("^^^^^^^^^^ function executed in {time_str} ^^^^^^^^^^",).green()
    );
    print_separator();

    let mut transcript = TRANSCRIPT.write().unwrap();
    transcript.push((name.to_string(), elapsed));
}

/// Prints a summary of all `stat_exec` invocations.
/// Call this just before the program exits to show a timing
/// summary.
pub fn summary_exec() {
    let transcript = TRANSCRIPT.read().unwrap();
    for (name, elapsed) in &*transcript {
        let time_str = format!("{} ms", elapsed.as_millis()).bold().italic();
        println!(
            "{}",
            format!("{} executed in {time_str}", name.bold()).green()
        );
    }
}
