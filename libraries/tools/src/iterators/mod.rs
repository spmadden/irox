use self::looping_forever::LoopingForever;

pub mod looping_forever;

pub trait Itertools: Iterator {
    ///
    /// Returns an iterator that never ends, sequentially looping over all items in this
    /// iterator forever, unless there are no items in the iterator.
    fn looping_forever(self) -> LoopingForever<Self::Item>
    where
        Self: Sized + Iterator,
        Self::Item: Clone,
    {
        looping_forever::LoopingForever {
            index: 0,
            items: self.collect(),
        }
    }
}

impl<T: ?Sized> Itertools for T where T: Iterator {}
