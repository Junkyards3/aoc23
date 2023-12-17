use ndarray::Array2;
use pathfinding::prelude::astar;
use std::fs::File;
use std::io::BufRead;

use crate::days::Day;

type Coordinate = (usize, usize);
type Heat = u32;

pub struct Day17 {
    heat_map: Array2<Heat>,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
enum Direction {
    Up,
    Left,
    Right,
    Down,
}

impl Direction {
    fn allowed_moves(&self) -> [Direction; 3] {
        match self {
            Direction::Up => [Direction::Up, Direction::Left, Direction::Right],
            Direction::Left => [Direction::Up, Direction::Left, Direction::Down],
            Direction::Right => [Direction::Up, Direction::Down, Direction::Right],
            Direction::Down => [Direction::Down, Direction::Left, Direction::Right],
        }
    }

    fn get_next_pos(
        &self,
        curr_pos: Coordinate,
        nb_rows: usize,
        nb_cols: usize,
    ) -> Option<Coordinate> {
        let (d_row, d_col) = match self {
            Direction::Up => (-1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
        };
        let (row, col) = (curr_pos.0 as isize + d_row, curr_pos.1 as isize + d_col);
        if (0..nb_rows as isize).contains(&row) && (0..nb_cols as isize).contains(&col) {
            Some((row as usize, col as usize))
        } else {
            None
        }
    }

    fn get_orthogonal_directions(&self) -> [Direction; 2] {
        match self {
            Direction::Up | Direction::Down => [Direction::Left, Direction::Right],
            Direction::Left | Direction::Right => [Direction::Up, Direction::Down],
        }
    }

    fn get_range_pos(
        &self,
        curr_pos: Coordinate,
        nb_rows: usize,
        nb_cols: usize,
    ) -> Vec<Coordinate> {
        let (r, c) = (curr_pos.0 as isize, curr_pos.1 as isize);
        match self {
            Direction::Up => (r - 10..=r - 4).map(|row| (row, c)).collect::<Vec<_>>(),
            Direction::Left => (c - 10..=c - 4).map(|col| (r, col)).collect::<Vec<_>>(),
            Direction::Right => (c + 4..=c + 10).map(|col| (r, col)).collect::<Vec<_>>(),
            Direction::Down => (r + 4..=r + 10).map(|row| (row, c)).collect::<Vec<_>>(),
        }
        .into_iter()
        .filter_map(|(row, col)| {
            if (0..nb_rows as isize).contains(&row) && (0..nb_cols as isize).contains(&col) {
                Some((row as usize, col as usize))
            } else {
                None
            }
        })
        .collect()
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct ConsecutiveDirection {
    direction: Direction,
    nb_times: u8,
}

impl ConsecutiveDirection {
    fn get_new_consecutive(&self, dir_taken: Direction) -> Self {
        if self.direction == dir_taken {
            ConsecutiveDirection {
                direction: self.direction,
                nb_times: self.nb_times + 1,
            }
        } else {
            ConsecutiveDirection {
                direction: dir_taken,
                nb_times: 1,
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct Node {
    pos: Coordinate,
    consecutive_direction: ConsecutiveDirection,
}

impl Node {
    fn get_successors(
        &self,
        nb_rows: usize,
        nb_cols: usize,
        heat_map: &Array2<Heat>,
    ) -> Vec<(Node, Heat)> {
        self.consecutive_direction
            .direction
            .allowed_moves()
            .iter()
            .filter_map(|dir_taken| {
                if let Some(next_pos) = dir_taken.get_next_pos(self.pos, nb_rows, nb_cols) {
                    let new_consecutive_direction =
                        self.consecutive_direction.get_new_consecutive(*dir_taken);
                    if new_consecutive_direction.nb_times <= 3 {
                        Some((
                            Node {
                                pos: next_pos,
                                consecutive_direction: new_consecutive_direction,
                            },
                            heat_map[next_pos],
                        ))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect()
    }
}

fn heat_from(start_pos: Coordinate, end_pos: Coordinate, heat_map: &Array2<Heat>) -> Heat {
    let (x1, y1) = start_pos;
    let (x2, y2) = end_pos;
    if x1 == x2 {
        let dy: isize = if y1 < y2 { 1 } else { -1 };
        let mut ret = 0;
        let mut y = y1 as isize;
        while y != (y2 as isize) {
            y += dy;
            ret += heat_map[(x1, y as usize)];
        }
        ret
    } else {
        let dx: isize = if x1 < x2 { 1 } else { -1 };
        let mut ret = 0;
        let mut x = x1 as isize;
        while x != (x2 as isize) {
            x += dx;
            ret += heat_map[(x as usize, y1)];
        }
        ret
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct Node2 {
    pos: Coordinate,
    possible_direction: [Direction; 2],
}

impl Node2 {
    fn get_successors_wobbly(
        &self,
        nb_rows: usize,
        nb_cols: usize,
        heat_map: &Array2<Heat>,
    ) -> Vec<(Node2, Heat)> {
        self.possible_direction
            .iter()
            .flat_map(|dir_taken| {
                dir_taken
                    .get_range_pos(self.pos, nb_rows, nb_cols)
                    .iter()
                    .map(|pos| {
                        let heat_consumed = heat_from(self.pos, *pos, &heat_map);
                        (
                            Node2 {
                                pos: *pos,
                                possible_direction: dir_taken.get_orthogonal_directions(),
                            },
                            heat_consumed,
                        )
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }
}
impl Day for Day17 {
    fn make_day(file: File) -> Self {
        let data: Vec<Vec<_>> = std::io::BufReader::new(file)
            .lines()
            .map(|line| {
                line.expect("doc should have lines")
                    .chars()
                    .map(|ch| ch.to_digit(10).expect("should be digit") as Heat)
                    .collect()
            })
            .collect();
        let rows = data.len();
        let cols = data[0].len();
        let flattened: Vec<_> = data.into_iter().flatten().collect();
        let heat_map = Array2::from_shape_vec((rows, cols), flattened).unwrap();
        Day17 { heat_map }
    }

    fn solution1(&self) -> String {
        let (nb_rows, nb_cols) = self.heat_map.dim();
        let start_node = Node {
            pos: (0, 0),
            consecutive_direction: ConsecutiveDirection {
                direction: Direction::Right,
                nb_times: 0,
            },
        };
        let result = astar(
            &start_node,
            |node| node.get_successors(nb_rows, nb_cols, &self.heat_map),
            |node| (node.pos.0.abs_diff(nb_rows) + node.pos.1.abs_diff(nb_cols)) as Heat,
            |node| node.pos == (nb_rows - 1, nb_cols - 1),
        );
        let (_, result) = result.unwrap();
        result.to_string()
    }

    fn solution2(&self) -> String {
        let (nb_rows, nb_cols) = self.heat_map.dim();
        let start_node = Node2 {
            pos: (0, 0),
            possible_direction: [Direction::Down, Direction::Right],
        };
        let result = astar(
            &start_node,
            |node| node.get_successors_wobbly(nb_rows, nb_cols, &self.heat_map),
            |node| (node.pos.0.abs_diff(nb_rows) + node.pos.1.abs_diff(nb_cols)) as Heat,
            |node| node.pos == (nb_rows - 1, nb_cols - 1),
        );
        let (_, result) = result.unwrap();
        result.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day17_sol1() {
        let input = File::open("./inputs/day17/input_test.txt").expect("File not found");
        let day = Day17::make_day(input);
        assert_eq!(day.solution1(), "102");
    }

    #[test]
    fn test_day17_sol2() {
        let input = File::open("./inputs/day17/input_test.txt").expect("File not found");
        let day = Day17::make_day(input);
        assert_eq!(day.solution2(), "94");
    }
}
