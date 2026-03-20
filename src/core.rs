//! Core Rust-first building blocks shared across the crate.
//!
//! This module is intentionally small. It provides lightweight newtypes and traits that
//! make generator code more expressive without forcing a large framework on users.

/// Strongly typed node identifier for graph-oriented APIs.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NodeId(pub usize);

/// Strongly typed edge identifier for graph-oriented APIs.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EdgeId(pub usize);

impl From<usize> for NodeId {
    fn from(value: usize) -> Self {
        NodeId(value)
    }
}

impl From<usize> for EdgeId {
    fn from(value: usize) -> Self {
        EdgeId(value)
    }
}

/// Marker trait for values that can act as edge weights.
///
/// This is primarily useful when building generic generator layers on top of `hpdg`.
pub trait Weight:
    Copy + Clone + Default + std::fmt::Debug + std::fmt::Display + PartialOrd
{
}

impl<T> Weight for T where
    T: Copy + Clone + Default + std::fmt::Debug + std::fmt::Display + PartialOrd
{
}
