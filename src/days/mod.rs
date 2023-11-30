use std::fs::File;

pub(crate) mod day1;

pub trait Day {
    fn make_day(file: File) -> Self;

    fn solution1(&self) -> String;
    fn solution2(&self) -> String;
}
