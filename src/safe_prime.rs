//! Generates cryptographically secure safe prime numbers.

use rand_core::OsRng;

use crate::common::MIN_BIT_LENGTH;
pub use crate::common::{
    gen_safe_prime as from_rng, is_safe_prime as check_with, is_safe_prime_baillie_psw as strong_check_with,
};
use crate::error::{Error, Result};

/// Constructs a new safe prime number with a size of `bit_length` bits.
///
/// This will initialize an `OsRng` instance and call the
/// `from_rng()` function.
///
/// Note: the `bit_length` MUST be at least 128-bits.
pub fn new(bit_length: usize) -> Result {
    if bit_length < MIN_BIT_LENGTH {
        Err(Error::BitLength(bit_length))
    } else {
        let mut rng = OsRng::default();
        Ok(from_rng(bit_length, &mut rng)?)
    }
}

/// Checks if number is a safe prime
pub fn check(candidate: &num_bigint::BigUint) -> bool {
    check_with(candidate, &mut OsRng::default())
}

/// Checks if number is a safe prime using the Baillie-PSW test
pub fn strong_check(candidate: &num_bigint::BigUint) -> bool {
    strong_check_with(candidate, &mut OsRng::default())
}

#[cfg(test)]
mod tests {
    use super::{check, new, strong_check};

    #[test]
    fn tests() {
        for bits in &[128, 256, 384] {
            let n = new(*bits).unwrap();
            assert!(check(&n));
            assert!(strong_check(&n));
        }
    }
}
