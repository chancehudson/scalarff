use scalarff::alt_bn128::Bn128FieldElement;
use scalarff::curve_25519::Curve25519FieldElement;
use scalarff::foi::FoiFieldElement;
use scalarff::quadratic_residues_at;
use scalarff::timing::stat_exec;
use scalarff::timing::summary_exec;
use scalarff::BigUint;
use scalarff::FieldElement;

use colored::Colorize;

fn main() {
    // calculate the next {count} square roots in a field
    // and print them
    let start_at = 360;
    let count = 1000;

    stat_exec(
        &format!("{count} residues in {}", Bn128FieldElement::name_str()),
        &mut || {
            type F = Bn128FieldElement;
            let field_name = F::name_str();
            let residues = print_residues::<F>(start_at, count);
            for (element, low_root, high_root) in &residues {
                println!(
                    "    {}_{} = {} * {}",
                    element.to_string().red().bold(),
                    field_name.to_string().green().bold(),
                    low_root.to_string(),
                    high_root
                );
            }
        },
    );

    stat_exec(
        &format!("{count} residues in {}", Curve25519FieldElement::name_str()),
        &mut || {
            type F = Curve25519FieldElement;
            let field_name = F::name_str();
            let residues = print_residues::<F>(start_at, count);
            for (element, low_root, high_root) in &residues {
                println!(
                    "    {}_{} = {} * {}",
                    element.to_string().red().bold(),
                    field_name.to_string().green().bold(),
                    low_root.to_string(),
                    high_root
                );
            }
        },
    );

    stat_exec(
        &format!("{count} residues in {}", FoiFieldElement::name_str()),
        &mut || {
            type F = Curve25519FieldElement;
            let field_name = F::name_str();
            let residues = print_residues::<F>(start_at, count);
            for (element, low_root, high_root) in &residues {
                println!(
                    "    {}_{} = {} * {}",
                    element.to_string().red().bold(),
                    field_name.to_string().green().bold(),
                    low_root.to_string(),
                    high_root
                );
            }
        },
    );

    summary_exec();
}

fn print_residues<T: FieldElement>(
    start_at: usize,
    count: usize,
) -> Vec<(BigUint, BigUint, BigUint)> {
    let message = format!(
        "finding the next {count} residues in field {}: starting at {start_at}",
        T::name_str()
    )
    .blue()
    .bold();
    println!("{message}",);
    // (element, low_root, high_root)
    let mut out = vec![];
    let residues = quadratic_residues_at::<T>(start_at.into(), count.into());
    out.append(
        &mut residues
            .iter()
            .map(|v| (v.0.to_biguint(), v.1.to_biguint(), v.2.to_biguint()))
            .collect::<Vec<_>>(),
    );
    out
}
