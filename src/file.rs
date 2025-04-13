use std::io::Write;
use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader},
};

const RESULTS_FILE: &str = "results.txt";

pub(crate) fn read_results() -> (i32, i32) {
    if let Ok(file) = File::open(RESULTS_FILE) {
        let reader = BufReader::new(file);
        for line in reader.lines().map_while(Result::ok) {
            let parts: Vec<&str> = line.trim().split(',').collect();
            if parts.len() == 2 {
                if let (Ok(last_number), Ok(max_divisors)) = (parts[0].parse(), parts[1].parse()) {
                    return (last_number, max_divisors);
                }
            }
        }
    }
    (0, 0) // Default start values if file is missing or malformed
}

pub(crate) fn write_results(number: i32, max_divisors: i32) {
    let mut file: File = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(RESULTS_FILE)
        .unwrap();
    writeln!(file, "{},{}", number, max_divisors).unwrap();
}
