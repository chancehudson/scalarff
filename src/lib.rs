//! A minimal, opinionated, library for working with scalar finite fields.
//! Curated scalar finite field implementations from the best cryptography libraries.
//! Provides a `FieldElement` trait for working with residues, and a `to_biguint`
//! method for arbitrary precision operations.
//!
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

use num_integer::Integer;

pub mod alt_bn128;
pub mod curve_25519;
pub mod foi;
mod functions;
pub mod matrix;
mod timing;

pub use functions::quadratic_residues_at;
pub use num_bigint::BigUint;
pub use timing::stat_exec;

pub fn print_separator() {
    println!("||||||||||||||||||||||||||||||||||||||||");
}

pub trait FieldElement:
    Add<Output = Self>
    + AddAssign
    + Div<Output = Self>
    + Mul<Output = Self>
    + MulAssign
    + Neg<Output = Self>
    + Sub<Output = Self>
    + SubAssign
    + FromStr
    + PartialEq
    + PartialOrd
    + Clone
    + Hash
    + Debug
    + From<u64>
    + Display
{
    fn zero() -> Self {
        Self::from(0)
    }

    fn one() -> Self {
        Self::from(1)
    }

    /// Get a valid string representation
    /// of the element.
    fn serialize(&self) -> String;

    /// Parse an element from a supposedly
    /// valid string representation.
    fn deserialize(str: &str) -> Self;

    /// The prime modulus of the field as an
    /// arbitrary precision integer.
    fn prime() -> BigUint;
    fn name_str() -> &'static str;

    /// Parse an element from a usize
    /// throws if the field size is smaller than
    /// the usize on the machine.
    fn from_usize(value: usize) -> Self {
        // usize -> u64 conversion only fails
        // on >64 bit systems, e.g. a 128 bit
        // computer
        Self::from(u64::try_from(value).unwrap())
    }

    /// Get a `num_bigint::BigUint` representation for arbitrary
    /// precision operations.
    fn to_biguint(&self) -> num_bigint::BigUint {
        // todo: use bytes
        BigUint::from_str(&self.serialize()).unwrap()
    }

    /// Calculate the [legendre symbol](https://en.wikipedia.org/wiki/Legendre_symbol#Definition)
    /// for a field element. Used to determine if the
    /// element is a quadratic residue.
    fn legendre(&self) -> i32 {
        if self == &Self::zero() {
            return 0;
        }
        let neg_one = Self::prime() - 1_u32;
        let one = BigUint::from(1_u32);
        let e = (-Self::one()) / (Self::one() + Self::one());
        let e_bigint = BigUint::from_str(&e.serialize()).unwrap();
        let a = BigUint::from_str(&self.serialize()).unwrap();
        let l = a.modpow(&e_bigint, &Self::prime());
        if l == neg_one {
            -1
        } else if l == one {
            return 1;
        } else {
            panic!("legendre symbol is not 1, -1, or 0");
        }
    }

    /// [Kumar 08](https://arxiv.org/pdf/2008.11814v4) prime field square root implementation.
    /// Always returns the smaller root e.g. the positive root.
    fn sqrt(&self) -> Self {
        if self == &Self::zero() {
            return Self::zero();
        }
        if self.legendre() != 1 {
            panic!("legendre symbol is not 1: root does not exist or input is 0");
        }
        // find a non-residue
        let mut x = Self::one() + Self::one();
        let non_residue;
        loop {
            if x.legendre() == -1 {
                non_residue = x.clone();
                break;
            }
            x += Self::one();
        }
        let b = BigUint::from_str(&non_residue.serialize()).unwrap();

        let a = BigUint::from_str(&self.serialize()).unwrap();
        let two = Self::one() + Self::one();
        let m = (-Self::one()) / two.clone();
        let mut apow = -Self::one();
        let mut bpow = Self::zero();
        while BigUint::from_str(&apow.serialize()).unwrap().is_even() {
            apow = apow / two.clone();
            bpow = bpow / two.clone();
            let a_ = a.modpow(
                &BigUint::from_str(&apow.serialize()).unwrap(),
                &Self::prime(),
            );
            let b_ = b.modpow(
                &BigUint::from_str(&bpow.serialize()).unwrap(),
                &Self::prime(),
            );
            if (a_ * b_) % Self::prime() == Self::prime() - 1_u32 {
                bpow += m.clone();
            }
        }
        apow = (apow + Self::one()) / two.clone();
        bpow = bpow / two;
        let a_ = a.modpow(
            &BigUint::from_str(&apow.serialize()).unwrap(),
            &Self::prime(),
        );
        let b_ = b.modpow(
            &BigUint::from_str(&bpow.serialize()).unwrap(),
            &Self::prime(),
        );
        let o = (a_ * b_) % Self::prime();
        let root = Self::deserialize(&o.to_string());
        let other_root = -root.clone();
        if root > other_root {
            other_root
        } else {
            root
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alt_bn128::Bn128FieldElement;
    use curve_25519::Curve25519FieldElement;
    use foi::FoiFieldElement;

    fn test_sqrt<T: FieldElement>() {
        let mut x = T::one();
        for _ in 0..1000 {
            let square = x.clone() * x.clone();
            let root = square.sqrt();
            assert_eq!(square, root.clone() * root.clone());
            x += T::one();
        }
    }

    #[test]
    fn sqrt_foi() {
        test_sqrt::<FoiFieldElement>();
    }

    #[test]
    fn sqrt_bn128() {
        test_sqrt::<Bn128FieldElement>();
    }

    #[test]
    fn sqrt_curve25519() {
        test_sqrt::<Curve25519FieldElement>();
    }
}
