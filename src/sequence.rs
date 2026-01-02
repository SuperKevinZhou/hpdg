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

    pub fn get_one(&self, i: usize) -> T {
        if let Some(value) = self.values.borrow().get(&i) {
            return value.clone();
        }
        let value = (self.formula)(i, &|idx| self.get_one(idx));
        self.values.borrow_mut().insert(i, value.clone());
        value
    }

    pub fn get_range(&self, left: usize, right: usize) -> Vec<T> {
        if left > right {
            return Vec::new();
        }
        (left..=right).map(|i| self.get_one(i)).collect()
    }
}
