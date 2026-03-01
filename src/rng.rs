use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

pub struct SeededRng {
    rng: StdRng,
}

impl SeededRng {
    pub fn new(seed: u64) -> Self {
        Self {
            rng: StdRng::seed_from_u64(seed),
        }
    }

    pub fn rng(&mut self) -> &mut StdRng {
        &mut self.rng
    }

    pub fn gen_range<T>(&mut self, range: std::ops::RangeInclusive<T>) -> T
    where
        T: rand::distr::uniform::SampleUniform + Copy,
    {
        self.rng.gen_range(range)
    }
}
