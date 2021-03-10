use std::iter::Iterator;
use std::ops::Range;

// NOTE In the end I didn't make it generic over the integers because I would have
// needed the external `num` crate and require T: num::Integer. It would have
// made this example too complicated, so I thought this will do for now.

/// Linear Congruential Generator
pub struct Lcg {
    /// Minimum value
    min: i64,
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
    /// Generates a new instance of [`Lcg`] if a suitable range is provided for
    /// random number generation.
    ///
    /// Panics if the input types don't match.
    pub fn new(
        range: Range<i64>,
        multiplier: u64,
        increment: u64,
        seed: u64,
    ) -> Result<Self, String> {
        if range.is_empty() {
            Err("Invalid range was given!".to_string())
        } else if range.end.checked_sub(range.start).is_none() {
            // if the range doesn't fit into an i64 it won't fit into an u64 either
            // this is kinda ugly and could be resolved by using a 128 bit number to store the multiplier
            Err("Too large range was given!".to_string())
        } else {
            Ok(Self {
                min: range.start,
                modulus: (range.end - range.start) as u64, // we know that we can safely cast now and end > start
                multiplier,
                increment,
                seed,
            })
        }
    }
}

impl Iterator for Lcg {
    type Item = i64;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.seed = self
            .multiplier
            .wrapping_mul(self.seed)
            .wrapping_add(self.increment)
            % self.modulus;
        // NOTE here, instead of just casting via `as`, I could use TryFrom and
        // handle the possible error, but for this example this might do.
        Some(self.seed as i64 + self.min)
    }
}

#[cfg(test)]
mod tests {
    use super::Lcg;

    #[test]
    fn check_range() {
        let range = -500..150;
        let mut lcg = Lcg::new(range.clone(), u64::MAX, 1432, 5).unwrap();
        for _ in 0..5000 {
            assert!(range.contains(&lcg.next().unwrap()))
        }

        let range = 1..200;
        let mut lcg = Lcg::new(range.clone(), 542342, 344, 4335).unwrap();
        for _ in 0..5000 {
            assert!(range.contains(&lcg.next().unwrap()))
        }

        let range = 1000..9999;
        let mut lcg = Lcg::new(range.clone(), 1103515245, 12345, 4343).unwrap();
        for _ in 0..5000 {
            assert!(range.contains(&lcg.next().unwrap()))
        }
    }

    #[test]
    fn bad_ranges() {
        // too large range
        let range = i64::MIN..i64::MAX;
        assert!(Lcg::new(range, 1, 2, 3).is_err());

        // empty range
        let range = 0..0;
        assert!(Lcg::new(range, 1, 2, 3).is_err());

        // end > start, so range will be empty
        let range = 3242..-555;
        assert!(Lcg::new(range, 1, 2, 3).is_err());
    }
}
