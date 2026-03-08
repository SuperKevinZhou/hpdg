use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use getrandom;

pub struct SeededRng {
    rng: StdRng,
}

fn splitmix64(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9E3779B97F4A7C15);
    let mut z = x;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
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

pub struct RngStream {
    base_seed: u64,
    counter: u64,
}

impl RngStream {
    pub fn new(seed: u64) -> Self {
        Self {
            base_seed: seed,
            counter: 0,
        }
    }

    pub fn fork(&self, stream_id: u64) -> SeededRng {
        SeededRng::new(splitmix64(self.base_seed ^ stream_id))
    }

    pub fn next(&mut self) -> SeededRng {
        let seed = splitmix64(self.base_seed.wrapping_add(self.counter));
        self.counter = self.counter.wrapping_add(1);
        SeededRng::new(seed)
    }
}

#[cfg(target_arch = "wasm32")]
pub fn random_u64() -> u64 {
    let mut buf = [0u8; 8];
    let _ = getrandom::getrandom(&mut buf);
    u64::from_le_bytes(buf)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn random_u64() -> u64 {
    rand::random()
}
