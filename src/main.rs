extern crate num;

mod primes;
mod stutter;
mod early;
mod e11;
mod e13;
mod e14;

fn main() {
    println!("{}", e14::e14(1_000_000));
}
