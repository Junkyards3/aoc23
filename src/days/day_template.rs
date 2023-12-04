use std::fs::File;
use std::io::BufRead;

use crate::days::Day;

pub struct Day0 {
    data: Vec<String>,
}

impl Day for Day0 {
    fn make_day(file: File) -> Self {
        let data = std::io::BufReader::new(file)
            .lines()
            .map(|line| line.expect("doc should have lines"))
            .collect();
        Day0 { data }
    }

    fn solution1(&self) -> String {
        "sol1".to_string()
    }

    fn solution2(&self) -> String {
        "sol2".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day0_sol1() {
        let input = File::open("./inputs/day0/input_test.txt").expect("File not found");
        let day = Day0::make_day(input);
        assert_eq!(day.solution1(), "sol1");
    }

    #[test]
    fn test_day0_sol2() {
        let input = File::open("./inputs/day0/input_test.txt").expect("File not found");
        let day = Day0::make_day(input);
        assert_eq!(day.solution2(), "sol2");
    }
}
