mod divisors;

use std::io::{self, Write};

use divisors::count_divisors;
use thousands::Separable;

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
