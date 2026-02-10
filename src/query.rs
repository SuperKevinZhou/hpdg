use rand::Rng;

﻿#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RangeQueryRandomMode {
    Less,
    AllowEqual,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryOp {
    Update,
    Query,
}

#[derive(Debug, Clone, Default)]
pub struct MixedRangeQuery {
    pub result: Vec<(QueryOp, Vec<i64>, Vec<i64>)>,
}

#[derive(Debug, Clone, Copy)]
pub struct RangeQueryConstraints {
    pub min_len: Option<i64>,
    pub max_len: Option<i64>,
}

impl Default for RangeQueryConstraints {
    fn default() -> Self {
        Self {
            min_len: None,
            max_len: None,
        }
    }
}

impl MixedRangeQuery {
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
            let op = if rng.gen::<f64>() < update_ratio {
                QueryOp::Update
            } else {
                QueryOp::Query
            };
            let (l, r, ()) = RangeQuery::<()>::get_one_query(position_range, mode, big_query);
            ret.result.push((op, l, r));
        }
        ret
    }

    pub fn to_string(&self) -> String {
        let mut lines = Vec::with_capacity(self.result.len());
        for (op, l, r) in &self.result {
            let tag = match op {
                QueryOp::Update => "U",
                QueryOp::Query => "Q",
            };
            lines.push(format!("{} {} {}", tag, join_vec(l), join_vec(r)));
        }
        lines.join("
")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RangeLimit {
    Max(i64),
    MinMax(i64, i64),
}

impl From<i64> for RangeLimit {
    fn from(value: i64) -> Self {
        RangeLimit::Max(value)
    }
}

impl From<(i64, i64)> for RangeLimit {
    fn from(value: (i64, i64)) -> Self {
        RangeLimit::MinMax(value.0, value.1)
    }
}

#[derive(Debug, Clone, Default)]
pub struct RangeQuery<W> {
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
        .map(|r| match r {
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

impl<W> RangeQuery<W> {
    pub fn new() -> Self {
        Self { result: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.result.len()
    }

    pub fn is_empty(&self) -> bool {
        self.result.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &(Vec<i64>, Vec<i64>, W)> {
        self.result.iter()
    }
}

impl RangeQuery<()> {
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
            if mode == RangeQueryRandomMode::Less && low == high {
                panic!("mode is set to less but upper-bound is equal to lower-bound");
            }

            let (mut l, mut r) = if rng.gen::<f64>() < big_query {
                let len = high - low + 1;
                let lb = if mode == RangeQueryRandomMode::Less {
                    2.max(len / 2)
                } else {
                    1.max(len / 2)
                };
                let ql = rng.gen_range(lb..=len);
                let l = rng.gen_range(low..=high - ql + 1);
                let r = l + ql - 1;
                (l, r)
            } else {
                let mut l = rng.gen_range(low..=high);
                let mut r = rng.gen_range(low..=high);
                while mode == RangeQueryRandomMode::Less && l == r {
                    l = rng.gen_range(low..=high);
                    r = rng.gen_range(low..=high);
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

    pub fn random(
        num: usize,
        position_range: &[RangeLimit],
        mode: RangeQueryRandomMode,
        big_query: f64,
    ) -> Self {
        let mut ret = Self::new();
        for _ in 0..num {
            ret.result.push(Self::get_one_query(position_range, mode, big_query));
        }
        ret
    }

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

            let (len_min, len_max) = if rng.gen::<f64>() < big_query {
                let lb = (range_len / 2).max(min_len);
                (lb, max_len)
            } else {
                (min_len, max_len)
            };
            let ql = rng.gen_range(len_min..=len_max);
            let l = rng.gen_range(low..=high - ql + 1);
            let r = l + ql - 1;

            query_l.push(l);
            query_r.push(r);
        }
        (query_l, query_r, ())
    }

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
}

impl<W> RangeQuery<W> {
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
    pub fn to_string_with_weight(&self) -> String {
        let mut lines = Vec::with_capacity(self.result.len());
        for (l, r, w) in &self.result {
            lines.push(format!("{} {} {}", join_vec(l), join_vec(r), w));
        }
        lines.join("
")
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_basic_queries() {
        let q = RangeQuery::random(5, &[RangeLimit::MinMax(1, 5)], RangeQueryRandomMode::AllowEqual, 0.0);
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
        let constraints = RangeQueryConstraints { min_len: Some(3), max_len: Some(3) };
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
