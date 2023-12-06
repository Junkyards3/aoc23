use roots::{find_roots_quadratic, Roots};
use std::fs::File;
use std::io::BufRead;

use crate::days::Day;

type Time = i64;
type Distance = i64;

pub struct Day6 {
    times: Vec<Time>,
    distances: Vec<Distance>,
}

fn get_inner_interval_size(race_time: Time, race_distance: Distance) -> Time {
    match find_roots_quadratic(1f64, -(race_time as f64), race_distance as f64) {
        Roots::Two([a, b]) => b.ceil() as Time - a.floor() as Time - 1,
        _ => panic!("should be a way to beat the course"),
    }
}

impl Day for Day6 {
    fn make_day(file: File) -> Self {
        let mut data = std::io::BufReader::new(file).lines().map(|line| {
            line.expect("doc should have lines")
                .split_once(":")
                .expect("should have colon")
                .1
                .split_whitespace()
                .map(|c| c.parse().expect("expected number"))
                .collect()
        });
        let times = data.next().unwrap();
        let distances = data.next().unwrap();
        Day6 { times, distances }
    }

    fn solution1(&self) -> String {
        let result = self
            .times
            .iter()
            .zip(self.distances.iter())
            .map(|(time, distance)| get_inner_interval_size(*time, *distance))
            .product::<Time>();
        result.to_string()
    }

    fn solution2(&self) -> String {
        let time: Time = self
            .times
            .iter()
            .map(|time| time.to_string())
            .reduce(|t1, t2| t1 + &t2)
            .expect("there is an elem")
            .parse()
            .expect("is a number");
        let distance: Distance = self
            .distances
            .iter()
            .map(|distance| distance.to_string())
            .reduce(|d1, d2| d1 + &d2)
            .expect("there is an elem")
            .parse()
            .expect("is a number");
        get_inner_interval_size(time, distance).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day6_sol1() {
        let input = File::open("./inputs/day6/input_test.txt").expect("File not found");
        let day = Day6::make_day(input);
        assert_eq!(day.solution1(), "288");
    }

    #[test]
    fn test_day6_sol2() {
        let input = File::open("./inputs/day6/input_test.txt").expect("File not found");
        let day = Day6::make_day(input);
        assert_eq!(day.solution2(), "71503");
    }
}
