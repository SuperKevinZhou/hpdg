# CYaRon API Comparison

This table maps common CYaRon APIs to their hpdg counterparts.

| CYaRon | hpdg | Notes |
| --- | --- | --- |
| `IO` | `io::IO` | Input/output buffers, file naming, output generation |
| `Graph` | `graph::Graph` | Edge list graph utilities |
| `Sequence` | `sequence::Sequence` | Formula-based sequences |
| `Vector` | `vector::Vector` | Random vectors and matrices |
| `String` | `string::StringGen` | Random strings and text |
| `RangeQuery` | `query::RangeQuery` | Random range queries |
| `compare` | `compare` module | Output comparison and graders |

For Rust-specific extras, see `core`, `rng`, and `traits` modules.
