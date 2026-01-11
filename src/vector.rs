use rand::Rng;
use std::collections::HashSet;

/// Modes for random vector generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VectorRandomMode {
    Unique,
    Repeatable,
    Float,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntRange {
    Max(i64),
    MinMax(i64, i64),
}

impl From<i64> for IntRange {
    fn from(value: i64) -> Self {
        IntRange::Max(value)
    }
}

impl From<(i64, i64)> for IntRange {
    fn from(value: (i64, i64)) -> Self {
        IntRange::MinMax(value.0, value.1)
    }
}

pub type IntVector = Vec<Vec<i64>>;

/// Vector utilities (random generators, helpers).
pub struct Vector;

fn normalize_int_ranges(position_range: &[IntRange]) -> Vec<IntRange> {
    if position_range.is_empty() {
        vec![IntRange::Max(10)]
    } else {
        position_range.to_vec()
    }
}

fn parse_int_ranges(ranges: &[IntRange]) -> (Vec<i64>, Vec<i64>) {
    let mut offsets: Vec<i64> = Vec::with_capacity(ranges.len());
    let mut lengths: Vec<i64> = Vec::with_capacity(ranges.len());
    for range in ranges {
        match *range {
            IntRange::Max(max) => {
                offsets.push(0);
                lengths.push(max);
            }
            IntRange::MinMax(min, max) => {
                assert!(max >= min, "upper-bound should be larger than lower-bound");
                offsets.push(min);
                lengths.push(max - min);
            }
        }
    }
    (offsets, lengths)
}

fn vector_space(lengths: &[i64]) -> u128 {
    lengths
        .iter()
        .fold(1u128, |acc, &len| acc * (len as u128 + 1))
}

fn get_vector_from_index(lengths: &[i64], mut hashcode: u128) -> Vec<i64> {
    let mut tmp: Vec<i64> = Vec::with_capacity(lengths.len());
    for &len in lengths {
        let base = len as u128 + 1;
        tmp.push((hashcode % base) as i64);
        hashcode /= base;
    }
    tmp
}

impl Vector {
    pub fn random_int(num: usize, position_range: &[IntRange]) -> IntVector {
        let ranges = normalize_int_ranges(position_range);
        let (offsets, lengths) = parse_int_ranges(&ranges);

        let mut rng = rand::rng();
        let mut result: IntVector = Vec::with_capacity(num);
        for _ in 0..num {
            let mut vec = Vec::with_capacity(ranges.len());
            for (&offset, &length) in offsets.iter().zip(lengths.iter()) {
                let val = rng.gen_range(offset..=offset + length);
                vec.push(val);
            }
            result.push(vec);
        }
        result
    }

    pub fn random_unique_vector(num: usize, position_range: &[IntRange]) -> IntVector {
        let ranges = normalize_int_ranges(position_range);
        let (offsets, lengths) = parse_int_ranges(&ranges);
        let space = vector_space(&lengths);
        assert!(num as u128 <= space, "not enough unique vectors in range");

        let mut rng = rand::rng();
        let mut used: HashSet<u128> = HashSet::with_capacity(num * 2 + 1);
        let mut result: IntVector = Vec::with_capacity(num);
        while result.len() < num {
            let rand_idx = rng.gen_range(0..space);
            if used.insert(rand_idx) {
                let mut vec = get_vector_from_index(&lengths, rand_idx);
                for (v, offset) in vec.iter_mut().zip(offsets.iter()) {
                    *v += *offset;
                }
                result.push(vec);
            }
        }
        result
    }

    pub fn random_repeatable_vector(num: usize, position_range: &[IntRange]) -> IntVector {
        Self::random_int(num, position_range)
    }
}
