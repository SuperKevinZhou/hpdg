pub trait Generator<T> {
    fn generate(&mut self) -> T;

    fn generate_n(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.generate()).collect()
    }
}
