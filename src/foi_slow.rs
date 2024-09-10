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

use super::FieldElement;

#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
pub struct FoiFieldElement(u128);

impl FoiFieldElement {
    pub fn prime_u128() -> u128 {
        18446744069414584321
    }
}

impl FieldElement for FoiFieldElement {
    fn name_str() -> &'static str {
        "oxfoi"
    }

    fn prime() -> num_bigint::BigUint {
        num_bigint::BigUint::from(Self::prime_u128())
    }

    fn serialize(&self) -> String {
        self.0.to_string()
    }

    fn deserialize(str: &str) -> Self {
        Self(str.parse::<u128>().unwrap())
    }

    fn to_bytes_le(&self) -> Vec<u8> {
        self.0.to_le_bytes().to_vec()
    }

    fn from_bytes_le(bytes: &[u8]) -> Self {
        const BYTES_SIZE: usize = 16;
        let mut sized_bytes = [0_u8; BYTES_SIZE];
        if bytes.len() > BYTES_SIZE {
            panic!("incorrect number of bytes passed to foi_slow::FoiFieldElement: expected at most {BYTES_SIZE} got {}", bytes.len());
        }
        for x in 0..BYTES_SIZE {
            if x < bytes.len() {
                sized_bytes[x] = bytes[x];
            }
        }
        Self(u128::from_le_bytes(sized_bytes))
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
        Ok(FoiFieldElement(s.parse::<u128>().unwrap()))
    }
}

impl From<u64> for FoiFieldElement {
    fn from(value: u64) -> Self {
        FoiFieldElement(u128::from(value))
    }
}

impl Add for FoiFieldElement {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        FoiFieldElement((self.0 + other.0) % Self::prime_u128())
    }
}

impl Sub for FoiFieldElement {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        FoiFieldElement((self.0 + Self::prime_u128() - other.0) % Self::prime_u128())
    }
}

impl Mul for FoiFieldElement {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        FoiFieldElement((self.0 * other.0) % Self::prime_u128())
    }
}

impl Div for FoiFieldElement {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let other_inv = other.to_biguint().modinv(&Self::prime());
        if let Some(inv) = other_inv {
            FoiFieldElement((self.0 * u128::try_from(inv).unwrap()) % Self::prime_u128())
        } else {
            panic!("Division by zero");
        }
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
        FoiFieldElement((Self::prime_u128() - self.0) % Self::prime_u128())
    }
}
