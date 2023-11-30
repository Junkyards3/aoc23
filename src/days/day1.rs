use crate::days::Day;
use std::fs::File;
use std::io::BufRead;

pub struct Day1 {
    _data: ()
}

impl Day for Day1 {
    fn make_day(file: File) -> Self {
        Day1 { _data: ()  }
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
    fn test_day1() {
        let day1 = Day1 { _data : () };
        assert_eq!(day1.solution1(), "sol1");
        assert_eq!(day1.solution2(), "sol2");
    }

}
