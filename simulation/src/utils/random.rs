use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

/// Seeded random number generator for reproducible games
pub struct SeededRandom {
    rng: StdRng,
}

impl SeededRandom {
    /// Create a new seeded RNG
    pub fn new(seed: u64) -> Self {
        Self {
            rng: StdRng::seed_from_u64(seed),
        }
    }

    /// Generate a random u32
    pub fn next_u32(&mut self) -> u32 {
        self.rng.gen()
    }

    /// Generate a random u64
    pub fn next_u64(&mut self) -> u64 {
        self.rng.gen()
    }

    /// Generate a random number in range [0, max)
    pub fn next_range(&mut self, max: usize) -> usize {
        self.rng.gen_range(0..max)
    }

    /// Shuffle a slice in place
    pub fn shuffle<T>(&mut self, slice: &mut [T]) {
        for i in (1..slice.len()).rev() {
            let j = self.rng.gen_range(0..=i);
            slice.swap(i, j);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seeded_random() {
        let mut rng1 = SeededRandom::new(12345);
        let mut rng2 = SeededRandom::new(12345);
        
        // Same seed should produce same sequence
        assert_eq!(rng1.next_u32(), rng2.next_u32());
        assert_eq!(rng1.next_u32(), rng2.next_u32());
    }
}

