use itertools::Itertools;
use pathfinding::prelude::dijkstra_reach;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;

use crate::days::Day;

type Num = i32;
type Coordinate = (Num, Num);

fn distance(c1: Coordinate, c2: Coordinate) -> Num {
    (c1.0.abs_diff(c2.0) + c1.1.abs_diff(c2.0)) as Num
}

fn neighbors((x, y): Coordinate) -> [Coordinate; 4] {
    [(x + 1, y), (x - 1, y), (x, y - 1), (x, y + 1)]
}

pub struct Day21 {
    rocks: HashSet<Coordinate>,
    start: Coordinate,
    nb_rows: Num,
    nb_cols: Num,
}

impl Day21 {
    fn compute_nodes_at_exactly_n_steps(&self, nb_steps: Num) -> usize {
        let reached_nodes = dijkstra_reach(&self.start, |node, _| {
            neighbors(*node)
                .iter()
                .filter(|neighbor| {
                    distance(**neighbor, self.start) <= nb_steps && !self.rocks.contains(neighbor)
                })
                .map(|neighbor| (*neighbor, 1))
                .collect_vec()
        });
        reached_nodes
            .filter(|reachable_item| {
                reachable_item.total_cost <= nb_steps
                    && (reachable_item.total_cost - nb_steps) % 2 == 0
            })
            .count()
    }

    fn compute_nodes_at_exactly_n_steps_infinite(&self, nb_steps: Num) -> usize {
        let reached_nodes = dijkstra_reach(&self.start, |node, _| {
            neighbors(*node)
                .iter()
                .filter(|(row, col)| {
                    distance((*row, *col), self.start) <= nb_steps
                        && !self
                            .rocks
                            .contains(&(row.rem_euclid(self.nb_rows), col.rem_euclid(self.nb_cols)))
                })
                .map(|neighbor| (*neighbor, 1))
                .collect_vec()
        });
        reached_nodes
            .filter(|reachable_item| {
                reachable_item.total_cost <= nb_steps
                    && (reachable_item.total_cost - nb_steps) % 2 == 0
            })
            .count()
    }
}

impl Day for Day21 {
    fn make_day(file: File) -> Self {
        let mut nb_rows = 0;
        let mut nb_cols = 0;
        let (start, rocks): (Vec<_>, Vec<_>) = std::io::BufReader::new(file)
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                nb_rows += 1;
                let line = line.expect("doc should have lines");
                nb_cols = line.len() as Num;
                line.chars()
                    .enumerate()
                    .filter(|(_, ch)| *ch != '.')
                    .map(move |(col, ch)| ((row as Num, col as Num), ch))
                    .collect_vec()
            })
            .partition(|(_, ch)| *ch == 'S');
        let start = start[0].0;
        let rocks = rocks.iter().map(|(coords, _)| *coords).collect();
        Day21 {
            start,
            rocks,
            nb_rows,
            nb_cols,
        }
    }

    fn solution1(&self) -> String {
        self.compute_nodes_at_exactly_n_steps(64).to_string()
    }

    fn solution2(&self) -> String {
        self.compute_nodes_at_exactly_n_steps(64).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day21_sol1() {
        let input = File::open("./inputs/day21/input_test.txt").expect("File not found");
        let day = Day21::make_day(input);
        assert_eq!(day.compute_nodes_at_exactly_n_steps(6), 16);
    }

    #[test]
    fn test_day21_sol2() {
        let input = File::open("./inputs/day21/input_test.txt").expect("File not found");
        let day = Day21::make_day(input);
        assert_eq!(day.compute_nodes_at_exactly_n_steps_infinite(6), 16);
        assert_eq!(day.compute_nodes_at_exactly_n_steps_infinite(10), 50);
        assert_eq!(day.compute_nodes_at_exactly_n_steps_infinite(50), 1594);
        assert_eq!(day.compute_nodes_at_exactly_n_steps_infinite(100), 6536);
        assert_eq!(day.compute_nodes_at_exactly_n_steps_infinite(500), 167004);
        assert_eq!(day.compute_nodes_at_exactly_n_steps_infinite(1000), 668697);
        /*assert_eq!(
            day.compute_nodes_at_exactly_n_steps_infinite(5000),
            16733044
        );*/
    }

    #[test]
    fn test_pattern() {
        let input = File::open("./inputs/day21/input_test.txt").expect("File not found");
        let day = Day21::make_day(input);
        assert_eq!(day.compute_nodes_at_exactly_n_steps_infinite(6), 16);
        assert_eq!(day.compute_nodes_at_exactly_n_steps_infinite(10), 50);
        assert_eq!(day.compute_nodes_at_exactly_n_steps_infinite(50), 1594);
        assert_eq!(day.compute_nodes_at_exactly_n_steps_infinite(100), 6536);
        assert_eq!(day.compute_nodes_at_exactly_n_steps_infinite(500), 167004);
        assert_eq!(day.compute_nodes_at_exactly_n_steps_infinite(1000), 668697);
        /*assert_eq!(
            day.compute_nodes_at_exactly_n_steps_infinite(5000),
            16733044
        );*/
    }
}
