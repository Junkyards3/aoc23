use std::fs::File;
use std::io::Read;

use crate::days::Day;

pub struct Day13 {
    islands: Vec<Island>,
}

#[derive(Debug)]
pub struct Island {
    grid: Vec<Vec<Terrain>>,
}

impl Island {
    fn get_horizontal_keys(&self) -> Vec<LineKey> {
        self.grid.iter().map(|line| to_key(line)).collect()
    }

    fn get_vertical_keys(&self) -> Vec<LineKey> {
        let transposed: Vec<Vec<_>> = (0..self.grid[0].len())
            .map(|col| {
                (0..self.grid.len())
                    .map(|row| self.grid[row][col])
                    .collect()
            })
            .collect();
        transposed.iter().map(|line| to_key(line)).collect()
    }
}

type LineKey = u128;

fn to_key(line: &[Terrain]) -> LineKey {
    let binary_str = line
        .iter()
        .map(|terrain| match terrain {
            Terrain::Ash => '1',
            Terrain::Rock => '0',
        })
        .collect::<String>();
    LineKey::from_str_radix(&binary_str, 2).unwrap()
}

impl From<&str> for Island {
    fn from(island_lines: &str) -> Self {
        let grid = island_lines
            .lines()
            .map(|line| line.chars().map(|c| c.into()).collect::<Vec<Terrain>>())
            .collect();
        Island { grid }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Terrain {
    Ash,
    Rock,
}

impl From<char> for Terrain {
    fn from(value: char) -> Self {
        match value {
            '.' => Terrain::Ash,
            '#' => Terrain::Rock,
            _ => panic!("expected . or #"),
        }
    }
}

fn find_symmetry(keys: &[LineKey]) -> Option<usize> {
    let length = keys.len() - 1;
    for idx in 0..length {
        //check symmetry between idx and idx+1
        let diff = (idx + 1).min(length.saturating_sub(idx));
        let check = (0..diff).all(|offset| keys[idx - offset] == keys[idx + 1 + offset]);
        if check {
            return Some(idx);
        }
    }
    None
}

fn count_bit_diffs(a: LineKey, b: LineKey) -> usize {
    (a ^ b).count_ones() as usize
}

fn find_symmetry_one_diff(keys: &[LineKey]) -> Option<usize> {
    let length = keys.len() - 1;
    for idx in 0..length {
        //check symmetry between idx and idx+1
        let diff = (idx + 1).min(length.saturating_sub(idx));
        let check = (0..diff)
            .filter(|offset| keys[idx - offset] != keys[idx + 1 + offset])
            .collect::<Vec<_>>();
        if check.len() == 1 && count_bit_diffs(keys[idx - check[0]], keys[idx + 1 + check[0]]) == 1
        {
            return Some(idx);
        }
    }
    None
}

impl Day for Day13 {
    fn make_day(file: File) -> Self {
        let mut contents = String::new();
        let mut buf_reader = std::io::BufReader::new(file);
        buf_reader
            .read_to_string(&mut contents)
            .expect("could read");
        let islands = contents
            .split("\n\n")
            .map(|island_lines| island_lines.into())
            .collect();
        Day13 { islands }
    }

    fn solution1(&self) -> String {
        let mut result = 0;
        for island in self.islands.iter() {
            let horizontal_keys = island.get_horizontal_keys();
            if let Some(t) = find_symmetry(&horizontal_keys) {
                result += 100 * (t + 1);
            } else {
                let vertical_keys = island.get_vertical_keys();
                if let Some(t) = find_symmetry(&vertical_keys) {
                    result += t + 1;
                } else {
                    panic!("no sym");
                }
            }
        }
        result.to_string()
    }

    fn solution2(&self) -> String {
        let mut result = 0;
        for island in self.islands.iter() {
            let horizontal_keys = island.get_horizontal_keys();
            if let Some(t) = find_symmetry_one_diff(&horizontal_keys) {
                result += 100 * (t + 1);
            } else {
                let vertical_keys = island.get_vertical_keys();
                if let Some(t) = find_symmetry_one_diff(&vertical_keys) {
                    result += t + 1;
                } else {
                    panic!("no sym one diff");
                }
            }
        }
        result.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day13_sol1() {
        let input = File::open("./inputs/day13/input_test.txt").expect("File not found");
        let day = Day13::make_day(input);
        assert_eq!(day.solution1(), "405");
    }

    #[test]
    fn test_day13_sol2() {
        let input = File::open("./inputs/day13/input_test.txt").expect("File not found");
        let day = Day13::make_day(input);
        assert_eq!(day.solution2(), "400");
    }
}
