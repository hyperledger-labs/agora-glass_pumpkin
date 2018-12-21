#![deny(missing_docs, unsafe_code, unused_import_braces, unused_qualifications, trivial_casts, trivial_numeric_casts)]
//! A crate for generating large prime numbers, suitable for cryptography.
//! extern crate glass_pumpkin;
//!
//! use glass_pumpkin::prime;
//!
//! fn main() {
//!     let p = prime::new(1024);
//!     let q = prime::new(1024);
//!
//!     let n = p * q;
//!
//!     println!("{}", n);
//! }
//!
//! You can also supply `OsRng` and generate primes from that.
//!
//! extern crate glass_pumpkin;
//! extern crate rand;
//!
//! use glass_pumpkin::prime;
//!
//! use rand::rngs::OsRng;
//! use rand::thread_rng;
//!
//! fn main() {
//!     let mut rng = OsRng::new().unwrap();
//!     let p = prime::from_rng(1024, &mut rng);
//!     let q = prime::from_rng(1024, &mut rng);
//!
//!     let n = p * q;
//!     println!("{}", n);
//! }
//!
//! `Primes` are generated similarly to OpenSSL except it applies some recommendations from the (Prime and Prejudice)[https://eprint.iacr.org/2018/749.pdf]
//!
//! 1. Generate a random odd number of a given bit-length.
//! 2. Divide the candidate by the first 2048 prime numbers
//! 3. Test the candidate with Fermat's Theorem.
//! 4. Runs log2(bits) Miller-Rabin tests.

extern crate num_bigint;
extern crate num_traits;
extern crate num_integer;
extern crate int_traits;
extern crate rand;

mod common;
pub mod error;
pub mod prime;
pub mod safe_prime;
