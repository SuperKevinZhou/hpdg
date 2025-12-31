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
    initial: HashMap<usize, T>,
}

impl<T, F> Sequence<T, F>
where
    T: Clone,
    F: Fn(usize, &dyn Fn(usize) -> T) -> T,
{
    pub fn new(formula: F) -> Self {
        Self {
            formula,
            initial: HashMap::new(),
        }
    }

    pub fn with_initial<I>(formula: F, initial: I) -> Self
    where
        I: Into<SequenceInit<T>>,
    {
        let initial = initial.into().into_map();
        Self { formula, initial }
    }
}
