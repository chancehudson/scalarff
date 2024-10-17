//! A minimal, opinionated, library for working with scalar finite fields.
//!
//! Curated scalar finite field implementations from the best cryptography libraries.
//! Provides a `FieldElement` trait for working with residues, and a `to_biguint`
//! method for arbitrary precision operations on the real representations of field elements.
//!
//! This library makes no guarantees about the timing of underlying field operations. **This
//! library should be considered vulnerable to timing attacks.**
//!
//! By default this library does not include any field implementations. Manually
//! enable support for fields by enabling the corresponding feature below:
//!   - `alt_bn128` - (aka Bn254)
//!   - `curve25519`
//!   - `oxfoi` - (aka goldilocks)
//!
//! Example usage:
//! ```toml
//! [dependencies]
//! scalarff = { version = "0.4.1", features = ["curve25519", "oxfoi"] }
//! ```
//!
//! ```rust
//! use scalarff::FieldElement; // Bring the trait in scope
//! // Import 1 or more concrete instances
//! use scalarff::Curve25519FieldElement;
//! use scalarff::FoiFieldElement;
//! ```
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

#[macro_use]
mod custom;

#[cfg(feature = "alt_bn128")]
pub mod alt_bn128;
#[cfg(feature = "curve25519")]
pub mod curve_25519;
#[cfg(all(feature = "oxfoi", target_pointer_width = "64"))]
pub mod oxfoi;
#[cfg(feature = "oxfoi")]
pub mod oxfoi_slow;

pub mod matrix;
pub mod timing;

#[cfg(feature = "alt_bn128")]
pub use alt_bn128::Bn128FieldElement;
#[cfg(feature = "curve25519")]
pub use curve_25519::Curve25519FieldElement;
pub use num_bigint::BigUint;
#[cfg(all(feature = "oxfoi", target_pointer_width = "64"))]
pub use oxfoi::OxfoiFieldElement;
#[cfg(all(feature = "oxfoi", not(target_pointer_width = "64")))]
pub use oxfoi_slow::OxfoiFieldElement;

/// A generic representation of a scalar finite field element.
/// For use in internal module logic. Supports field operations
/// using builtin operators (*-+/) and other convenience traits.
/// Handles serialization and deserialization to a reasonable
/// string representation.
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
    + Clone
    + Hash
    + Debug
    + From<u64>
    + Display
{
    /// Get the zero element.
    fn zero() -> Self {
        Self::from(0)
    }

    /// Get the one element.
    fn one() -> Self {
        Self::from(1)
    }

    /// Minimum number of bytes needed to represent
    /// an element.
    fn byte_len() -> usize;

    /// Sample a random element from the field using a supplied
    /// source of randomness. Requires the `random` feature to be enabled.
    #[cfg(feature = "random")]
    fn sample_uniform<R: rand::Rng>(src: &mut R) -> Self {
        let bytes = vec![0; Self::byte_len()]
            .iter()
            .map(|_| src.gen_range(0..=255))
            .collect::<Vec<_>>();
        Self::from_bytes_le(&bytes)
    }

    /// Get a valid string representation
    /// of the element.
    fn serialize(&self) -> String;

    /// Parse an element from a supposedly
    /// valid string representation.
    fn deserialize(str: &str) -> Self;

    /// The prime modulus of the field as an
    /// arbitrary precision integer.
    fn prime() -> BigUint {
        // this is a generic implementation.
        // Concrete instances may provide
        // a better/faster implementation
        (-Self::one()).to_biguint() + 1_u32
    }

    /// A short string identifier for the field.
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
        // BigUint::from_str(&self.serialize()).unwrap()
        BigUint::from_bytes_le(self.to_bytes_le().as_slice())
    }

    /// Convert a `num_bigint::BigUint` into a field element
    /// precision operations. Numbers will be converted % self.prime()
    fn from_biguint(v: &BigUint) -> Self {
        Self::from_bytes_le(&v.clone().to_bytes_le()[..])
    }

    /// Parse an element from a byte representation. Panics
    /// if the byte representation is too long. e.g. if the bytes
    /// represent a value > Self::prime().
    fn from_bytes_le(bytes: &[u8]) -> Self;

    /// Convert a field element to a byte representation.
    /// The number of bytes may be variable, but is guaranteed
    /// to be accepted by `from_bytes_le` for the same curve.
    fn to_bytes_le(&self) -> Vec<u8>;

    /// A string representation of a field element using
    /// only the lower 60 bits of the element. A normal
    /// decimal representation will be given if it's shorter
    /// than the lower 60 bit representation.
    /// This is a lossy representation.
    fn lower60_string(&self) -> String {
        const POW: u32 = 60;
        // careful here, if POW is >= 64 we will overflow
        // the u64 below
        let two_pow = BigUint::from(/*here ->*/ 2_u64.pow(POW));
        let plain_str = self.serialize();
        let l60_str = format!("{}_L60", self.to_biguint() % two_pow);
        // add a couple characters so we always print
        // 0xfoi elements as decimal strings
        if l60_str.len() + 3 < plain_str.len() {
            l60_str
        } else {
            plain_str
        }
    }

    /// Take a logarithm using a custom base and return the
    /// floored value. `O(logb(n))` time complexity where `n`
    /// is the size of the element.
    fn log_floor(&self, b: Self) -> u32 {
        if b.to_biguint() > self.to_biguint() {
            return 0;
        } else if b == *self {
            return 1;
        }
        let e = self.to_biguint();
        let b = b.to_biguint();
        let mut x = b.clone();
        let mut i = 1;
        while x < e {
            x *= b.clone();
            if x >= e {
                return i;
            }
            i += 1;
        }
        unreachable!();
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
        let root = (a_ * b_) % Self::prime();
        let other_root = Self::prime() - root.clone();
        if root > other_root {
            Self::from_biguint(&other_root)
        } else {
            Self::from_biguint(&root)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_sqrt<T: FieldElement>() {
        let mut x = T::one();
        for _ in 0..1000 {
            let square = x.clone() * x.clone();
            let root = square.sqrt();
            assert_eq!(square, root.clone() * root.clone());
            x += T::one();
        }
    }

    scalar_ring!(F13FieldElement, 13, "f13");

    #[test]
    fn sqrt_scalar_ring() {
        test_sqrt::<F13FieldElement>();
    }

    #[test]
    fn sqrt_foi_slow() {
        test_sqrt::<oxfoi_slow::OxfoiFieldElement>();
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn sqrt_foi() {
        test_sqrt::<oxfoi::OxfoiFieldElement>();
    }

    #[test]
    fn sqrt_bn128() {
        test_sqrt::<alt_bn128::Bn128FieldElement>();
    }

    #[test]
    fn sqrt_curve25519() {
        test_sqrt::<curve_25519::Curve25519FieldElement>();
    }
}
