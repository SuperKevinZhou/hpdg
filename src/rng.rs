//! Deterministic and platform-friendly random helpers.
//!
//! Use [`crate::rng::SeededRng`] when a single reproducible generator is enough, or
//! [`crate::rng::RngStream`] when
//! you want to derive multiple independent sub-generators from one base seed.
//!
//! # Example
//!
//! ```rust
//! use hpdg::rng::RngStream;
//!
//! let mut streams = RngStream::new(20250327);
//! let mut a = streams.next();
//! let mut b = streams.fork(42);
//!
//! let av = a.gen_range(1..=10);
//! let bv = b.gen_range(1..=10);
//! assert!((1..=10).contains(&av));
//! assert!((1..=10).contains(&bv));
//! ```
//!
//! ## Choosing An API
//!
//! - Use [`crate::rng::SeededRng`] for one reproducible generator.
//! - Use [`crate::rng::RngStream`] when you want deterministic substreams for different testcase families.
//! - Use [`crate::rng::random_u64`] when reproducibility is not required and you just need one fresh value.

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

/// A reproducible random-number generator backed by [`StdRng`].
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
    /// Create a deterministic RNG from a fixed seed.
    pub fn new(seed: u64) -> Self {
        Self {
            rng: StdRng::seed_from_u64(seed),
        }
    }

    /// Borrow the underlying [`StdRng`] for custom sampling.
    pub fn rng(&mut self) -> &mut StdRng {
        &mut self.rng
    }

    /// Sample a value from an inclusive range.
    ///
    /// This mirrors the familiar `gen_range`-style helper while delegating to `rand 0.9`.
    pub fn gen_range<T>(&mut self, range: std::ops::RangeInclusive<T>) -> T
    where
        T: rand::distr::uniform::SampleUniform + Copy + PartialOrd,
    {
        self.rng.random_range(range)
    }
}

/// Split a base seed into multiple reproducible RNG streams.
pub struct RngStream {
    base_seed: u64,
    counter: u64,
}

impl RngStream {
    /// Create a new RNG stream rooted at `seed`.
    pub fn new(seed: u64) -> Self {
        Self {
            base_seed: seed,
            counter: 0,
        }
    }

    /// Derive a deterministic sub-RNG using an explicit stream identifier.
    ///
    /// Calling `fork` with the same `stream_id` and the same base seed always yields the same
    /// derived stream.
    pub fn fork(&self, stream_id: u64) -> SeededRng {
        SeededRng::new(splitmix64(self.base_seed ^ stream_id))
    }

    /// Derive the next deterministic sub-RNG from the stream.
    ///
    /// Successive calls advance an internal counter, making it convenient to assign one stream
    /// per testcase or worker.
    pub fn next(&mut self) -> SeededRng {
        let seed = splitmix64(self.base_seed.wrapping_add(self.counter));
        self.counter = self.counter.wrapping_add(1);
        SeededRng::new(seed)
    }
}

/// Generate a random `u64` on WebAssembly targets using `getrandom`.
#[cfg(target_arch = "wasm32")]
pub fn random_u64() -> u64 {
    let mut buf = [0u8; 8];
    let _ = getrandom::getrandom(&mut buf);
    u64::from_le_bytes(buf)
}

/// Generate a random `u64` on native targets.
#[cfg(not(target_arch = "wasm32"))]
pub fn random_u64() -> u64 {
    rand::random()
}
