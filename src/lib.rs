use std::iter::Iterator;

// Some sample "good" values
const MODULUS: u64 = 1 << 32;
const MULTIPLIER: u64 = 1103515245;
const INCREMENT: u64 = 12345;

/// Linear Congruential Generator
pub struct Lcg {
    ///// Range in which the random number is generated
    //pub range: Range<T>,
    /// Modulus value (m > 0)
    modulus: u64,
    /// Multiplier value (0 < a < m)
    multiplier: u64,
    /// Increment value (0 <= c < m)
    increment: u64,
    /// Seed value, i.e. the state of the generator (0 <= seed < m)
    seed: u64,
}

impl Lcg {
    #[inline]
    pub fn new(modulus: u64, multiplier: u64, increment: u64, seed: u64) -> Self {
        Self {
            modulus,
            multiplier,
            increment,
            seed,
        }
    }
}

impl Iterator for Lcg {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        self.seed = (self.multiplier * self.seed + self.increment) % self.modulus;
        Some(self.seed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let lcg = Lcg::new(MODULUS, MULTIPLIER, INCREMENT, 7777);

        let states: Vec<u64> = lcg.take(10).collect();
        println!("states: {:?}", states);
        assert!(false);
    }
}
