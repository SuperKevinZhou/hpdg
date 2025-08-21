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

- [ ] **Testcases**
- [ ] **Graph**
  - [x] *Edge*: weighted and unweighted edge which supports creating and printing
  - [x] *SwitchGraph*: a simple graph which supports switching
  - [ ] *Graph*:
    - [x] Manual creating and editing
    - [x] Print the graph in various mode
    - [x] Generate trees and binary-trees
    - [ ] Generate any graphs
    - [ ] Generate directed and undirected DAGs
    - [ ] Generate from degree sequences
    - [ ] Generate connected graphs
    - [ ] Generate forests
  - [ ] *GraphMatrix*: a type of graph represented by adjacency matrix
- [ ] **Math**
  - [x] *is_perm*
  - [x] *is_pal_string* & *is_pal_u64*
  - [x] *divisor_sum*
  - [x] *is_pandigital*, *is_pandigital_u64* & *is_pandigital_u64_default*
  - [ ] Other useful functions...
