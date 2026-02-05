use rand::Rng;

﻿#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RangeQueryRandomMode {
    Less,
    AllowEqual,
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
}
