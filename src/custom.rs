/// The provided `modulus` should be a number `< 2^64`.
/// This function creates a commutative ring with the provided
/// modulus. This ring may be considered a field if the modulus
/// is prime.
/// Expects `FieldElement` to be in scope
///
/// This macro is intended for testing and educational purposes.
#[macro_export]
macro_rules! scalar_ring {
    ( $name: ident, $modulus: literal, $name_str: expr ) => {
        /// An element in a ring with a custom modulus
        /// this modulus must be < 2^64 so we can do modular
        /// multiplication using the u128 type.
        #[derive(std::fmt::Debug, Clone, Copy, PartialEq, Eq, std::hash::Hash)]
        pub struct $name(u128);

        impl FieldElement for $name {
            fn name_str() -> &'static str {
                $name_str
            }

            fn zero() -> Self {
                $name(0)
            }

            fn one() -> Self {
                $name(1)
            }

            fn byte_len() -> usize {
                // add 1 because ilog2 rounds down
                // this causes a worst case of 1 extra byte
                //
                // number of bits in the modulus
                let mod_bits = ($modulus as u128).ilog2() + 1;

                // add 1 here as well because floored division rounds down
                // this causes another worst case +1 byte
                (usize::try_from(mod_bits).unwrap() / 8) + 1
            }

            fn to_bytes_le(&self) -> Vec<u8> {
                #[cfg(debug_assertions)]
                {
                    let bytes = self.0.to_le_bytes();
                    for i in bytes.iter().skip(Self::byte_len()) {
                        assert_eq!(*i, 0, "Scalar value is too large for modulus");
                    }
                }
                self.0.to_le_bytes()[0..Self::byte_len()].to_vec()
            }

            fn from_bytes_le(bytes: &[u8]) -> Self {
                let mut padded_bytes = bytes.to_vec();
                if bytes.len() < 16 {
                    padded_bytes.resize(16, 0);
                }
                $name(u128::from_le_bytes(padded_bytes.try_into().unwrap()) % $modulus)
            }
        }

        #[cfg(feature = "serde")]
        impl serde::Serialize for $name {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.serialize_bytes(&self.to_bytes_le())
            }
        }

        #[cfg(feature = "serde")]
        impl<'a> serde::Deserialize<'a> for $name {
            fn deserialize<S: serde::Deserializer<'a>>(serializer: S) -> Result<Self, S::Error> {
                let bytes = <Vec<u8>>::deserialize(serializer)?;
                Ok(Self::from_bytes_le(&bytes))
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl std::str::FromStr for $name {
            type Err = anyhow::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok($name(s.parse::<u128>()? % $modulus))
            }
        }

        impl From<u64> for $name {
            fn from(value: u64) -> Self {
                $name(u128::from(value) % $modulus)
            }
        }

        impl std::ops::Add for $name {
            type Output = Self;

            fn add(self, other: Self) -> Self {
                $name((self.0 + other.0) % $modulus)
            }
        }

        impl std::ops::Sub for $name {
            type Output = Self;

            fn sub(self, other: Self) -> Self {
                $name((self.0 + $modulus - other.0) % $modulus)
            }
        }

        impl std::ops::Mul for $name {
            type Output = Self;

            fn mul(self, other: Self) -> Self {
                $name((self.0 * other.0) % $modulus)
            }
        }

        impl std::ops::Div for $name {
            type Output = Self;

            fn div(self, other: Self) -> Self {
                let other_inv = other.to_biguint().modinv(&Self::prime());
                if let Some(inv) = other_inv {
                    $name((self.0 * u128::try_from(inv).unwrap()) % $modulus)
                } else {
                    panic!("Division by zero");
                }
            }
        }

        impl std::ops::AddAssign for $name {
            fn add_assign(&mut self, other: Self) {
                *self = *self + other;
            }
        }

        impl std::ops::MulAssign for $name {
            fn mul_assign(&mut self, other: Self) {
                *self = *self * other;
            }
        }

        impl std::ops::SubAssign for $name {
            fn sub_assign(&mut self, other: Self) {
                *self = *self - other;
            }
        }

        impl std::ops::Neg for $name {
            type Output = Self;

            fn neg(self) -> Self {
                $name(($modulus - self.0) % $modulus)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::FieldElement;

    // define a field element in f13 (finite field with 13 elements)
    // do some tests on it
    scalar_ring!(F13FieldElement, 13_u128, "f13");
    scalar_ring!(BabyBearElement, 2013265921_u128, "babybear");

    #[test]
    fn str_name() {
        assert_eq!(F13FieldElement::name_str(), "f13");
    }

    #[test]
    fn mul_add_ops() {
        let x = F13FieldElement(7);
        assert_eq!(x * x, F13FieldElement(10));
        for x in 0..13 {
            let x_e = F13FieldElement(x);
            assert_eq!(x_e * x_e, F13FieldElement((x * x) % 13));
            assert_eq!(x_e + x_e, F13FieldElement((x + x) % 13));
        }
    }

    #[test]
    fn serialization() {
        assert_eq!(BabyBearElement::byte_len(), 4);
        let x = -BabyBearElement::one();
        let bytes = x.to_bytes_le();
        assert_eq!(bytes.len(), BabyBearElement::byte_len());
        let y = BabyBearElement::from_bytes_le(&bytes);
        assert_eq!(x, y);
        // test the zero element
        assert_eq!(
            BabyBearElement::zero().to_bytes_le().len(),
            BabyBearElement::byte_len()
        );
        for v in BabyBearElement::zero().to_bytes_le() {
            assert_eq!(v, 0);
        }
        // test some random elements
        #[cfg(feature = "random")]
        for _ in 0..1000 {
            let x = BabyBearElement::sample_uniform(&mut rand::thread_rng());
            let bytes = x.to_bytes_le();
            assert_eq!(bytes.len(), BabyBearElement::byte_len());
            let y = BabyBearElement::from_bytes_le(&bytes);
            assert_eq!(x, y);
        }
    }
}
