#![allow(dead_code)]

fn main() {
    for n in 0..=255 {
        println!("fib_iter({}) -> {}", n, fib_iter(n))
    }
}

fn fib(n: u8) -> u128 {
    match n {
        0 | 1 => n as u128,
        _ => fib(n - 1) + fib(n - 2),
    }
}

fn fib_iter(n: u8) -> f64 {
    let mut a = 0.0;
    let mut b = 1.0;

    for _ in 0..n {
        let prev = b;
        b += a;
        a = prev;
    }

    a
}
