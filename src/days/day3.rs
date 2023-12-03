use std::fs::File;
use std::io::BufRead;

use crate::days::Day;

pub struct Day3 {
    engine_schematic: Vec<Vec<EngineElement>>,
}

impl Day3 {
    fn construct_engine_numbers(&self) -> Vec<EngineNumber> {
        let mut engine_numbers = Vec::with_capacity(self.engine_schematic.len() * 5);
        for row in 0..self.engine_schematic.len() {
            let mut cur_num = None;
            let mut accum = Vec::with_capacity(self.engine_schematic[0].len());

            for column in 0..self.engine_schematic[0].len() {
                match (cur_num, &self.engine_schematic[row][column]) {
                    (None, EngineElement::Digit(d)) => {
                        cur_num = Some((column, *d));
                    }
                    (Some((col_num, num)), EngineElement::Digit(d)) => {
                        cur_num = Some((col_num, 10 * num + *d));
                    }
                    (None, _) => {}
                    (Some((col_num, num)), _) => {
                        cur_num = None;
                        accum.push(EngineNumber {
                            value: num,
                            row: row as i32,
                            start_col: col_num as i32,
                            end_col: column as i32 - 1,
                            marked: false,
                        });
                    }
                }
            }

            if let Some((col_num, num)) = cur_num {
                accum.push(EngineNumber {
                    value: num,
                    row: row as i32,
                    start_col: col_num as i32,
                    end_col: self.engine_schematic[0].len() as i32 - 1,
                    marked: false,
                });
            }

            engine_numbers.append(&mut accum);
        }
        engine_numbers
    }
}

#[derive(Debug)]
enum EngineElement {
    Digit(u32),
    Symbol(char),
    Period,
}

#[derive(Debug)]
struct EngineNumber {
    value: u32,
    row: i32,
    start_col: i32,
    end_col: i32,
    marked: bool,
}

impl EngineNumber {
    fn is_next_to(&self, row: i32, col: i32) -> bool {
        (self.row - 1..=self.row + 1).contains(&row)
            && (self.start_col - 1..=self.end_col + 1).contains(&col)
    }

    fn try_mark(&mut self, row: i32, col: i32) {
        self.marked |= self.is_next_to(row, col);
    }
}
impl From<char> for EngineElement {
    fn from(value: char) -> Self {
        match value {
            '.' => EngineElement::Period,
            '0'..='9' => EngineElement::Digit(value.to_digit(10).unwrap()),
            c => EngineElement::Symbol(c),
        }
    }
}

impl Day for Day3 {
    fn make_day(file: File) -> Self {
        let engine_schematic = std::io::BufReader::new(file)
            .lines()
            .map(|line| {
                line.expect("doc should have lines")
                    .chars()
                    .map(|ch| ch.into())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Day3 { engine_schematic }
    }

    fn solution1(&self) -> String {
        let mut engine_numbers = self.construct_engine_numbers();

        self.engine_schematic
            .iter()
            .enumerate()
            .for_each(|(row, line)| {
                line.iter()
                    .enumerate()
                    .filter(|(_, element)| matches!(element, EngineElement::Symbol(_)))
                    .for_each(|(col, _)| {
                        engine_numbers.iter_mut().for_each(|engine_number| {
                            engine_number.try_mark(row as i32, col as i32)
                        });
                    });
            });

        let result: u32 = engine_numbers
            .iter()
            .filter(|engine_number| engine_number.marked)
            .map(|engine_number| engine_number.value)
            .sum();
        result.to_string()
    }

    fn solution2(&self) -> String {
        let engine_numbers = self.construct_engine_numbers();

        let result: u32 = self
            .engine_schematic
            .iter()
            .enumerate()
            .map(|(row, line)| {
                line.iter()
                    .enumerate()
                    .filter(|(_, element)| matches!(element, EngineElement::Symbol('*')))
                    .filter_map(|(col, _)| {
                        let close_numbers_to_symbol = engine_numbers
                            .iter()
                            .filter(|engine_number| {
                                engine_number.is_next_to(row as i32, col as i32)
                            })
                            .collect::<Vec<_>>();
                        if close_numbers_to_symbol.len() == 2 {
                            Some(
                                close_numbers_to_symbol
                                    .iter()
                                    .map(|engine_number| engine_number.value)
                                    .product::<u32>(),
                            )
                        } else {
                            None
                        }
                    })
                    .sum::<u32>()
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
        let input = File::open("./inputs/day3/input_test.txt").expect("File not found");
        let day3 = Day3::make_day(input);
        assert_eq!(day3.solution1(), "4361");
    }

    #[test]
    fn test_day1_sol2() {
        let input = File::open("./inputs/day3/input_test.txt").expect("File not found");
        let day3 = Day3::make_day(input);
        assert_eq!(day3.solution2(), "467835");
    }
}
