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

use ark_bn254::Fr;
use ark_ff::biginteger::BigInt;
use ark_ff::BigInteger;
use ark_ff::PrimeField;
use ark_std::str::FromStr;
use num_bigint::BigUint;

use super::FieldElement;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Bn128FieldElement(Fr);

#[cfg(feature = "serde")]
impl serde::Serialize for Bn128FieldElement {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_bytes(&self.to_bytes_le())
    }
}

#[cfg(feature = "serde")]
impl<'a> serde::Deserialize<'a> for Bn128FieldElement {
    fn deserialize<S: serde::Deserializer<'a>>(serializer: S) -> Result<Self, S::Error> {
        let bytes = <Vec<u8>>::deserialize(serializer)?;
        Ok(Self::from_bytes_le(&bytes))
    }
}

impl FieldElement for Bn128FieldElement {
    fn name_str() -> &'static str {
        "alt_bn128"
    }

    fn prime() -> num_bigint::BigUint {
        Fr::MODULUS.into()
    }

    fn byte_len() -> usize {
        32
    }

    fn to_bytes_le(&self) -> Vec<u8> {
        const LIMBS: usize = 4;
        let v: BigInt<LIMBS> = self.0.into_bigint();
        if v < BigInt::zero() {
            panic!("arkworks returned a negative value in byte serialization");
        }
        v.to_bytes_le()
    }

    fn from_bytes_le(bytes: &[u8]) -> Self {
        Self(Fr::from_str(&BigUint::from_bytes_le(bytes).to_string()).unwrap())
    }
}

impl Debug for Bn128FieldElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Display for Bn128FieldElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.0.clone().to_string();
        if s.is_empty() {
            write!(f, "0")
        } else {
            write!(f, "{}", s)
        }
    }
}

impl FromStr for Bn128FieldElement {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Bn128FieldElement(Fr::from_str(s).unwrap()))
    }
}

impl From<u64> for Bn128FieldElement {
    fn from(value: u64) -> Self {
        Bn128FieldElement(Fr::from(value))
    }
}

impl Add for Bn128FieldElement {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Bn128FieldElement(self.0 + other.0)
    }
}

impl Sub for Bn128FieldElement {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Bn128FieldElement(self.0 - other.0)
    }
}

impl Mul for Bn128FieldElement {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Bn128FieldElement(self.0 * other.0)
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Div for Bn128FieldElement {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Bn128FieldElement(self.0 / other.0)
    }
}

impl AddAssign for Bn128FieldElement {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl MulAssign for Bn128FieldElement {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

impl SubAssign for Bn128FieldElement {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl Neg for Bn128FieldElement {
    type Output = Self;

    fn neg(self) -> Self {
        Bn128FieldElement(-self.0)
    }
}
