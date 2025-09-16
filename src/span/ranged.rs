pub trait Ranged<T> {
    fn range(&self) -> std::ops::Range<T>;
}

impl Ranged<usize> for std::ops::Range<usize> {
    fn range(&self) -> Self {
        self.clone()
    }
}
