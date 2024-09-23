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

use curve25519_dalek::scalar::Scalar;
use ff::PrimeField;
use num_bigint::BigUint;

use super::FieldElement;
use super::RingElement;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Curve25519FieldElement(Scalar);

impl FieldElement for Curve25519FieldElement {}

impl RingElement for Curve25519FieldElement {
    fn name_str() -> &'static str {
        "curve25519"
    }

    fn inv(&self) -> anyhow::Result<Self> {
        Ok(Self(self.0.invert()))
    }

    fn serialize(&self) -> String {
        self.clone().to_string()
    }

    fn deserialize(str: &str) -> Self {
        Self::from_str(str).unwrap()
    }

    fn byte_len() -> usize {
        32
    }

    fn to_bytes_le(&self) -> Vec<u8> {
        self.0.to_bytes().to_vec()
    }

    fn from_bytes_le(bytes: &[u8]) -> Self {
        // 32 is hard coded/typed in the curve25519_dalek library
        const BYTES_SIZE: usize = 32;
        let mut new_bytes: [u8; BYTES_SIZE] = [0; BYTES_SIZE];
        if bytes.len() > BYTES_SIZE {
            panic!("incorrect number of bytes passed to Curve25519FieldElement: expected {BYTES_SIZE} got {}", bytes.len());
        }
        for x in 0..BYTES_SIZE {
            if x < bytes.len() {
                new_bytes[x] = bytes[x];
            }
        }
        Self(Scalar::from_bytes_mod_order(new_bytes))
    }
}

impl Debug for Curve25519FieldElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", BigUint::from_bytes_le(self.0.as_bytes()))
    }
}

impl Display for Curve25519FieldElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", BigUint::from_bytes_le(self.0.as_bytes()))
    }
}

impl FromStr for Curve25519FieldElement {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // The curve25519_dalek implementation of from_str_vartime
        // does not accept leading zeroes. In the other implementations we _do_
        // accept leading zeroes so we sanitize the string here as needed
        let trimmed = s.trim_start_matches('0');
        if trimmed.is_empty() {
            Ok(Self::zero())
        } else {
            Ok(Curve25519FieldElement(
                Scalar::from_str_vartime(trimmed).unwrap(),
            ))
        }
    }
}

impl From<u64> for Curve25519FieldElement {
    fn from(value: u64) -> Self {
        Curve25519FieldElement(Scalar::from(value))
    }
}

impl Add for Curve25519FieldElement {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Curve25519FieldElement(self.0 + other.0)
    }
}

impl Sub for Curve25519FieldElement {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Curve25519FieldElement(self.0 - other.0)
    }
}

impl Mul for Curve25519FieldElement {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Curve25519FieldElement(self.0 * other.0)
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Div for Curve25519FieldElement {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Curve25519FieldElement(self.0 * other.0.invert())
    }
}

impl AddAssign for Curve25519FieldElement {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl MulAssign for Curve25519FieldElement {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

impl SubAssign for Curve25519FieldElement {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl Neg for Curve25519FieldElement {
    type Output = Self;

    fn neg(self) -> Self {
        Curve25519FieldElement(-self.0)
    }
}
