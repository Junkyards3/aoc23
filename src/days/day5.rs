use std::fs::File;
use std::io::BufRead;
use std::str::FromStr;

use crate::days::Day;

type Quantity = u32;

pub struct Day5 {
    start_seeds: Vec<Quantity>,
    transformers: Vec<Transformer>,
}

impl Day5 {
    fn get_range_seeds(&self) -> Vec<Quantity> {
        self.start_seeds
            .chunks(2)
            .flat_map(|chunk| (chunk[0]..chunk[0] + chunk[1]))
            .collect()
    }
}
#[derive(Debug)]
struct Transformer {
    transformer_lines: Vec<TransformerLine>,
}

impl Transformer {
    fn convert_quantity(&self, quantity: Quantity) -> Quantity {
        for transformer_line in self.transformer_lines.iter() {
            if quantity >= transformer_line.source_start
                && quantity < transformer_line.source_start + transformer_line.length
            {
                return quantity + transformer_line.destination_start
                    - transformer_line.source_start;
            }
        }
        quantity
    }
}

#[derive(Debug)]
struct TransformerLine {
    destination_start: Quantity,
    source_start: Quantity,
    length: Quantity,
}

impl FromStr for TransformerLine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers = s
            .split_whitespace()
            .map(|nb_str| nb_str.parse().unwrap())
            .collect::<Vec<_>>();
        let result = TransformerLine {
            destination_start: numbers[0],
            source_start: numbers[1],
            length: numbers[2],
        };
        Ok(result)
    }
}

impl Day for Day5 {
    fn make_day(file: File) -> Self {
        let document_lines = std::io::BufReader::new(file)
            .lines()
            .map(|line| line.expect("doc should have lines"))
            .collect::<Vec<_>>();
        let start_seeds = document_lines[0]
            .split_once(": ")
            .expect("should have colon")
            .1
            .split_whitespace()
            .map(|nb_str| nb_str.parse().expect("should be seed number"))
            .collect();
        let mut index = 1;
        let mut transformers = Vec::with_capacity(6);
        while index < document_lines.len() {
            if document_lines[index].is_empty() {
                index += 1;
            } else {
                index += 1;
                let mut transformer_lines = Vec::with_capacity(20);
                while index < document_lines.len() && !document_lines[index].is_empty() {
                    transformer_lines.push(
                        document_lines[index]
                            .parse()
                            .expect("should be parsable as transformer line"),
                    );
                    index += 1;
                }
                transformers.push(Transformer { transformer_lines });
            }
        }
        Day5 {
            start_seeds,
            transformers,
        }
    }

    fn solution1(&self) -> String {
        let result = self
            .start_seeds
            .iter()
            .map(|seed| {
                self.transformers
                    .iter()
                    .fold(*seed, |curr_quantity, transformer| {
                        transformer.convert_quantity(curr_quantity)
                    })
            })
            .min()
            .expect("there is at least one seed");
        result.to_string()
    }

    fn solution2(&self) -> String {
        let result = self
            .get_range_seeds()
            .iter()
            .map(|seed| {
                self.transformers
                    .iter()
                    .fold(*seed, |curr_quantity, transformer| {
                        transformer.convert_quantity(curr_quantity)
                    })
            })
            .min()
            .expect("there is at least one seed");
        result.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day5_sol1() {
        let input = File::open("./inputs/day5/input_test.txt").expect("File not found");
        let day5 = Day5::make_day(input);
        assert_eq!(day5.solution1(), "35");
    }

    #[test]
    fn test_day5_sol2() {
        let input = File::open("./inputs/day5/input_test.txt").expect("File not found");
        let day5 = Day5::make_day(input);
        assert_eq!(day5.solution2(), "46");
    }
}
