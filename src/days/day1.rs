use std::fs::File;
use std::io::BufRead;

use crate::days::Day;

const EXTRACT_WORDS: [(&str, u32); 18] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
];

pub struct Day1 {
    document_lines: Vec<String>,
}

impl Day1 {}

fn get_number_from_line(line: &str) -> Result<u32, ()> {
    let mut digits = line.chars().filter(|c| c.is_numeric());
    let first_digit = digits.next().ok_or(())?.to_digit(10).ok_or(())?;
    let last_digit = match digits.last() {
        None => first_digit,
        Some(c) => c.to_digit(10).ok_or(())?,
    };
    let result = 10 * first_digit + last_digit;
    Ok(result)
}

fn get_number_from_line2(line: &str) -> Result<u32, ()> {
    let digits = extract_numbers_from_string(line, &EXTRACT_WORDS);
    let first_digit = digits.first().ok_or(())?;
    let last_digit = match digits.last() {
        None => first_digit,
        Some(c) => c,
    };
    let result = 10 * first_digit + last_digit;
    Ok(result)
}

fn extract_numbers_from_string<'a>(line: &str, to_extract: &'a [(&str, u32)]) -> Vec<u32> {
    let mut cut_line = line.to_string();
    let mut extracted = vec![];
    while !cut_line.is_empty() {
        let extract = to_extract
            .iter()
            .find(|(word, _)| cut_line.starts_with(*word));
        if let Some((_, value)) = extract {
            extracted.push(*value)
        }

        cut_line.remove(0);
    }
    extracted
}

impl Day for Day1 {
    fn make_day(file: File) -> Self {
        let document_lines = std::io::BufReader::new(file)
            .lines()
            .map(|line| line.expect("doc should have lines"))
            .collect();
        Day1 { document_lines }
    }

    fn solution1(&self) -> String {
        let result: u32 = self
            .document_lines
            .iter()
            .map(|line| {
                get_number_from_line(line)
                    .expect(&format!("there should be two digits on line : {}", line))
            })
            .sum();
        result.to_string()
    }

    fn solution2(&self) -> String {
        let result: u32 = self
            .document_lines
            .iter()
            .map(|line| {
                get_number_from_line2(line)
                    .expect(&format!("there should be two digits on line : {}", line))
            })
            .sum();
        result.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1_sol1() {
        let document_lines = vec![
            "1abc2".to_string(),
            "pqr3stu8vwx".to_string(),
            "a1b2c3d4e5f".to_string(),
            "treb7uchet".to_string(),
        ];
        let day1 = Day1 { document_lines };
        assert_eq!(day1.solution1(), "142");
    }

    #[test]
    fn test_day1_sol2() {
        let document_lines = vec![
            "two1nine".to_string(),
            "eightwothree".to_string(),
            "abcone2threexyz".to_string(),
            "xtwone3four".to_string(),
            "4nineeightseven2".to_string(),
            "zoneight234".to_string(),
            "7pqrstsixteen".to_string(),
        ];
        let day1 = Day1 { document_lines };
        assert_eq!(day1.solution2(), "281");
    }
}
