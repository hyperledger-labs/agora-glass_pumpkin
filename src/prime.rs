//! Generates cryptographically secure prime numbers.

use rand_core::OsRng;

use crate::common::MIN_BIT_LENGTH;
pub use crate::common::{
    gen_prime as from_rng, is_prime, is_prime_baillie_psw,
};
use crate::error::{Error, Result};

/// Constructs a new prime number with a size of `bit_length` bits.
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

/// Test if number is prime by
///
/// 1- Trial division by first 2048 primes
/// 2- Perform a Fermat Test
/// 3- Perform log2(bitlength) + 5 rounds of Miller-Rabin
///    depending on the number of bits
pub fn check(candidate: &num_bigint::BigUint) -> bool {
    is_prime(candidate, &mut OsRng::default())
}

/// Checks if number is a prime using the Baillie-PSW test
pub fn strong_check(candidate: &num_bigint::BigUint) -> bool {
    is_prime_baillie_psw(candidate, &mut OsRng::default())
}

#[cfg(test)]
mod tests {
    use super::{check, new, strong_check};

    #[test]
    fn tests() {
        for bits in &[128, 256, 512, 1024] {
            let n = new(*bits).unwrap();
            assert!(check(&n));
            assert!(strong_check(&n));
        }
    }
}
