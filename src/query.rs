//! Random range-query generators.
//!
//! This module helps build workloads for segment trees, Fenwick trees, sparse tables,
//! multidimensional prefix sums, and similar data-structure problems.
//!
//! # Example
//!
//! ```rust
//! use hpdg::query::{RangeLimit, RangeQuery, RangeQueryRandomMode};
//!
//! let queries = RangeQuery::random(3, &[RangeLimit::MinMax(1, 10)], RangeQueryRandomMode::AllowEqual, 0.3);
//! assert_eq!(queries.len(), 3);
//! ```

use rand::Rng;

/// Random endpoint generation policy for range queries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RangeQueryRandomMode {
    /// Require strict inequality `left < right` for every dimension.
    Less,
    /// Allow degenerate intervals with `left == right`.
    AllowEqual,
}

/// Operation type for mixed queries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryOp {
    /// Update operation.
    Update,
    /// Query operation.
    Query,
}

/// Mixed update/query sequence.
#[derive(Debug, Clone, Default)]
pub struct MixedRangeQuery {
    /// Generated operations as `(op, left, right)`.
    pub result: Vec<(QueryOp, Vec<i64>, Vec<i64>)>,
}

/// Constraints for query lengths.
#[derive(Debug, Clone, Copy, Default)]
pub struct RangeQueryConstraints {
    /// Inclusive lower bound for interval length.
    pub min_len: Option<i64>,
    /// Inclusive upper bound for interval length.
    pub max_len: Option<i64>,
}

/// Per-dimension range limit for queries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RangeLimit {
    /// Inclusive range `[0, max]`.
    Max(i64),
    /// Inclusive range `[min, max]`.
    MinMax(i64, i64),
}

impl From<i64> for RangeLimit {
    fn from(value: i64) -> Self {
        Self::Max(value)
    }
}

impl From<(i64, i64)> for RangeLimit {
    fn from(value: (i64, i64)) -> Self {
        Self::MinMax(value.0, value.1)
    }
}

/// Container for generated range queries.
#[derive(Debug, Clone, Default)]
pub struct RangeQuery<W> {
    /// Generated query tuples as `(left, right, payload)`.
    pub result: Vec<(Vec<i64>, Vec<i64>, W)>,
}

fn normalize_ranges(position_range: &[RangeLimit]) -> Vec<(i64, i64)> {
    let ranges = if position_range.is_empty() {
        vec![RangeLimit::Max(10)]
    } else {
        position_range.to_vec()
    };

    ranges
        .into_iter()
        .map(|range| match range {
            RangeLimit::Max(v) => (1, v),
            RangeLimit::MinMax(l, r) => (l, r),
        })
        .collect()
}

fn join_vec(values: &[i64]) -> String {
    values
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

impl MixedRangeQuery {
    /// Generate random mixed update/query operations.
    ///
    /// `big_query` biases sampling toward longer intervals, while `update_ratio` controls
    /// how often an operation is tagged as [`QueryOp::Update`].
    pub fn random(
        num: usize,
        position_range: &[RangeLimit],
        mode: RangeQueryRandomMode,
        big_query: f64,
        update_ratio: f64,
    ) -> Self {
        let mut rng = rand::rng();
        let mut ret = Self::default();

        for _ in 0..num {
            let op = if rng.random::<f64>() < update_ratio {
                QueryOp::Update
            } else {
                QueryOp::Query
            };
            let (l, r, ()) = RangeQuery::<()>::get_one_query(position_range, mode, big_query);
            ret.result.push((op, l, r));
        }

        ret
    }

    /// Convert mixed queries to a newline-separated string.
    ///
    /// Each line is rendered as `U l... r...` or `Q l... r...`.
    pub fn to_string(&self) -> String {
        let mut lines = Vec::with_capacity(self.result.len());
        for (op, l, r) in &self.result {
            let tag = match op {
                QueryOp::Update => "U",
                QueryOp::Query => "Q",
            };
            lines.push(format!("{} {} {}", tag, join_vec(l), join_vec(r)));
        }
        lines.join("\n")
    }
}

impl<W> RangeQuery<W> {
    /// Create an empty query container.
    pub fn new() -> Self {
        Self { result: Vec::new() }
    }

    /// Return the number of generated queries.
    pub fn len(&self) -> usize {
        self.result.len()
    }

    /// Return `true` if the container has no queries.
    pub fn is_empty(&self) -> bool {
        self.result.is_empty()
    }

    /// Iterate over generated query triples `(left, right, payload)`.
    pub fn iter(&self) -> impl Iterator<Item = &(Vec<i64>, Vec<i64>, W)> {
        self.result.iter()
    }
}

impl RangeQuery<()> {
    /// Generate a single unweighted query.
    ///
    /// The returned tuple is `(left, right, ())`, where `left` and `right` are vectors for
    /// multi-dimensional queries.
    pub fn get_one_query(
        position_range: &[RangeLimit],
        mode: RangeQueryRandomMode,
        big_query: f64,
    ) -> (Vec<i64>, Vec<i64>, ()) {
        let ranges = normalize_ranges(position_range);
        let mut rng = rand::rng();
        let mut query_l = Vec::with_capacity(ranges.len());
        let mut query_r = Vec::with_capacity(ranges.len());

        for (low, high) in ranges {
            assert!(high >= low, "upper-bound should be larger than lower-bound");
            assert!(
                mode != RangeQueryRandomMode::Less || low != high,
                "mode is set to less but upper-bound is equal to lower-bound"
            );

            let (l, r) = if rng.random::<f64>() < big_query {
                let len = high - low + 1;
                let lb = if mode == RangeQueryRandomMode::Less {
                    2.max(len / 2)
                } else {
                    1.max(len / 2)
                };
                let ql = rng.random_range(lb..=len);
                let l = rng.random_range(low..=high - ql + 1);
                (l, l + ql - 1)
            } else {
                let mut l = rng.random_range(low..=high);
                let mut r = rng.random_range(low..=high);
                while mode == RangeQueryRandomMode::Less && l == r {
                    l = rng.random_range(low..=high);
                    r = rng.random_range(low..=high);
                }
                if l > r {
                    std::mem::swap(&mut l, &mut r);
                }
                (l, r)
            };

            query_l.push(l);
            query_r.push(r);
        }

        (query_l, query_r, ())
    }

    /// Generate `num` random unweighted queries.
    pub fn random(
        num: usize,
        position_range: &[RangeLimit],
        mode: RangeQueryRandomMode,
        big_query: f64,
    ) -> Self {
        let mut ret = Self::new();
        for _ in 0..num {
            ret.result
                .push(Self::get_one_query(position_range, mode, big_query));
        }
        ret
    }

    /// Generate a single query while honoring length constraints.
    ///
    /// Constraints are interpreted per dimension after the endpoint range has been chosen.
    pub fn get_one_query_with_constraints(
        position_range: &[RangeLimit],
        mode: RangeQueryRandomMode,
        big_query: f64,
        constraints: RangeQueryConstraints,
    ) -> (Vec<i64>, Vec<i64>, ()) {
        let ranges = normalize_ranges(position_range);
        let mut rng = rand::rng();
        let mut query_l = Vec::with_capacity(ranges.len());
        let mut query_r = Vec::with_capacity(ranges.len());

        for (low, high) in ranges {
            assert!(high >= low, "upper-bound should be larger than lower-bound");
            let range_len = high - low + 1;
            let mut min_len = constraints.min_len.unwrap_or(1);
            let mut max_len = constraints.max_len.unwrap_or(range_len);
            if mode == RangeQueryRandomMode::Less {
                min_len = min_len.max(2);
            }
            max_len = max_len.min(range_len);
            assert!(min_len <= max_len, "invalid length constraints");

            let (len_min, len_max) = if rng.random::<f64>() < big_query {
                ((range_len / 2).max(min_len), max_len)
            } else {
                (min_len, max_len)
            };
            let ql = rng.random_range(len_min..=len_max);
            let l = rng.random_range(low..=high - ql + 1);
            let r = l + ql - 1;

            query_l.push(l);
            query_r.push(r);
        }

        (query_l, query_r, ())
    }

    /// Generate `num` random queries while honoring length constraints.
    pub fn random_with_constraints(
        num: usize,
        position_range: &[RangeLimit],
        mode: RangeQueryRandomMode,
        big_query: f64,
        constraints: RangeQueryConstraints,
    ) -> Self {
        let mut ret = Self::new();
        for _ in 0..num {
            ret.result.push(Self::get_one_query_with_constraints(
                position_range,
                mode,
                big_query,
                constraints,
            ));
        }
        ret
    }

    /// Convert unweighted queries to output string.
    /// Render the generated queries as a newline-separated string.
    pub fn to_string(&self) -> String {
        let mut lines = Vec::with_capacity(self.result.len());
        for (l, r, ()) in &self.result {
            lines.push(format!("{} {}", join_vec(l), join_vec(r)));
        }
        lines.join("\n")
    }
}

impl<W> RangeQuery<W> {
    /// Generate a single query together with a derived weight or payload.
    ///
    /// The payload is computed by calling `weight_fn(&left, &right)`.
    pub fn get_one_query_with_weight<F>(
        position_range: &[RangeLimit],
        mode: RangeQueryRandomMode,
        big_query: f64,
        weight_generator: &F,
        index: usize,
    ) -> (Vec<i64>, Vec<i64>, W)
    where
        F: Fn(usize, &[i64], &[i64]) -> W,
    {
        let (l, r, ()) = RangeQuery::<()>::get_one_query(position_range, mode, big_query);
        let w = weight_generator(index, &l, &r);
        (l, r, w)
    }

    /// Generate `num` random weighted queries.
    pub fn random_with_weight<F>(
        num: usize,
        position_range: &[RangeLimit],
        mode: RangeQueryRandomMode,
        big_query: f64,
        weight_generator: F,
    ) -> Self
    where
        F: Fn(usize, &[i64], &[i64]) -> W,
    {
        let mut ret = Self::new();
        for i in 0..num {
            ret.result.push(Self::get_one_query_with_weight(
                position_range,
                mode,
                big_query,
                &weight_generator,
                i + 1,
            ));
        }
        ret
    }
}

impl<W: std::fmt::Display> RangeQuery<W> {
    /// Render weighted queries as a newline-separated string.
    ///
    /// Each line is formatted as `left... right... weight`.
    pub fn to_string_with_weight(&self) -> String {
        let mut lines = Vec::with_capacity(self.result.len());
        for (l, r, w) in &self.result {
            lines.push(format!("{} {} {}", join_vec(l), join_vec(r), w));
        }
        lines.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_basic_queries() {
        let q = RangeQuery::random(
            5,
            &[RangeLimit::MinMax(1, 5)],
            RangeQueryRandomMode::AllowEqual,
            0.0,
        );
        assert_eq!(q.len(), 5);
        for (l, r, ()) in q.result {
            assert!(l[0] <= r[0]);
        }
    }

    #[test]
    fn test_random_with_weight() {
        let q = RangeQuery::random_with_weight(
            3,
            &[RangeLimit::MinMax(1, 4)],
            RangeQueryRandomMode::AllowEqual,
            0.0,
            |idx, _l, _r| idx as i64,
        );
        assert_eq!(q.len(), 3);
        assert_eq!(q.result[0].2, 1);
    }

    #[test]
    fn test_constraints() {
        let constraints = RangeQueryConstraints {
            min_len: Some(3),
            max_len: Some(3),
        };
        let q = RangeQuery::random_with_constraints(
            2,
            &[RangeLimit::MinMax(1, 5)],
            RangeQueryRandomMode::AllowEqual,
            0.0,
            constraints,
        );
        for (l, r, ()) in q.result {
            assert_eq!(r[0] - l[0] + 1, 3);
        }
    }

    #[test]
    fn test_mixed_queries_output() {
        let mixed = MixedRangeQuery::random(
            2,
            &[RangeLimit::MinMax(1, 3)],
            RangeQueryRandomMode::AllowEqual,
            0.0,
            0.5,
        );
        let out = mixed.to_string();
        assert_eq!(out.lines().count(), 2);
    }
}
