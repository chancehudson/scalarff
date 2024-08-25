use scalarff::alt_bn128::Bn128FieldElement;
use scalarff::curve_25519::Curve25519FieldElement;
use scalarff::foi::FoiFieldElement;
use scalarff::quadratic_residues_at;
use scalarff::stat_exec;
use scalarff::BigUint;
use scalarff::FieldElement;

use colored::Colorize;

fn main() {
    // calculate the next {count} square roots in a field
    // and print them
    let start_at = 360;
    let count = 10;

    stat_exec(&mut || {
        type F = Bn128FieldElement;
        let field_name = F::name_str();
        let residues = print_residues::<F>(start_at, count);
        for (element, low_root, high_root) in &residues {
            println!(
                "    {} -{} = {} * {}",
                field_name.to_string().green().bold(),
                element.to_string().red().bold(),
                low_root.to_string(),
                high_root
            );
        }
    });

    stat_exec(&mut || {
        type F = Curve25519FieldElement;
        let field_name = F::name_str();
        let residues = print_residues::<F>(start_at, count);
        for (element, low_root, high_root) in &residues {
            println!(
                "    {} -{} = {} * {}",
                field_name.to_string().green().bold(),
                element.to_string().red().bold(),
                low_root.to_string(),
                high_root
            );
        }
    });

    stat_exec(&mut || {
        type F = FoiFieldElement;
        let field_name = F::name_str();
        let residues = print_residues::<F>(start_at, count);
        for (element, low_root, high_root) in &residues {
            println!(
                "    {} -{} = {} * {}",
                field_name.to_string().green().bold(),
                element.to_string().red().bold(),
                low_root.to_string(),
                high_root
            );
        }
    });
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
