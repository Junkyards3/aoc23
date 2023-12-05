use std::fs::File;

pub(crate) mod day1;
pub(crate) mod day2;
pub(crate) mod day3;
pub(crate) mod day4;
pub(crate) mod day5;

pub trait Day {
    fn make_day(file: File) -> Self;

    fn solution1(&self) -> String;
    fn solution2(&self) -> String;
}
