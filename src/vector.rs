/// Modes for random vector generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VectorRandomMode {
    Unique,
    Repeatable,
    Float,
}

/// Vector utilities (random generators, helpers).
pub struct Vector;
