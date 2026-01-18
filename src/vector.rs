use rand::Rng;
use std::collections::HashSet;

/// Modes for random vector generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VectorRandomMode {
    Unique,
    Repeatable,
    Float,
}

/// Integer range specification for each dimension.
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
pub type FloatVector = Vec<Vec<f64>>;

/// Floating-point range specification for each dimension.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FloatRange {
    Max(f64),
    MinMax(f64, f64),
}

impl From<f64> for FloatRange {
    fn from(value: f64) -> Self {
        FloatRange::Max(value)
    }
}

impl From<(f64, f64)> for FloatRange {
    fn from(value: (f64, f64)) -> Self {
        FloatRange::MinMax(value.0, value.1)
    }
}

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

fn normalize_float_ranges(position_range: &[FloatRange]) -> Vec<FloatRange> {
    if position_range.is_empty() {
        vec![FloatRange::Max(10.0)]
    } else {
        position_range.to_vec()
    }
}

fn parse_float_ranges(ranges: &[FloatRange]) -> (Vec<f64>, Vec<f64>) {
    let mut offsets: Vec<f64> = Vec::with_capacity(ranges.len());
    let mut lengths: Vec<f64> = Vec::with_capacity(ranges.len());
    for range in ranges {
        match *range {
            FloatRange::Max(max) => {
                offsets.push(0.0);
                lengths.push(max);
            }
            FloatRange::MinMax(min, max) => {
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
    /// Generate random integer vectors (duplicates allowed).
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

    /// Generate random integer vectors without duplicates.
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

    /// Generate random integer vectors with duplicates allowed (alias).
    pub fn random_repeatable_vector(num: usize, position_range: &[IntRange]) -> IntVector {
        Self::random_int(num, position_range)
    }

    /// Generate random floating-point vectors.
    pub fn random_float_vector(num: usize, position_range: &[FloatRange]) -> FloatVector {
        let ranges = normalize_float_ranges(position_range);
        let (offsets, lengths) = parse_float_ranges(&ranges);

        let mut rng = rand::rng();
        let mut result: FloatVector = Vec::with_capacity(num);
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

    /// Convert a hashcode into a vector based on the provided ranges.
    pub fn get_vector(position_range: &[IntRange], hashcode: u128) -> Vec<i64> {
        let ranges = normalize_int_ranges(position_range);
        let (offsets, lengths) = parse_int_ranges(&ranges);
        let mut vec = get_vector_from_index(&lengths, hashcode);
        for (v, offset) in vec.iter_mut().zip(offsets.iter()) {
            *v += *offset;
        }
        vec
    }

    /// Generate an integer matrix with the same range for each column.
    pub fn random_matrix(rows: usize, cols: usize, range: IntRange) -> IntVector {
        let ranges = vec![range; cols];
        Self::random_int(rows, &ranges)
    }

    /// Format a vector with a custom separator.
    pub fn format_vector<T: std::fmt::Display>(vec: &[T], sep: &str) -> String {
        vec.iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(sep)
    }

    /// Format a matrix using column and row separators.
    pub fn format_matrix<T: std::fmt::Display>(
        matrix: &[Vec<T>],
        sep: &str,
        row_sep: &str,
    ) -> String {
        matrix
            .iter()
            .map(|row| Self::format_vector(row, sep))
            .collect::<Vec<_>>()
            .join(row_sep)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_vector() {
        let ranges = [IntRange::Max(2), IntRange::MinMax(3, 4)];
        assert_eq!(Vector::get_vector(&ranges, 0), vec![0, 3]);
        assert_eq!(Vector::get_vector(&ranges, 1), vec![1, 3]);
    }

    #[test]
    fn test_random_int_bounds() {
        let ranges = [IntRange::Max(1)];
        let vectors = Vector::random_int(3, &ranges);
        assert_eq!(vectors.len(), 3);
        for v in vectors {
            assert_eq!(v.len(), 1);
            assert!(v[0] >= 0 && v[0] <= 1);
        }
    }

    #[test]
    fn test_random_unique_vectors() {
        let ranges = [IntRange::Max(1), IntRange::Max(1)];
        let vectors = Vector::random_unique_vector(4, &ranges);
        let mut set = std::collections::HashSet::new();
        for v in vectors {
            set.insert(v);
        }
        assert_eq!(set.len(), 4);
    }

    #[test]
    fn test_formatting_helpers() {
        let v = vec![1, 2, 3];
        assert_eq!(Vector::format_vector(&v, ","), "1,2,3");
        let m = vec![vec![1, 2], vec![3, 4]];
        assert_eq!(Vector::format_matrix(&m, " ", "\n"), "1 2\n3 4");
    }
}
