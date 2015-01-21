use std::mem;

pub trait IteratorStutter: Iterator + Sized {
    fn unstutter<T>(self) -> Unstutter<Self, T> where
        Self: Iterator<Item=T>,
        T: Eq,
    {
        Unstutter { iter: self, flag: false, next: None }
    }

    fn unstutter_count<T>(self) -> UnstutterCount<Self, T> where
        Self: Iterator<Item=T>,
        T: Eq,
    {
        UnstutterCount { iter: self, flag: false, next: None }
    }
}

impl<I, T> IteratorStutter for I where I: Iterator<Item=T>, T: Eq {}

#[derive(Clone,Show)]
pub struct Unstutter<I, T> {
    iter: I,
    flag: bool,
    next: Option<T>,
}

impl<I, T> Iterator for Unstutter<I, T> where
    I: Iterator<Item=T>,
    T: Eq,
{
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        if self.flag {
            None
        } else if self.next.is_none() {
            match self.iter.next() {
                None => { self.flag = true; None }
                Some(x) => { self.next = Some(x); self.next() }
            }
        } else {
            loop {
                let x = self.iter.next();
                if x.is_none() {
                    self.flag = true;
                    return mem::replace(&mut self.next, None);
                } else if x != self.next {
                    return mem::replace(&mut self.next, x);
                }
            }
        }
    }
}

#[derive(Clone,Show)]
pub struct UnstutterCount<I, T> {
    iter: I,
    flag: bool,
    next: Option<T>,
}

impl<I, T> Iterator for UnstutterCount<I, T> where
    I: Iterator<Item=T>,
    T: Eq,
{
    type Item = (T, usize);

    #[inline]
    fn next(&mut self) -> Option<(T, usize)> {
        if self.flag {
            None
        } else if self.next.is_none() {
            match self.iter.next() {
                None => { self.flag = true; None }
                Some(x) => { self.next = Some(x); self.next() }
            }
        } else {
            let mut count = 1us;
            loop {
                let x = self.iter.next();
                if x.is_none() {
                    self.flag = true;
                    let ret = mem::replace(&mut self.next, None).unwrap();
                    return Some((ret, count));
                } else if x != self.next {
                    let ret = mem::replace(&mut self.next, x).unwrap();
                    return Some((ret, count));
                }
                count += 1;
            }
        }
    }
}
