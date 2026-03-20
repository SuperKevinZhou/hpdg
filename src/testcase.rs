//! Backward-compatible testcase alias.
//!
//! Older examples may refer to `Testcase` following the naming style used by some OI
//! generators. In `hpdg`, the canonical type is [`crate::io::IO`].

/// Alias for [`crate::io::IO`].
pub use crate::io::IO as Testcase;
