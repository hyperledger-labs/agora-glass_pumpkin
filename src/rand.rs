use crypto_bigint::{NonZero, RandomMod, UInt};
use rand_core::{CryptoRng, RngCore};

/// Iterator to generate a given amount of random numbers. For convenience of
/// use with miller_rabin tests, you can also append a specified number at the
/// end of the generated stream.
pub(crate) struct Randoms<R, const L: usize> {
    appended: Option<UInt<L>>,
    lower_limit: UInt<L>,
    range: NonZero<UInt<L>>,
    amount: usize,
    rng: R,
}

impl<R: CryptoRng + RngCore, const L: usize> Randoms<R, L> {
    pub(crate) fn new(lower_limit: &UInt<L>, upper_limit: &UInt<L>, amount: usize, rng: R) -> Self {
        // TODO: fail if upper_limit <= lower_limit?
        let range = upper_limit.wrapping_sub(lower_limit);
        let range_nonzero = NonZero::new(range).unwrap();
        Self {
            appended: None,
            lower_limit: *lower_limit,
            range: range_nonzero,
            amount,
            rng,
        }
    }

    /// Append the number at the end to appear as if it was generated. This
    /// doesn't affect stream length. Only one number can be appended,
    /// subsequent calls will replace the previously appended number.
    pub(crate) fn with_appended(mut self, x: UInt<L>) -> Self {
        self.appended = Some(x);
        self
    }

    fn gen_biguint(&mut self) -> UInt<L> {
        let random = UInt::<L>::random_mod(&mut self.rng, &self.range);
        random.wrapping_add(&self.lower_limit)
    }
}

impl<R: CryptoRng + RngCore, const L: usize> Iterator for Randoms<R, L> {
    type Item = UInt<L>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.amount == 0 {
            None
        } else if self.amount == 1 {
            let r = match core::mem::replace(&mut self.appended, None) {
                Some(x) => x,
                None => self.gen_biguint(),
            };
            self.amount -= 1;
            Some(r)
        } else {
            self.amount -= 1;
            Some(self.gen_biguint())
        }
    }
}

#[cfg(test)]
mod test {
    use super::Randoms;
    use crypto_bigint::U256;
    use rand::thread_rng;

    #[test]
    fn generate_amount_test() {
        let amount = 3;
        let rands = Randoms::new(&U256::ZERO, &U256::ONE, amount, thread_rng());
        let generated = rands.collect::<Vec<_>>();
        assert_eq!(generated.len(), amount);

        let rands = Randoms::new(&U256::ZERO, &U256::ONE, amount, thread_rng())
            .with_appended(U256::from(2u32));
        let generated = rands.collect::<Vec<_>>();
        assert_eq!(generated.len(), amount);
    }
}
