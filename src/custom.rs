/// The provided `modulus` should be a number `< 2^64`.
/// This function creates a commutative ring with the provided
/// modulus. This ring may be considered a field if the modulus
/// is prime.
/// Expects `FieldElement` to be in scope
///
/// This macro is intended for testing and educational purposes.
#[macro_export]
macro_rules! custom_ring {
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

            fn serialize(&self) -> String {
                self.0.to_string()
            }

            fn deserialize(str: &str) -> Self {
                $name(str.parse::<u128>().unwrap())
            }

            fn to_bytes_le(&self) -> Vec<u8> {
                self.0.to_le_bytes().to_vec()
            }

            fn from_bytes_le(bytes: &[u8]) -> Self {
                let mut padded_bytes = bytes.to_vec();
                if bytes.len() < 16 {
                    padded_bytes.resize(16, 0);
                }
                $name(u128::from_le_bytes(padded_bytes.try_into().unwrap()))
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl std::str::FromStr for $name {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok($name(s.parse::<u128>().unwrap()))
            }
        }

        impl From<u64> for $name {
            fn from(value: u64) -> Self {
                $name(u128::from(value))
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
    custom_ring!(F13FieldElement, 13_u128, "f13");

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
}
