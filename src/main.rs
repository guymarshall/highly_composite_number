mod divisors;
mod file;

use divisors::count_divisors;
use file::{read_results, write_results};
use thousands::Separable;

fn main() {
    let (mut number, mut max_divisors): (i32, i32) = read_results();

    println!(
        "Starting values:\nNumber: {}, Divisor count: {}\n",
        number.separate_with_commas(),
        max_divisors.separate_with_commas()
    );

    loop {
        let divisors: i32 = count_divisors(number);
        if divisors > max_divisors {
            max_divisors = divisors;

            println!(
                "\rNumber: {}, Divisor count: {}",
                number.separate_with_commas(),
                max_divisors.separate_with_commas()
            );
            write_results(number, max_divisors);
        }

        number += 1;
    }
}
