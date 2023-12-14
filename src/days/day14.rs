use ndarray::{Array2, Axis};
use std::fs::File;
use std::io::BufRead;

use crate::days::Day;

pub struct Day14 {
    tiles: Array2<Tile>,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Tile {
    Empty,
    Square,
    Circle,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Empty,
            '#' => Tile::Square,
            'O' => Tile::Circle,
            _ => panic!("not a tile char"),
        }
    }
}

impl Day for Day14 {
    fn make_day(file: File) -> Self {
        let data = std::io::BufReader::new(file)
            .lines()
            .map(|line| {
                line.expect("doc should have lines")
                    .chars()
                    .map(|c| c.into())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let rows = data.len();
        let cols = data[0].len();
        let flattened: Vec<_> = data.into_iter().flatten().collect();
        let tiles = Array2::from_shape_vec((rows, cols), flattened).unwrap();
        Day14 { tiles }
    }

    fn solution1(&self) -> String {
        let mut tiles = self.tiles.clone();
        tiles.index_axis_mut(Axis(1)).map(|view| {
            view.for_each(|x|)
        });
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
    fn test_day14_sol1() {
        let input = File::open("./inputs/day14/input_test.txt").expect("File not found");
        let day = Day14::make_day(input);
        assert_eq!(day.solution1(), "sol1");
    }

    #[test]
    fn test_day14_sol2() {
        let input = File::open("./inputs/day14/input_test.txt").expect("File not found");
        let day = Day14::make_day(input);
        assert_eq!(day.solution2(), "sol2");
    }
}
