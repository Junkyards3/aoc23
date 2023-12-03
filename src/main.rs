use crate::days::{day1::Day1, day2::Day2, day3::Day3, Day};
use std::fs::File;
use std::time::{Duration, Instant};

mod days;

fn time_function<T>(f: impl FnOnce() -> T) -> (T, Duration) {
    let now = Instant::now();
    (f(), now.elapsed())
}

fn choose_unit(duration: Duration) -> (u128, String) {
    if duration < Duration::from_millis(10) {
        (duration.as_micros(), "µs".to_string())
    } else if duration < Duration::from_secs(10) {
        (duration.as_millis(), "ms".to_string())
    } else {
        (duration.as_secs() as u128, "s".to_string())
    }
}

fn run_day(day: impl Day, duration: Duration) {
    let (time_parse, unit_parse) = choose_unit(duration);
    println!("Parse time : {}{}\n", time_parse, unit_parse);

    let (result1, duration1) = time_function(|| day.solution1());
    let (time1, unit1) = choose_unit(duration1);
    println!("Solution 1 : {} ({}{})", result1, time1, unit1);

    let (result2, duration2) = time_function(|| day.solution2());
    let (time2, unit2) = choose_unit(duration2);
    println!("Solution 2 : {} ({}{})", result2, time2, unit2);
}

macro_rules! run_day {
    ($day_struct:ty, $input:ident) => {
        let (day, duration) = time_function(move || <$day_struct>::make_day($input));
        run_day(day, duration);
    };
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
        "1" => {
            run_day!(Day1, input);
        }
        "2" => {
            run_day!(Day2, input);
        }
        "3" => {
            run_day!(Day3, input);
        }
        _ => panic!("day not found"),
    };
}
