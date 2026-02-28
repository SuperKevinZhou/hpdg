#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NodeId(pub usize);

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

pub trait Weight: Copy + Clone + Default + std::fmt::Debug + std::fmt::Display + PartialOrd {}

impl<T> Weight for T where
    T: Copy + Clone + Default + std::fmt::Debug + std::fmt::Display + PartialOrd
{
}
