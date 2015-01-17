use std::num::Float;
use std::ops::Index;
use std::iter;

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

    pub fn factors(&mut self, n: usize) -> Vec<usize> {
        let mut ret = Vec::new();
        let mut n = n;
        while n > 1 {
            let p = self.factor(n);
            ret.push(p);
            n /= p;
        }
        ret
    }

    #[allow(unstable)]
    fn get(&mut self, i: usize) -> usize {
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
