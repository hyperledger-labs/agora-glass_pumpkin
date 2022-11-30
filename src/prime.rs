//! Generates cryptographically secure prime numbers.

use crypto_bigint::UInt;
use rand_core::OsRng;

pub use crate::common::{
    gen_prime as from_rng, is_prime as check_with, is_prime_baillie_psw as strong_check_with,
};
use crate::error::Result;

/// Constructs a new prime number with a size of `bit_length` bits.
///
/// This will initialize an `OsRng` instance and call the
/// `from_rng()` function.
///
/// Note: the `bit_length` MUST be at least 128-bits.
pub fn new<const L: usize>(bit_length: usize) -> Result<L> {
    let mut rng = OsRng::default();
    from_rng::<L, _>(bit_length, &mut rng)
}

/// Test if number is prime by
///
/// 1- Trial division by first 2048 primes
/// 2- Perform a Fermat Test
/// 3- Perform log2(bitlength) + 5 rounds of Miller-Rabin
///    depending on the number of bits
pub fn check<const L: usize>(candidate: &UInt<L>) -> bool {
    check_with(candidate, &mut OsRng::default())
}

/// Checks if number is a prime using the Baillie-PSW test
pub fn strong_check<const L: usize>(candidate: &UInt<L>) -> bool {
    strong_check_with(candidate, &mut OsRng::default())
}

#[cfg(test)]
mod tests {
    use super::{check, new, strong_check};

    #[test]
    fn tests() {
        tests_impl::<2>(128);
        tests_impl::<4>(256);
        tests_impl::<8>(512);
        tests_impl::<16>(1024);
    }

    fn tests_impl<const L: usize>(bit_length: usize) {
        let n = new::<L>(bit_length).unwrap();
        assert!(check(&n));
        assert!(strong_check(&n));
    }
}
