pub trait IteratorExtMore: Iterator + Sized {
    fn take_while_plus1<P>(self, predicate: P) ->
        TakeWhilePlus1<Self::Item, Self, P>
        where P: FnMut(&Self::Item) -> bool,
    {
        TakeWhilePlus1 { iter: self, flag: false, predicate: predicate }
    }
}

impl<I> IteratorExtMore for I where I: Iterator {}

#[derive(Clone,Show)]
pub struct TakeWhilePlus1<A, I, P>
    where I: Iterator<Item=A>, P: FnMut(&A) -> bool
{
    iter: I,
    flag: bool,
    predicate: P,
}

impl<A, I, P> Iterator for TakeWhilePlus1<A, I, P>
    where I: Iterator<Item=A>, P: FnMut(&A) -> bool
{
    type Item = A;

    #[inline]
    fn next(&mut self) -> Option<A> {
        if self.flag {
            None
        } else {
            match self.iter.next() {
                Some(x) => {
                    if (self.predicate)(&x) { self.flag = true }
                    Some(x)
                }
                None => None
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (_, upper) = self.iter.size_hint();
        (0, upper) // can't know a lower bound, due to the predicate
    }
}
