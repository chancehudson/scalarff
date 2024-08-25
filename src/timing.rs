use std::time::Instant;

use colored::Colorize;

use super::print_separator;

pub fn stat_exec(f: &mut dyn Fn()) {
    let now = Instant::now();
    f();
    let elapsed = now.elapsed();
    let time_str = format!("{} ms", elapsed.as_millis()).bold().italic();
    println!(
        "{}",
        format!("^^^^^^^^^^ function excecuted in {time_str} ^^^^^^^^^^",).green()
    );
    print_separator();
}
