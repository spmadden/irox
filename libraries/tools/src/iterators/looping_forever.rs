#[must_use]
pub struct LoopingForever<T>
where
    T: Clone,
{
    pub(crate) items: Vec<T>,
    pub(crate) index: usize,
}

impl<I> Iterator for LoopingForever<I>
where
    I: Clone,
{
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        let size = self.items.len();
        if size == 0 {
            return None;
        }
        let item = self.items[self.index].clone();
        self.index = (self.index + 1) % size;

        Some(item)
    }
}