use std::cmp;
use std::collections::RingBuf;
use std::io::{File, BufferedReader};
use std::iter::{iterate, repeat};
use std::num::Int;

use primes::Primes;
use stutter::IteratorStutter;

#[allow(unused_imports)]
use e11::e11;

mod primes;
mod e11;
mod stutter;

#[allow(dead_code)]
#[allow(unstable)]
fn e2() -> usize {
    iterate((1, 1), |(a, b)| (a + b, a)).
        map(|(a, _)| a).
        take_while(|&a| a < 4_000_000).
        filter(|&a| a % 2 == 0).
        fold(0, |acc, a| acc + a)
}

#[allow(dead_code)]
fn e3(n: usize) -> usize {
    Primes::new().factors(n).last().unwrap()
}

#[allow(unstable)]
fn count_digits(n: usize) -> usize {
    iterate(n, |n| n / 10).take_while(|&n| n > 0).count()
}

#[allow(unstable)]
fn get_digit(n: usize, i: usize) -> usize {
    (n / (10.pow(i))) % 10
}

#[allow(unstable)]
fn is_palindrome(n: usize) -> bool {
    let len = count_digits(n);
    let i_max = len - 1;
    let m = len / 2;
    (0..m).all(|i| get_digit(n, i) == get_digit(n, i_max - i))
}

#[allow(dead_code)]
#[allow(unstable)]
fn e4(n: usize) -> usize {
    let min = 10.pow(n - 1);
    let max = 10.pow(n);
    (min..max).
        flat_map(|i: usize| (i..max).map(move |j| i * j)).
        filter(|&x: &usize| is_palindrome(x)).
        fold(0, |x, y| cmp::max(x, y))
}

#[allow(dead_code)]
#[allow(unstable)]
fn e5(n: usize) -> usize {
    let max = n + 1;
    let mut primes = Primes::new();
    let mut hist: Vec<usize> = repeat(0).take(max).collect();
    for i in (2..max) {
        for (p, n) in primes.factors(i).unstutter_count() {
            if n > hist[p] { hist[p] = n }
        }
    }
    hist.iter().
        enumerate().
        filter(|&(_, &n)| n > 0).
        fold(1, |a, (f, &n)| {
            a * f.pow(n)
        })
}

#[allow(dead_code)]
#[allow(unstable)]
fn e6(n: usize) -> usize {
    let max = n + 1;
    ((1..max).fold(0, |a, x| a + x).pow(2)
     - (1..max).fold(0, |a, x| a + x.pow(2)))
}

#[allow(dead_code)]
fn e7(n: usize) -> usize {
    Primes::new().get(n - 1)
}

#[allow(dead_code)]
#[allow(unstable)]
fn e8(n: usize) -> usize {
    let path = Path::new("data/e8.txt");
    let mut file = BufferedReader::new(File::open(&path));
    let mut digits = file.chars().
        map(|r| r.unwrap()).
        filter(|c| c.is_digit(10)).
        map(|c| c.to_digit(10).unwrap());
    let mut factors: RingBuf<usize> = repeat(1).take(n).collect();
    let mut result: usize = 1;
    for nd in digits {
        factors.pop_front();
        factors.push_back(nd);
        let product = factors.iter().fold(1, |a, &x| a * x);
        result = cmp::max(result, product);
    }
    result
}

#[allow(dead_code)]
#[allow(unstable)]
fn e9(sum: usize) -> usize {
    let (a, b, c) = (1..((sum + 2) / 3)).flat_map(|a| {
        ((a + 1)..(((sum - a) + 1) / 2)).map(move |b| {
            let c = sum - a - b;
            (a, b, c)
        })
    }).find(|&(a, b, c)| {
        (a * a) + (b * b) == (c * c)
    }).unwrap();
    a * b * c
}

#[allow(dead_code)]
#[allow(unstable)]
fn e10(max: usize) -> usize {
    Primes::new().iter().take_while(|&p| p < max).fold(0, |a, p| a + p)
}

#[allow(dead_code)]
fn e12(d: usize) -> usize {
    let mut primes = Primes::new();
    (1..).scan(0, |prev, i| {
        *prev += i;
        Some(*prev)
    }).find(|&n| {
        primes.ndivisors(n) > d
    }).unwrap()
}

fn main() {
    println!("{}", e5(20));
}
