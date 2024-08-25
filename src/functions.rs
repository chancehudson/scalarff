use super::FieldElement;

/// Find the next `count` positive quadratic residues starting from element `start`
/// IDEA: find the _nearest_ quadratic residues. e.g. search in both directions: positive and negative
pub fn quadratic_residues_at<T: FieldElement>(start: usize, count: usize) -> Vec<(T, T, T)> {
    let mut out = Vec::new();
    let mut x = start;
    while out.len() < count {
        let element = T::from_usize(x);
        match element.legendre() {
            -1 => {
                // number is a non-residue (no roots in field)
            }
            1 => {
                // number is a residue
                // return number and roots
                let low_root = element.sqrt();
                let high_root = -low_root.clone();

                assert_eq!(element, low_root.clone() * low_root.clone());
                assert_eq!(element, high_root.clone() * high_root.clone());
                assert_eq!(-element.clone(), low_root.clone() * high_root.clone());

                out.push((element, low_root, high_root));
            }
            0 => {
                // number is 0, skip
            }
            _ => unreachable!(),
        }
        x += 1;
    }
    out
}
