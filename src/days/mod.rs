use std::fs::File;

pub(crate) mod day1;
pub(crate) mod day10;
pub(crate) mod day11;
pub(crate) mod day12;
pub(crate) mod day13;
pub(crate) mod day14;
pub(crate) mod day15;
pub(crate) mod day16;
pub(crate) mod day17;
pub(crate) mod day18;
pub(crate) mod day2;
pub(crate) mod day3;
pub(crate) mod day4;
pub(crate) mod day5;
pub(crate) mod day6;
pub(crate) mod day7;
pub(crate) mod day8;
pub(crate) mod day9;

pub trait Day {
    fn make_day(file: File) -> Self;

    fn solution1(&self) -> String;
    fn solution2(&self) -> String;
}
