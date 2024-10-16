use colored::Colorize;

use scalarff::timing::stat_exec;
use scalarff::timing::summary_exec;
use scalarff::Bn128FieldElement;
use scalarff::Curve25519FieldElement;
use scalarff::FieldElement;
use scalarff::OxfoiFieldElement;

fn main() {
    // calculate the next {count} square roots in a field
    // and print them
    let start_at = 360;
    let count = 1000;

    stat_exec(&mut || {
        type T = Bn128FieldElement;
        print_residues::<T>(start_at, count);
        format!("{count} quadratic residues in {}", T::name_str())
    });
    stat_exec(&mut || {
        type T = Curve25519FieldElement;
        print_residues::<T>(start_at, count);
        format!("{count} quadratic residues in {}", T::name_str())
    });
    stat_exec(&mut || {
        type T = OxfoiFieldElement;
        print_residues::<T>(start_at, count);
        format!("{count} quadratic residues in {}", T::name_str())
    });

    summary_exec();
}

/// Find the next `count` positive quadratic residues starting from element `start_at`
/// IDEA: find the _nearest_ quadratic residues. e.g. search in both directions: positive and negative
fn print_residues<T: FieldElement>(start_at: usize, count: usize) {
    let field_name = T::name_str();
    let message = format!(
        "finding the next {count} residues in field {}: starting at {start_at}",
        field_name
    )
    .blue()
    .bold();
    println!("{message}",);

    let mut found_count = 0;
    let mut x = start_at;
    while found_count < count {
        let element = T::from_usize(x);
        match element.legendre() {
            1 => {
                // number is a residue
                // return number and roots
                let low_root = element.sqrt();
                let high_root = -low_root.clone();

                assert_eq!(element, low_root.clone() * low_root.clone());
                assert_eq!(element, high_root.clone() * high_root.clone());
                assert_eq!(-element.clone(), low_root.clone() * high_root.clone());

                println!(
                    "    -{}_{} = {} * {}",
                    element.lower60_string().red().bold(),
                    T::name_str().green().bold(),
                    low_root.lower60_string(),
                    high_root.lower60_string(),
                );
                found_count += 1;
            }
            -1 => {
                // number is a non-residue (no roots in field)
            }
            0 => {
                // number is 0, skip
            }
            _ => unreachable!(),
        }
        x += 1;
    }
}
