use std::io::Write;
use std::str::FromStr;

fn main() {
    let mut numbers = Vec::new();

    for arg in std::env::args().skip(1) {
        numbers.push(u64::from_str(&arg)
            .expect("error parsing arg"));
    }

    if numbers.len() == 0 {
        writeln!(std::io::stderr(), "Usage : gcd number n ").unwrap();
        std::process::exit(1);
    }

    let mut d = numbers[0];

    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("gcd of {:?} is {}", numbers, d);
}


fn gcd(mut a: u64, mut b: u64) -> u64 {
    if a < b {
        let tmp = a;
        a = b;
        b = tmp;
    }

    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}