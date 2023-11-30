use crate::days::{day1::Day1, Day};
use std::fs::File;
use std::time::{Duration, Instant};

mod days;

fn time_function(f: impl Fn() -> String) -> (String, Duration) {
    let now = Instant::now();
    (f(), now.elapsed())
}

fn run_day(day: impl Day) {
    let (result1, duration1) = time_function(|| day.solution1());
    println!("The result is {}", result1);
    println!(
        "It took {} microseconds to run the function !",
        duration1.as_micros()
    );

    let (result2, duration2) = time_function(|| day.solution2());
    println!("The result is {}", result2);
    println!(
        "It took {} microseconds to run the function !",
        duration2.as_micros()
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let day_number = args.get(1).expect("No day specified").as_str();
    let use_test_input = args.get(2).map_or(false, |s| s == "test");

    let path_input = if use_test_input {
        format!("./inputs/day{}/input_test.txt", day_number)
    } else {
        format!("./inputs/day{}/input.txt", day_number)
    };

    let input = File::open(path_input).expect("File not found");

    match day_number {
        "1" => run_day(Day1::make_day(input)),
        _ => panic!("No day specified"),
    };
}
