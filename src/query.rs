#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
