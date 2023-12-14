use itertools::Either;
use ndarray::{Array2, Axis};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;

use crate::days::Day;

pub struct Day14 {
    tiles: Array2<Tile>,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
enum Tile {
    Empty,
    Square,
    Circle,
}

impl Default for Tile {
    fn default() -> Self {
        Tile::Empty
    }
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

fn get_set_up(len: usize, reversed: bool) -> (isize, isize, impl Iterator<Item = usize>) {
    match reversed {
        true => (len as isize - 1, -1, Either::Left((0..len).rev())),
        false => (0, 1, Either::Right(0..len)),
    }
}

fn tilt(tiles: &Array2<Tile>, axis_number: usize, reversed: bool) -> Array2<Tile> {
    let mut tilted = Array2::default(tiles.raw_dim());
    for (lane, mut new_lane) in tiles
        .axis_iter(Axis(axis_number))
        .zip(tilted.axis_iter_mut(Axis(axis_number)))
    {
        let (mut current_free_spot, step, range) = get_set_up(lane.len(), reversed);

        for idx in range {
            match lane[idx] {
                Tile::Square => {
                    new_lane[idx] = Tile::Square;
                    current_free_spot = idx as isize + step;
                }
                Tile::Circle => {
                    new_lane[current_free_spot as usize] = Tile::Circle;
                    current_free_spot += step;
                }
                Tile::Empty => {}
            }
        }
    }
    tilted
}

fn get_load(tiles: &Array2<Tile>) -> usize {
    tiles
        .axis_iter(Axis(0))
        .rev()
        .enumerate()
        .map(|(load_min_one, row)| {
            row.iter().filter(|tile| **tile == Tile::Circle).count() * (load_min_one + 1)
        })
        .sum::<usize>()
}

fn tilt_four_times(tiles: Array2<Tile>) -> Array2<Tile> {
    let mut new_tilted = tilt(&tiles, 1, false);
    new_tilted = tilt(&new_tilted, 0, false);
    new_tilted = tilt(&new_tilted, 1, true);
    new_tilted = tilt(&new_tilted, 0, true);
    new_tilted
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
        let tilted = tilt(&self.tiles, 1, false);
        let result = get_load(&tilted);
        result.to_string()
    }

    fn solution2(&self) -> String {
        let tiles = self.tiles.clone();
        let mut current_step = 0;
        let period: usize;
        let mut map_seen_tiles = HashMap::new();
        let mut vec_seen_tiles = vec![tiles.clone()];
        let mut curr_tilted = tiles.clone();
        loop {
            current_step += 1;
            curr_tilted = tilt_four_times(curr_tilted);
            match map_seen_tiles.get(&curr_tilted) {
                Some(step) => {
                    period = current_step - step;
                    break;
                }
                None => {
                    map_seen_tiles.insert(curr_tilted.clone(), current_step);
                }
            }
            vec_seen_tiles.push(curr_tilted.clone());
        }
        let same_tiles_step = ((1000000000 - current_step) % period) + (current_step - period);
        let result = get_load(&vec_seen_tiles[same_tiles_step]);
        result.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day14_sol1() {
        let input = File::open("./inputs/day14/input_test.txt").expect("File not found");
        let day = Day14::make_day(input);
        assert_eq!(day.solution1(), "136");
    }

    #[test]
    fn test_day14_sol2() {
        let input = File::open("./inputs/day14/input_test.txt").expect("File not found");
        let day = Day14::make_day(input);
        assert_eq!(day.solution2(), "64");
    }
}
