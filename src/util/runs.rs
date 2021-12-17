use std::iter::Peekable;
    
pub struct RunsIterator<T: PartialEq, I: Iterator<Item = T>> {
    peeker: Peekable<I>
}
impl<T: PartialEq, I: Iterator<Item = T>> RunsIterator<T, I> {
    fn from_iter(iter: I) -> RunsIterator<T, I> { RunsIterator { peeker: iter.peekable() } }
}
impl<T: PartialEq, I: Iterator<Item = T>> Iterator for RunsIterator<T, I> {
    type Item = (T, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.peeker.next()?;
        let mut count: usize = 1;
        loop {
            let next = self.peeker.peek();
            if next.map(|x| *x != item).unwrap_or(true) { return Some((item, count)); }
            count += 1;
            self.peeker.next();
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) { (0, self.peeker.size_hint().1) }
}
pub trait RunsTrait<T: PartialEq, I: Iterator<Item = T>> {
    fn runs(self) -> RunsIterator<T, I>;
}
impl<T: PartialEq, I: Iterator<Item = T>> RunsTrait<T, I> for I {
    fn runs(self) -> RunsIterator<T, I> { RunsIterator::from_iter(self) }
}



pub struct DedupIterator<T: PartialEq, I: Iterator<Item = T>> {
    peeker: Peekable<I>
}
impl<T: PartialEq, I: Iterator<Item = T>> DedupIterator<T, I> {
    fn from_iter(iter: I) -> DedupIterator<T, I> { DedupIterator { peeker: iter.peekable() } }
}
impl<T: PartialEq, I: Iterator<Item = T>> Iterator for DedupIterator<T, I> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.peeker.next()?;
        loop {
            let next = self.peeker.peek();
            if next.map(|x| *x != item).unwrap_or(true) { return Some(item); }
            self.peeker.next();
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) { (0, self.peeker.size_hint().1) }
}
pub trait DedupTrait<T: PartialEq, I: Iterator<Item = T>> {
    fn dedup(self) -> DedupIterator<T, I>;
}
impl<T: PartialEq, I: Iterator<Item = T>> DedupTrait<T, I> for I {
    fn dedup(self) -> DedupIterator<T, I> { DedupIterator::from_iter(self) }
}