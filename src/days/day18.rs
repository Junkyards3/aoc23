use std::fs::File;
use std::io::BufRead;

use crate::days::Day;

type Num = i128;
type Coordinate = (Num, Num);

pub struct Day18 {
    dig_instructions: Vec<DigInstruction>,
    dig_instructions_alternate: Vec<DigInstruction>,
}

impl Day18 {
    fn compute_inside_points(&self, alternate: bool) -> Num {
        let mut current_vertice = (0, 0);
        let mut double_area = 0;
        let mut boundary_points = 0;
        let iter = if alternate {
            self.dig_instructions_alternate.iter()
        } else {
            self.dig_instructions.iter()
        };
        for dig_instruction in iter {
            let (dr, dc) = dig_instruction.direction.get_movement();
            current_vertice = (
                current_vertice.0 + dr * dig_instruction.nb_digs,
                current_vertice.1 + dc * dig_instruction.nb_digs,
            );
            boundary_points += dig_instruction.nb_digs;
            let diff_area = if dr == 0 {
                current_vertice.0 * dig_instruction.nb_digs * dc
            } else {
                -current_vertice.1 * dig_instruction.nb_digs * dr
            };
            double_area += diff_area;
        }
        (double_area.abs() / 2) + (boundary_points / 2) + 1
    }
}
#[derive(Debug)]
struct DigInstruction {
    direction: Direction,
    nb_digs: Num,
}

impl DigInstruction {
    fn get_two_instructions_from_line(value: &str) -> (Self, Self) {
        let elements = value.split_whitespace().collect::<Vec<_>>();
        let direction = elements[0].chars().next().unwrap().into();
        let nb_digs = elements[1].parse().unwrap();
        let instr1 = DigInstruction { direction, nb_digs };
        let dir_alternate = match elements[2].chars().nth(7).unwrap() {
            '0' => Direction::Right,
            '1' => Direction::Down,
            '2' => Direction::Left,
            '3' => Direction::Up,
            _ => panic!("wrong digit"),
        };
        let nb_digs_alternate = Num::from_str_radix(&elements[2][2..7], 16).unwrap();
        let instr2 = DigInstruction {
            direction: dir_alternate,
            nb_digs: nb_digs_alternate,
        };
        (instr1, instr2)
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Direction {
    fn get_movement(&self) -> Coordinate {
        match self {
            Direction::Up => (-1, 0),
            Direction::Left => (0, -1),
            Direction::Down => (1, 0),
            Direction::Right => (0, 1),
        }
    }
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'U' => Direction::Up,
            'L' => Direction::Left,
            'D' => Direction::Down,
            'R' => Direction::Right,
            _ => panic!("unexpected direction char"),
        }
    }
}

impl Day for Day18 {
    fn make_day(file: File) -> Self {
        let (dig_instructions, dig_instructions_alternate) = std::io::BufReader::new(file)
            .lines()
            .map(|line| {
                DigInstruction::get_two_instructions_from_line(
                    line.expect("doc should have lines").as_str(),
                )
            })
            .unzip();
        Day18 {
            dig_instructions,
            dig_instructions_alternate,
        }
    }

    fn solution1(&self) -> String {
        let inside_points = self.compute_inside_points(false);
        inside_points.to_string()
    }

    fn solution2(&self) -> String {
        let inside_points = self.compute_inside_points(true);
        inside_points.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day18_sol1() {
        let input = File::open("./inputs/day18/input_test.txt").expect("File not found");
        let day = Day18::make_day(input);
        assert_eq!(day.solution1(), "62");
    }

    #[test]
    fn test_day18_sol2() {
        let input = File::open("./inputs/day18/input_test.txt").expect("File not found");
        let day = Day18::make_day(input);
        assert_eq!(day.solution2(), "952408144115");
    }
}
