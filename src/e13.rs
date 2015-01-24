use num::bigint::{BigUint,ToBigUint};
use std::io::{BufferedReader,File};

#[allow(unstable)]
#[allow(dead_code)]
pub fn e13(n: usize) -> usize {
    let path = Path::new("data/e13.txt");
    let mut reader = BufferedReader::new(File::open(&path));
    let sum = reader.lines().
        map(|line| {
            BigUint::parse_bytes(line.unwrap().trim().as_bytes(), 10).unwrap()
        }).
        fold(0.to_biguint().unwrap(), |acc, x| acc + x);
    let digits = format!("{}", sum);
    (&digits)[0..n].parse::<usize>().unwrap()
}
