use std::iter::iterate;
use std::num::Float;

#[allow(dead_code)]
#[allow(unstable)]
fn e2() -> usize {
    iterate((1, 1), |(a, b)| (a + b, a)).
        map(|(a, _)| a).
        take_while(|&a| a < 4_000_000).
        filter(|&a| a % 2 == 0).
        fold(0, |acc, a| acc + a)
}

pub struct Primes {
    primes: Vec<usize>,
}

impl Primes {
    fn new() -> Primes {
        Primes { primes: vec![2, 3] }
    }

    fn next(&mut self) -> usize {
        let mut i = self.primes[self.primes.len() - 1];
        loop {
            i += 2;
            let max = (i as f64).sqrt().ceil() as usize;
            if !self.primes.iter().take_while(|&&p| p <= max).any(|&p| i % p == 0) {
                break;
            }
        }
        self.primes.push(i);
        return i;
    }

    fn iter(&mut self) -> PrimesIter {
        PrimesIter { primes: self, index: 0 }
    }
}

pub struct PrimesIter<'a> {
    primes: &'a mut Primes,
    index: usize,
}

impl<'a> Iterator for PrimesIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let result = if self.primes.primes.len() <= self.index {
            self.primes.next()
        } else {
            self.primes.primes[self.index]
        };
        self.index += 1;
        return Some(result);
    }
}

fn e3(n: usize) -> usize {
    let mut primes = Primes::new();
    let mut x = n;
    loop {
        let p = primes.iter().find(|&p| x % p == 0).unwrap();
        if p == x { break; }
        x = x / p;
    }
    return x;
}

fn main() {
    println!("{}", e3(600851475143));
}
