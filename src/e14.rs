pub struct Collatz(usize);

impl Iterator for Collatz {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        match *self {
            Collatz(0) => None,
            Collatz(1) => { self.0 = 0; Some(1) },
            Collatz(n) => {
                self.0 = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
                Some(n)
            }
        }
    }
}

#[allow(unstable)]
#[allow(dead_code)]
pub fn e14(bound: usize) -> usize {
    let (_, n) = (0..bound).map(|n| (Collatz(n).count(), n)).max().unwrap();
    n
}
