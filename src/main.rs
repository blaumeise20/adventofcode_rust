#![feature(destructuring_assignment)]
#![feature(associated_type_defaults)]
#![feature(box_syntax)]
use std::{env::args, fmt::Display, fs::File, io::Read, time::Instant};
mod util;

mod _2021;

fn main() {
    let year = args().nth(1).expect("specify year");
    let day = args().nth(2).expect("specify day");
    let year = year.parse::<i32>().expect("invalid year");
    let day = day.parse::<i32>().expect("invalid day");

    println!("\x1b[1;34mAdvent of Code - {} day {}\x1b[m", year, day);
    println!();

    let mut instance = match year {
        2021 => _2021::new(day).expect("Day not found"),
        _ => panic!("Year {} not known", year),
    };

    let mut input = File::open(format!("input/{}/day_{}.txt", year, day)).expect("no input found");
    let mut input_str = String::new();
    input.read_to_string(&mut input_str).expect("could not read input");
    let input = input_str.as_str();

    // execute part 1
    println!("\x1b[1;32mPart 1\x1b[m");
    let start = Instant::now();
    let result = instance.part_1(input);
    let duration = start.elapsed();
    println!("\x1b[1mResult:\x1b[m \x1b[4;33m{}\x1b[m", result);
    log("Total Time", ((duration.as_nanos() as f64) / 1_000_000f64).to_string() + " ms");

    // execute part 2
    println!();
    println!("\x1b[1;32mPart 2\x1b[m");
    let start = Instant::now();
    let result = instance.part_2(input);
    let duration = start.elapsed();
    println!("\x1b[1mResult:\x1b[m \x1b[4;33m{}\x1b[m", result);
    log("Total Time", ((duration.as_nanos() as f64) / 1_000_000f64).to_string() + " ms");
}

pub fn log<T: Display>(label: &str, value: T) {
    println!("\x1b[1m{}:\x1b[m {}", label, value);
}
pub fn debug() {
    log("Debug", "Debug");
}

pub trait Day {
    fn part_1(&mut self, input: &str) -> String;
    #[allow(unused_variables)]
    fn part_2(&mut self, input: &str) -> String { "".to_string() }
}
