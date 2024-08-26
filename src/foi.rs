use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Neg;
use std::ops::Sub;
use std::ops::SubAssign;
use std::str::FromStr;

use twenty_first::math::b_field_element::BFieldElement;

use super::FieldElement;

#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
pub struct FoiFieldElement(BFieldElement);

impl FieldElement for FoiFieldElement {
    fn name_str() -> &'static str {
        "oxfoi"
    }

    fn prime() -> num_bigint::BigUint {
        num_bigint::BigUint::from(BFieldElement::P)
    }

    fn serialize(&self) -> String {
        self.0.value().to_string()
    }

    fn deserialize(str: &str) -> Self {
        Self(BFieldElement::from_str(str).unwrap())
    }

    fn to_bytes_le(&self) -> Vec<u8> {
        self.0.value().to_le_bytes().to_vec()
    }

    fn from_bytes_le(bytes: &[u8]) -> Self {
        const BYTES_SIZE: usize = 8;
        let mut sized_bytes = [0_u8; BYTES_SIZE];
        if bytes.len() > BYTES_SIZE {
            panic!("incorrect number of bytes passed to Curve25519FieldElement: expected {BYTES_SIZE} got {}", bytes.len());
        }
        for x in 0..BYTES_SIZE {
            if x < bytes.len() {
                sized_bytes[x] = bytes[x];
            }
        }
        Self(BFieldElement::from(u64::from_le_bytes(sized_bytes)))
    }
}

impl Display for FoiFieldElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for FoiFieldElement {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(FoiFieldElement(BFieldElement::from_str(s).unwrap()))
    }
}

impl From<u64> for FoiFieldElement {
    fn from(value: u64) -> Self {
        FoiFieldElement(BFieldElement::from(value))
    }
}

impl Add for FoiFieldElement {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        FoiFieldElement(self.0 + other.0)
    }
}

impl Sub for FoiFieldElement {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        FoiFieldElement(self.0 - other.0)
    }
}

impl Mul for FoiFieldElement {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        FoiFieldElement(self.0 * other.0)
    }
}

impl Div for FoiFieldElement {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        FoiFieldElement(self.0 / other.0)
    }
}

impl AddAssign for FoiFieldElement {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl MulAssign for FoiFieldElement {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

impl SubAssign for FoiFieldElement {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl Neg for FoiFieldElement {
    type Output = Self;

    fn neg(self) -> Self {
        FoiFieldElement(-self.0)
    }
}
