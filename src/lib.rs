//! # hpdg
//!
//! `hpdg` is a Rust-first data generator for competitive programming and OI workflows.
//! The crate takes inspiration from tools such as CYaRon, but it leans into Rust's
//! own strengths: strong typing, feature-gated modules, reproducible randomness, and
//! composable building blocks.
//!
//! ## Why hpdg?
//!
//! - Build graph, string, vector, geometry, and query generators from a single crate.
//! - Keep testcase generation reproducible with seeded RNG streams.
//! - Use Rust-native abstractions such as traits and newtypes instead of stringly APIs.
//! - Mix quick scripts and larger generator codebases without changing tools.
//!
//! ## Quick Start
//!
//! ```rust
//! use hpdg::graph::Graph;
//! use hpdg::io::IO;
//! use hpdg::string::{SentenceConfig, StringGen};
//! use hpdg::vector::{IntRange, Vector};
//!
//! let mut io = IO::new("sample".to_string());
//!
//! let mut graph = Graph::new(4, false);
//! graph.add_edge(1, 2, None);
//! graph.add_edge(2, 3, None);
//!
//! let points = Vector::random_int(2, &[IntRange::MinMax(1, 5), IntRange::MinMax(10, 20)]);
//!
//! let mut cfg = SentenceConfig::default();
//! cfg.sentence_terminators = ".".to_string();
//! let sentence = StringGen::random_sentence(4, Some(&cfg));
//!
//! io.input_writeln("4 2");
//! io.input_writeln(graph);
//! io.input_writeln(points.len());
//! io.input_writeln(sentence);
//! ```
//!
//! ## Module Guide
//!
//! - [`io`]: testcase buffers, file naming, process execution, and streaming output.
//! - [`graph`]: graph containers plus random trees, DAGs, connected graphs, and helpers.
//! - [`math`]: number theory, combinatorics, digit helpers, and formatting utilities.
//! - [`sequence`]: memoized formula-based sequences and common progressions.
//! - [`vector`]: random integer and floating-point vectors and matrices.
//! - [`string`]: random words, sentences, paragraphs, dictionary sampling, and regex-like text.
//! - [`polygon`]: integer geometry helpers, convex hulls, and simple polygons.
//! - [`query`]: random range queries, mixed update/query streams, and weighted generators.
//! - [`compare`]: compare outputs from strings, files, or child processes with custom graders.
//! - [`rng`]: deterministic RNG wrappers and reproducible stream splitting.
//! - [`core`] and [`traits`]: Rust-flavored primitives such as newtypes and generator traits.
//!
//! ## Recommended Path
//!
//! If you are new to the crate, the smoothest way to explore it is:
//!
//! 1. Start with [`io::IO`] for testcase structure and file output.
//! 2. Pick a generator module such as [`graph::Graph`], [`string::StringGen`], or [`vector::Vector`].
//! 3. Use [`rng::SeededRng`] or [`rng::RngStream`] when you need reproducible cases.
//! 4. Add [`compare`] helpers when you want a local standard-solution workflow.
//!
//! ## Stability
//!
//! `hpdg` is still evolving. The core modules are usable today, but APIs may change before
//! `1.0` as the crate becomes more polished and more Rust-idiomatic.
//!
/// Experimental C ABI bindings.
#[cfg(feature = "c-bindings")]
pub mod c_api;
/// Output comparison utilities and customizable graders.
#[cfg(feature = "compare")]
pub mod compare;
/// Rust-first primitives such as strongly typed node and edge identifiers.
#[cfg(feature = "core")]
pub mod core;
/// Shared error types used across the crate.
#[cfg(feature = "error")]
pub mod error;
/// Graph containers, formatters, and random graph/tree generators.
#[cfg(feature = "graph")]
pub mod graph;
/// Testcase I/O buffers, file naming, process execution, and streaming writers.
#[cfg(feature = "io")]
pub mod io;
/// Math helpers for number theory, combinatorics, digits, and formatting.
#[cfg(feature = "math")]
pub mod math;
/// Integer geometry helpers for points and polygons.
#[cfg(feature = "polygon")]
pub mod polygon;
/// Random range-query generators, including weighted and mixed workloads.
#[cfg(feature = "query")]
pub mod query;
/// Deterministic RNG wrappers and reproducible random streams.
#[cfg(feature = "rng")]
pub mod rng;
/// Memoized formula-based sequences and common progression helpers.
#[cfg(feature = "sequence")]
pub mod sequence;
/// Random strings, words, sentences, paragraphs, and regex-like generators.
#[cfg(feature = "string")]
pub mod string;
/// Backward-compatible alias around [`io::IO`].
#[cfg(feature = "testcase")]
pub mod testcase;
/// Traits for building reusable generator abstractions.
#[cfg(feature = "traits")]
pub mod traits;
/// Small utility helpers for argument parsing, path escaping, and trait probes.
#[cfg(feature = "utils")]
pub mod utils;
/// Random vectors, matrices, and vector formatting utilities.
#[cfg(feature = "vector")]
pub mod vector;
