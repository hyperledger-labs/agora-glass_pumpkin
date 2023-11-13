use num_bigint::{BigUint, RandBigInt};
use rand_core::RngCore;

/// Iterator to generate a given amount of random numbers. For convenience of
/// use with miller_rabin tests, you can also append a specified number at the
/// end of the generated stream.
pub struct Randoms<R> {
    appended: Option<BigUint>,
    lower_limit: BigUint,
    upper_limit: BigUint,
    amount: usize,
    rng: R,
}

impl<R: RngCore> Randoms<R> {
    pub fn new(lower_limit: BigUint, upper_limit: BigUint, amount: usize, rng: R) -> Self {
        Self {
            appended: None,
            lower_limit,
            upper_limit,
            amount,
            rng,
        }
    }

    /// Append the number at the end to appear as if it was generated. This
    /// doesn't affect stream length. Only one number can be appended,
    /// subsequent calls will replace the previously appended number.
    pub fn with_appended(mut self, x: BigUint) -> Self {
        self.appended = Some(x);
        self
    }

    fn gen_biguint(&mut self) -> BigUint {
        self.rng
            .gen_biguint_range(&self.lower_limit, &self.upper_limit)
    }
}

impl<R: RngCore> Iterator for Randoms<R> {
    type Item = BigUint;

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
    use rand::thread_rng;

    #[test]
    fn generate_amount_test() {
        let amount = 3;
        let rands = Randoms::new(0_u8.into(), 1_u8.into(), amount, thread_rng());
        let generated = rands.collect::<Vec<_>>();
        assert_eq!(generated.len(), amount);

        let rands =
            Randoms::new(0_u8.into(), 1_u8.into(), amount, thread_rng()).with_appended(2_u8.into());
        let generated = rands.collect::<Vec<_>>();
        assert_eq!(generated.len(), amount);
    }
}
