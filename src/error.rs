//! Error structs

use crate::common::MIN_BIT_LENGTH;
use rand;
use std::{error, fmt, result};

/// Default result struct
pub type Result = result::Result<num_bigint::BigUint, Error>;

/// Error struct
#[derive(Debug)]
pub enum Error {
    /// Handles when the OS Rng fails to initialize
    OsRngInitialization(rand::Error),
    /// Handles when the bit sizes are too small
    BitLength(usize),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::OsRngInitialization(ref err) => {
                write!(f, "Error initializing OS random number generator: {}", err)
            }
            Error::BitLength(length) => write!(
                f,
                "The given bit length is too small; must be at least {}: {}",
                MIN_BIT_LENGTH, length
            ),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::OsRngInitialization(ref err) => err.description(),
            Error::BitLength(_) => "The given bit length was less than 128",
        }
    }
}

impl From<rand::Error> for Error {
    fn from(err: rand::Error) -> Error {
        Error::OsRngInitialization(err)
    }
}
