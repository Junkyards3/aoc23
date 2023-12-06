use std::fs::File;
use std::io::BufRead;
use std::ops::Range;
use std::str::FromStr;

use crate::days::Day;

type Quantity = i128;

pub struct Day5 {
    start_seeds: Vec<Quantity>,
    transformers: Vec<Transformer>,
}

impl Day5 {
    fn convert_quantity(&self, seed: Quantity) -> Quantity {
        self.transformers
            .iter()
            .fold(seed, |curr_quantity, transformer| {
                transformer.convert_quantity(curr_quantity)
            })
    }

    fn get_ranges(&self) -> Vec<Range<Quantity>> {
        self.start_seeds
            .chunks(2)
            .map(|chunk| chunk[0]..chunk[0] + chunk[1])
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

    fn map_range(&self, range: Range<Quantity>) -> Vec<Range<Quantity>> {
        let mut mapped_ranges = vec![];
        let mut remaining_ranges = vec![range];
        for line in self.transformer_lines.iter() {
            let mut new_ranges = vec![];
            for remaining_range in remaining_ranges {
                let mut transformer_result = line.map_range(remaining_range);
                if let Some(mapped_range) = transformer_result.mapped_range {
                    mapped_ranges.push(mapped_range);
                }
                new_ranges.append(&mut transformer_result.remaining_ranges);
            }
            remaining_ranges = new_ranges;
        }
        mapped_ranges.append(&mut remaining_ranges);
        merge_ranges(&mut mapped_ranges);
        mapped_ranges
    }
}

fn merge_ranges(ranges: &mut Vec<Range<Quantity>>) {
    ranges.sort_unstable_by_key(|range| range.start);
    let mut idx = 0;
    while idx < ranges.len() - 1 {
        if ranges[idx].end >= ranges[idx + 1].start {
            ranges[idx].end = ranges[idx + 1].end;
            ranges.remove(idx + 1);
        } else {
            idx += 1;
        }
    }
}

#[derive(Debug, Clone)]
struct TransformerLine {
    destination_start: Quantity,
    source_start: Quantity,
    length: Quantity,
}

#[derive(Debug)]
struct RangeTransformResult {
    mapped_range: Option<Range<Quantity>>,
    remaining_ranges: Vec<Range<Quantity>>,
}

impl TransformerLine {
    fn map_range(&self, range: Range<Quantity>) -> RangeTransformResult {
        let left_part = range.start.min(self.source_start)..range.end.min(self.source_start);
        let right_part = range.start.max(self.source_start + self.length)
            ..range.end.max(self.source_start + self.length);
        let inner_part =
            range.start.max(self.source_start)..range.end.min(self.source_start + self.length);
        let diff = self.destination_start - self.source_start;

        let mut remaining_ranges = vec![];
        if !left_part.is_empty() {
            remaining_ranges.push(left_part);
        }
        if !right_part.is_empty() {
            remaining_ranges.push(right_part);
        }

        let mapped_range = if inner_part.is_empty() {
            None
        } else {
            Some(inner_part.start + diff..inner_part.end + diff)
        };
        RangeTransformResult {
            mapped_range,
            remaining_ranges,
        }
    }
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
                transformer_lines.sort_unstable_by_key(|tr: &TransformerLine| tr.source_start);
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
            .map(|seed| self.convert_quantity(*seed))
            .min()
            .expect("there is at least one seed");
        result.to_string()
    }

    fn solution2(&self) -> String {
        /*let mut result = 0;
        loop {
            let seed = self.convert_quantity_reverse(result);
            if self.contains_seed(seed) {
                break;
            }
            result += 1;
        }
        result.to_string()*/
        let seed_ranges = self.get_ranges();
        let result = self
            .transformers
            .iter()
            .fold(seed_ranges, |ranges, transformer| {
                ranges
                    .into_iter()
                    .flat_map(|range| transformer.map_range(range))
                    .collect()
            })
            .iter()
            .map(|range| range.start)
            .min()
            .unwrap();
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

    #[test]
    fn test_day5_reverse() {
        let input = File::open("./inputs/day5/input.txt").expect("File not found");
        let day5 = Day5::make_day(input);
        for seed in 0..100 {
            let location = day5.convert_quantity(seed);
            assert_eq!(day5.convert_quantity_reverse(location), seed);
        }
    }
}
