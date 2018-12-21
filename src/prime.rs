//! Generates cryptographically secure prime numbers.

use rand::rngs::OsRng;

pub use crate::common::gen_prime as from_rng;
pub use crate::common::is_prime as check;
use crate::common::MIN_BIT_LENGTH;
use crate::error::{Error, Result};

/// Constructs a new prime number with a size of `bit_length` bits.
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
