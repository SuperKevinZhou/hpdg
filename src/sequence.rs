use std::cell::RefCell;
use std::collections::HashMap;

/// Initial values used by a sequence.
pub enum SequenceInit<T> {
    List(Vec<T>),
    Map(HashMap<usize, T>),
}

impl<T> SequenceInit<T> {
    fn into_map(self) -> HashMap<usize, T> {
        match self {
            SequenceInit::List(values) => values.into_iter().enumerate().collect(),
            SequenceInit::Map(map) => map,
        }
    }
}

impl<T> From<Vec<T>> for SequenceInit<T> {
    fn from(values: Vec<T>) -> Self {
        SequenceInit::List(values)
    }
}

impl<T> From<HashMap<usize, T>> for SequenceInit<T> {
    fn from(values: HashMap<usize, T>) -> Self {
        SequenceInit::Map(values)
    }
}

/// Formula-driven sequence generator.
pub struct Sequence<T, F>
where
    F: Fn(usize, &dyn Fn(usize) -> T) -> T,
{
    formula: F,
    values: RefCell<HashMap<usize, T>>,
}

impl<T, F> Sequence<T, F>
where
    T: Clone,
    F: Fn(usize, &dyn Fn(usize) -> T) -> T,
{
    pub fn new(formula: F) -> Self {
        Self {
            formula,
            values: RefCell::new(HashMap::new()),
        }
    }

    pub fn with_initial<I>(formula: F, initial: I) -> Self
    where
        I: Into<SequenceInit<T>>,
    {
        let initial = initial.into().into_map();
        Self {
            formula,
            values: RefCell::new(initial),
        }
    }

    /// Retrieve a single element (memoized).
    pub fn get_one(&self, i: usize) -> T {
        if let Some(value) = self.values.borrow().get(&i) {
            return value.clone();
        }
        let value = (self.formula)(i, &|idx| self.get_one(idx));
        self.values.borrow_mut().insert(i, value.clone());
        value
    }

    /// Retrieve a range [left, right] (inclusive).
    pub fn get_range(&self, left: usize, right: usize) -> Vec<T> {
        if left > right {
            return Vec::new();
        }
        (left..=right).map(|i| self.get_one(i)).collect()
    }

    /// Retrieve elements by an index slice.
    pub fn get_slice(&self, indices: &[usize]) -> Vec<T> {
        indices.iter().map(|&i| self.get_one(i)).collect()
    }

    /// Iterate over a range [left, right] (inclusive).
    pub fn iter_range(&self, left: usize, right: usize) -> SequenceRangeIter<'_, T, F> {
        SequenceRangeIter {
            sequence: self,
            current: left,
            end: right,
        }
    }
}

pub struct SequenceRangeIter<'a, T, F>
where
    F: Fn(usize, &dyn Fn(usize) -> T) -> T,
{
    sequence: &'a Sequence<T, F>,
    current: usize,
    end: usize,
}

impl<'a, T, F> Iterator for SequenceRangeIter<'a, T, F>
where
    T: Clone,
    F: Fn(usize, &dyn Fn(usize) -> T) -> T,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current > self.end {
            return None;
        }
        let value = self.sequence.get_one(self.current);
        self.current += 1;
        Some(value)
    }
}

/// Helper for arithmetic progressions.
/// Example: arithmetic_sequence(1, 2) -> 1, 3, 5, ...
pub fn arithmetic_sequence(
    start: i64,
    diff: i64,
) -> Sequence<i64, impl Fn(usize, &dyn Fn(usize) -> i64) -> i64> {
    Sequence::new(move |i, _| start + diff * i as i64)
}

/// Helper for geometric progressions.
/// Example: geometric_sequence(1, 3) -> 1, 3, 9, ...
pub fn geometric_sequence(
    start: i64,
    ratio: i64,
) -> Sequence<i64, impl Fn(usize, &dyn Fn(usize) -> i64) -> i64> {
    Sequence::new(move |i, _| start * ratio.pow(i as u32))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence_basic_formula() {
        let seq = Sequence::new(|i, _| 2 * i as i64 + 3);
        assert_eq!(seq.get_one(1), 5);
        assert_eq!(seq.get_range(1, 3), vec![5, 7, 9]);
    }

    #[test]
    fn test_sequence_recursive_formula() {
        let seq = Sequence::with_initial(|i, f| f(i - 1) + 1, vec![0i64]);
        assert_eq!(seq.get_range(0, 4), vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_sequence_helpers() {
        let ar = arithmetic_sequence(1, 2);
        assert_eq!(ar.get_range(0, 3), vec![1, 3, 5, 7]);
        let geo = geometric_sequence(1, 3);
        assert_eq!(geo.get_range(0, 4), vec![1, 3, 9, 27, 81]);
    }
}
