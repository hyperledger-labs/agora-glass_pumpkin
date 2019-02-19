//! Generates cryptographically secure safe prime numbers.

use rand::rngs::OsRng;

pub use crate::common::{gen_safe_prime as from_rng, is_safe_prime as check, is_safe_prime_baillie_psw as strong_check};
use crate::common::MIN_BIT_LENGTH;
use crate::error::{Error, Result};

/// Constructs a new safe prime number with a size of `bit_length` bits.
///
/// This will initialize an `OsRng` instance and call the
/// `from_rng()` function.
///
/// Note: the `bit_length` MUST be at least 512-bits.
pub fn new(bit_length: usize) -> Result {
    if bit_length < MIN_BIT_LENGTH {
        Err(Error::BitLength(bit_length))
    } else {
        let mut rng = OsRng::new()?;
        Ok(from_rng(bit_length, &mut rng)?)
    }
}

#[cfg(test)]
mod tests {
    use super::{new, check, strong_check};

    #[test]
    fn tests() {
        for bits in [128, 256, 384].iter() {
            let n = new(*bits).unwrap();
            assert!(check(&n));
            assert!(strong_check(&n));
        }
    }
}
