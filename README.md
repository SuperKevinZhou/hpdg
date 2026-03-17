# hpdg

`hpdg` (**H**igh-**P**erformance **D**ata **G**enerator) is a Rust-first data generator for competitive programming and OI workflows.

The project is inspired by Luogu's CYaRon, but it leans into Rust's strengths instead of cloning the Python API one-to-one: strong typing, trait-based extension points, feature flags, reproducible RNG streams, and better composition for larger generators.

## Status

`hpdg` is under active development, but it is no longer just a skeleton:

- The crate compiles across library, examples, benches, and tests.
- Core generator modules are implemented and usable today.
- The public API is still evolving, so breaking changes are possible before `1.0`.
- Python/C binding scaffolding exists, but the Rust crate is the primary supported surface right now.

## What Works Today

| Area | Current support |
| --- | --- |
| IO / testcase helpers | In-memory buffers, configurable file naming, separators/formatters, flush to disk, output generation via external programs, timeout helpers, parallel generation, streaming writers, batch builders |
| Graph | `Edge`, `SwitchGraph`, editable `Graph`, graph formatting, label/edge shuffling, chain/flower/tree/binary tree generators, simple graph/multigraph/connected graph/forest/DAG helpers, degree-sequence constructors, adjacency-list and matrix export, `GraphMatrix`, `Merger`, hack-SPFA generator |
| Math | Permutation/palindrome helpers, divisor and digit utilities, Fibonacci, primality + Miller-Rabin, factorization, permutation/binomial/Catalan helpers, sieve, `exgcd`, modular inverse, `phi`, `miu`, base conversion, number-to-words |
| Sequence | Formula-driven `Sequence`, initial-value bootstrapping, range/slice helpers, arithmetic and geometric helpers |
| Vector | Random integer vectors, unique/repeatable modes, random float vectors, matrix helpers, formatting helpers |
| String | Random strings/words/sentences/paragraphs, simplified regex-based generation, dictionary-based generation, ASCII/Unicode modes |
| Polygon | Random points, simple polygons, convex hulls, perimeter/area helpers |
| Query | Random range queries, mixed update/query streams, per-dimension constraints, weighted query generation |
| Compare | File/string/program comparison, custom graders, whitespace-insensitive grader, parallel compare helpers |
| Rust-specific extras | `NodeId`/`EdgeId` newtypes, `Weight` trait, seeded RNG streams, `Generator` / `GraphGenerator` traits, shared error types |

## What Is Still Rough

These parts exist, but they are still early and should be treated as unstable:

- Crate-level documentation and docs.rs polish are still in progress.
- Some modules have broad functionality but still need API cleanup and more examples.
- Python bindings and the C ABI are scaffolding-level rather than polished releases.
- Full CYaRon parity is not the goal; some APIs will stay intentionally more Rust-idiomatic.

## Quick Example

```rust
use hpdg::graph::Graph;
use hpdg::io::IO;
use hpdg::string::{SentenceConfig, StringGen};
use hpdg::vector::{IntRange, Vector};

fn main() {
    let mut io = IO::new("example".to_string());

    let mut g = Graph::new(5, false);
    g.add_edge(1, 2, None);
    g.add_edge(2, 3, None);

    let vecs = Vector::random_int(3, &[IntRange::Max(10), IntRange::MinMax(5, 8)]);

    let mut cfg = SentenceConfig::default();
    cfg.sentence_terminators = ".".to_string();
    let sentence = StringGen::random_sentence(6, Some(&cfg));

    io.input_writeln("5 2");
    io.input_writeln(format!("{}", g));
    io.input_writeln(vecs.len());
    for v in vecs {
        io.input_writeln(v.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
    }
    io.input_writeln(sentence);

    let _ = io.flush_to_disk();
}
```

See also:

- `examples/full_generation.rs`
- `docs/rust-quickstart.md`
- `docs/cyaron-api.md`

## Installation

Until the first crates.io release lands, depend on the Git repository directly:

```toml
[dependencies]
hpdg = { git = "https://github.com/SuperKevinZhou/hpdg" }
```

After the crate is published, a normal crates.io dependency will be the preferred installation path.

## Feature Flags

Most generator modules are enabled by default.

Optional integration flags currently include:

- `python-bindings`
- `c-bindings`

The remaining feature flags mostly mirror internal module boundaries such as `io`, `graph`, `math`, `sequence`, `vector`, `string`, `polygon`, `query`, `compare`, `utils`, `core`, `rng`, `traits`, and `error`.

## Project Direction

The long-term goal is to make `hpdg` a practical Rust alternative to CYaRon for serious OI data generation:

- Keep common workflows concise.
- Prefer deterministic, reproducible generation.
- Provide stronger invariants through types and traits.
- Add Rust-specific building blocks instead of only mirroring Python behavior.

Issues and PRs are welcome.

## License

MIT
