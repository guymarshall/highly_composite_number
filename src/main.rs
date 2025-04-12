use std::io::{self, Write};

use thousands::Separable;

fn count_divisors(mut number: i32) -> i32 {
    let mut divisor_count: i32 = 1;
    let mut current_divisor: i32 = 2;
    while current_divisor * current_divisor <= number {
        let mut exponent: i32 = 0;
        while number % current_divisor == 0 {
            number /= current_divisor;
            exponent += 1;
        }
        if exponent > 0 {
            divisor_count *= exponent + 1;
        }
        current_divisor += 1;
    }
    if number > 1 {
        divisor_count *= 2;
    }
    divisor_count
}

fn main() {
    let mut max_divisors: i32 = 0;
    for number in 1..=10_000_000 {
        let divisors: i32 = count_divisors(number);
        if divisors > max_divisors {
            max_divisors = divisors;
            print!(
                "\rNumber: {}, Divisor count: {}",
                number.separate_with_commas(),
                max_divisors.separate_with_commas()
            );
            io::stdout().flush().unwrap();
        }
    }
    println!();
}
