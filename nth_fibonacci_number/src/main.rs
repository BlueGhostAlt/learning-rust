use std::io;

const PHI: f64 = 1.618033988749895;
const SQRT_OF_FIVE: f64 = 2.23606797749979;

fn fib(n: i32) -> i64 {
    ((PHI.powi(n) - (-PHI).powi(-n)) / SQRT_OF_FIVE).trunc() as i64
}

fn main() {
    loop {
        println!("Please input which nth fibonacci number you want to calculate");

        let mut n = String::new();

        io::stdin()
            .read_line(&mut n)
            .expect("Failed to read line..");

        let n: i32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your nth fibonacci number is {}", fib(n));
        break;
    }
}
