use std::time::Instant;

use super::print_separator;

pub fn stat_exec(f: &mut dyn Fn()) {
    let now = Instant::now();
    f();
    let elapsed = now.elapsed();
    println!("^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^");
    println!(
        "^^^^^^^^^^ function excecuted in {} ms",
        elapsed.as_millis()
    );
    print_separator();
}
