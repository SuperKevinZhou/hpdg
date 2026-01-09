use rand::Rng;

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

impl Vector {
    pub fn random_int(num: usize, position_range: &[IntRange]) -> IntVector {
        let ranges_owned;
        let ranges = if position_range.is_empty() {
            ranges_owned = vec![IntRange::Max(10)];
            &ranges_owned
        } else {
            position_range
        };

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
}
