# Rust Quickstart

This guide shows a minimal workflow to use hpdg in Rust projects, plus a few best practices for reproducible data generation.

## Add Dependency

```toml
[dependencies]
hpdg = { path = ".." }
```

## Basic IO

```rust
use hpdg::io::IO;

let mut io = IO::new("sample".to_string());
io.input_writeln("3 4");
io.output_writeln("7");
let _ = io.flush_to_disk();
```

## Graph Example

```rust
use hpdg::graph::Graph;

let mut g = Graph::new(5, false);
g.add_edge(1, 2, None);
g.add_edge(2, 3, None);
println!("{}", g);
```

## Best Practices

- Prefer a deterministic seed when debugging or sharing cases.
- Keep generators small and composable; build complex cases by layering.
- Use `IO::output_gen` for standard-solution checking when available.
- Keep range constraints explicit to avoid accidental invalid queries.
- Use streaming writers for very large outputs.
