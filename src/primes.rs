use std::num::Float;
use std::ops::Index;
use std::iter;
use stutter::IteratorStutter;

pub struct Primes {
    cache: Vec<usize>,
}

impl Primes {
    pub fn new() -> Primes {
        Primes{cache: vec![2, 3]}
    }

    pub fn factor(&mut self, n: usize) -> usize {
        self.iter().find(|&p| n % p == 0).unwrap()
    }

    pub fn factors(&mut self, n: usize) -> Factors {
        Factors { primes: self, value: n }
    }

    pub fn ndivisors(&mut self, n: usize) -> usize {
        self.factors(n).unstutter_count().fold(1, |acc, (_, a)| acc * (a + 1))
    }

    #[allow(unstable)]
    pub fn get(&mut self, i: usize) -> usize {
        let cache = &mut self.cache;
        let i_max = cache.len() - 1;
        if i_max >= i {
            cache[i]
        } else {
            (i_max..i).fold(cache[i_max], |p, _| {
                let p1 = iter::count(p + 2, 2).find(|&i| {
                    let max = (i as f64).sqrt().ceil() as usize;
                    !cache.iter().
                        take_while(|&&p| p <= max).
                        any(|&p| i % p == 0)
                }).unwrap();
                cache.push(p1);
                p1
            })
        }
    }

    pub fn iter(&mut self) -> Iter {
        Iter { primes: self, index: 0 }
    }
}

pub struct Iter<'a> {
    primes: &'a mut Primes,
    index: usize,
}

impl<'a> Iterator for Iter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let result = self.primes.get(self.index);
        self.index += 1;
        Some(result)
    }
}

pub struct Factors<'a> {
    primes: &'a mut Primes,
    value: usize,
}

impl<'a> Iterator for Factors<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        if self.value == 1 {
            None
        } else {
            let p = self.primes.factor(self.value);
            self.value /= p;
            Some(p)
        }
    }
}
