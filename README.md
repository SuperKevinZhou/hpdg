# hpdg

Welcome to hpdg (**H**igh-**P**erformance **D**ata **G**enerator)!

**hpdg** is a high-performance data generator for competitive programming use, which put performance at the first place. As it is written in Rust, it should be faster than Luogu's CYaRon data generator. It will also make the program almost impossible to misuse different varibles for Rust's strong and safe typing system.

This problem is currently under heavy development and isn't ready for production use. Any PR / Issue is welcome.

This is a data generator for competitive programming (Olympaid in Informatics) problems.

Compared to Luogu's CYaRon, this project aims to be a faster data generator written in Rust.

## Features (not all implemented yet)

- Fast testcase input/output
  - Testcase output generation from std program
- Multi-thread data generating for multi-testcase problems.
- Graph (and tree) generating with various parameters.
- Python port and C++ port.

## Current State

- [ ] **Testcases / IO**
  - [x] In-memory input/output buffer and file naming (`Testcase`)
  - [ ] Write/flush to disk
  - [ ] Custom separators / formatting helpers
  - [ ] Directory creation & path escaping
  - [ ] Run std program to generate outputs (like CYaRon `IO.output_gen`)
  - [ ] Process control (timeout, kill children)
  - [ ] Batch testcase helpers / multi-case runners
  - [ ] Output capture & compare with std
- [ ] **Graph**
  - [x] *Edge*: weighted and unweighted edge which supports creating and printing
  - [x] *SwitchGraph*: a simple graph which supports switching
  - [x] *SwitchGraph*: from degree sequences (directed / undirected)
  - [ ] *Graph*:
    - [x] Manual creating and editing
    - [x] Print / shuffle graph with custom edge formatter
    - [x] Generate trees and binary-trees
    - [ ] Chain / flower generators
    - [ ] Generate any graphs (simple / multigraph, optional self-loop)
    - [ ] Generate directed and undirected DAGs
    - [ ] Generate from degree sequences (Graph-level wrapper)
    - [ ] Generate connected graphs
    - [ ] Generate forests
    - [ ] Hack SPFA graph generator
    - [ ] Convert to adjacency list / matrix and statistics helpers
  - [ ] *GraphMatrix*: a type of graph represented by adjacency matrix
  - [ ] *Merger*: merge graphs / build connected components
- [ ] **Math**
  - [x] *is_perm*
  - [x] *is_pal_string* & *is_pal_u64*
  - [x] *divisor_sum*
  - [x] *is_pandigital*, *is_pandigital_u64* & *is_pandigital_u64_default*
  - [ ] *is_palindromic* wrapper (int / str)
  - [ ] *pal_list*
  - [ ] Sum/product/sum-of-squares of digits
  - [ ] Fibonacci (fast doubling)
  - [ ] Prime test (Miller-Rabin), sieve, factorization
  - [ ] Permutation / binomial / Catalan
  - [ ] exgcd / mod inverse / phi / miu
  - [ ] Base conversion / number-to-words
- [ ] **Sequence**
  - [ ] Formula-based sequence generator (CYaRon `Sequence`)
  - [ ] Range slicing / batch generation
- [ ] **Vector**
  - [ ] Random integer vectors (unique / repeatable modes)
  - [ ] Random float vectors
  - [ ] Multi-dimensional vectors / matrices
- [ ] **String / Text**
  - [ ] Random string with alphabet and constraints
  - [ ] Random word / sentence / paragraph
  - [ ] Random regex-matching strings
  - [ ] Dictionary-based generation
- [ ] **Polygon / Geometry**
  - [ ] Convex hull generator
  - [ ] Simple polygon generator
  - [ ] Perimeter / area helpers
- [ ] **Range Query**
  - [ ] Random range queries (intervals, updates)
  - [ ] Weighted query generators
- [ ] **Compare / Judge Utils**
  - [ ] Output compare for files / programs
  - [ ] Custom grader hook
  - [ ] Parallel comparison
- [ ] **Utils**
  - [ ] `ati` / `list_like` / `int_like`
  - [ ] CLI args processing
  - [ ] Path escaping / string helpers
- [ ] **Rust-specific Extras (optional)**
  - [ ] Strongly typed IDs (newtype) + trait-based generators
  - [ ] Generic weight types (`impl Weight`)
  - [ ] Deterministic RNG with seed and reproducible streams
  - [ ] Streaming writers to avoid big buffers
  - [ ] Property-based tests / benchmarks
  - [ ] Python / C++ bindings
