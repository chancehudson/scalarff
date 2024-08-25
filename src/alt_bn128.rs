use ark_bn254::Fr;
use ark_ff::PrimeField;
use ark_std::str::FromStr;

use super::FieldElement;

pub type Bn128FieldElement = Fr;

impl FieldElement for Fr {
    fn name_str() -> &'static str {
        "alt_bn128"
    }

    fn prime() -> num_bigint::BigUint {
        Self::MODULUS.into()
    }

    // why does arkworks serialize 0 to an empty string?
    // why would you do that?
    fn serialize(&self) -> String {
        let s = self.clone().to_string();
        if s.is_empty() {
            "0".to_string()
        } else {
            s
        }
    }

    fn deserialize(str: &str) -> Self {
        Fr::from_str(str).unwrap()
    }
}
