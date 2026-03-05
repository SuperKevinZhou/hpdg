pub trait Generator<T> {
    fn generate(&mut self) -> T;

    fn generate_n(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.generate()).collect()
    }
}

pub trait GraphGenerator {
    fn generate(&mut self) -> crate::graph::Graph;
}

impl<F> GraphGenerator for F
where
    F: FnMut() -> crate::graph::Graph,
{
    fn generate(&mut self) -> crate::graph::Graph {
        self()
    }
}
