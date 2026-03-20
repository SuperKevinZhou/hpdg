//! Traits for building reusable generators on top of `hpdg`.
//!
//! The traits in this module are intentionally lightweight: they are handy for your own
//! abstractions, but they stay out of the way if you prefer ad-hoc functions.

/// A simple trait for stateful generators that yield one value at a time.
pub trait Generator<T> {
    /// Produce a single generated value.
    fn generate(&mut self) -> T;

    /// Generate `n` values by repeatedly calling [`Generator::generate`].
    fn generate_n(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.generate()).collect()
    }
}

/// A specialized trait for graph generators.
///
/// Closures returning [`crate::graph::Graph`] automatically implement this trait.
pub trait GraphGenerator {
    /// Produce a graph instance.
    fn generate(&mut self) -> crate::graph::Graph;
}

impl<F> GraphGenerator for F
where
    F: FnMut() -> crate::graph::Graph,
{
    fn generate(&mut self) -> crate::graph::Graph {
        self()
    }
}
