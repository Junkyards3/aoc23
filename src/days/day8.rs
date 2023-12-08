use num::integer::lcm;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::str::FromStr;

use crate::days::Day;

#[derive(Debug)]
pub struct Day8 {
    instructions: Vec<Direction>,
    nodes: HashMap<String, Node>,
}

#[derive(Debug)]
pub struct Walker<'a> {
    curr_node_id: &'a str,
    nb_instructions: usize,
}

impl<'a> Walker<'a> {
    fn make_from_id(id: &'a str) -> Walker {
        Walker {
            curr_node_id: id,
            nb_instructions: 0,
        }
    }

    fn walk<F>(mut self, day8: &'a Day8, end_condition: F) -> usize
    where
        F: Fn(&str) -> bool,
    {
        let length = day8.instructions.len();
        while !end_condition(self.curr_node_id) {
            self.curr_node_id = match day8.instructions[self.nb_instructions % length] {
                Direction::Left => &day8.nodes[self.curr_node_id].left_node_id,
                Direction::Right => &day8.nodes[self.curr_node_id].right_node_id,
            };
            self.nb_instructions += 1;
        }
        self.nb_instructions
    }
}
#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Node {
    id: String,
    left_node_id: String,
    right_node_id: String,
}

impl FromStr for Node {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, follow_up_str) = s.split_once(" = ").ok_or(())?;
        let pattern: &[_] = &['(', ')'];
        let (left_node_id, right_node_id) = follow_up_str
            .trim_matches(pattern)
            .split_once(", ")
            .ok_or(())?;
        let id = id.to_owned();
        let left_node_id = left_node_id.to_owned();
        let right_node_id = right_node_id.to_owned();
        Ok(Node {
            id,
            left_node_id,
            right_node_id,
        })
    }
}
impl Day for Day8 {
    fn make_day(file: File) -> Self {
        let document_lines = std::io::BufReader::new(file)
            .lines()
            .map(|line| line.expect("doc should have lines"))
            .collect::<Vec<_>>();

        let instructions = document_lines[0]
            .chars()
            .map(|dir_ch| dir_ch.try_into().expect("should be L or R"))
            .collect();

        let nodes = document_lines
            .into_iter()
            .skip(2)
            .map(|line_node| {
                let node = line_node.parse::<Node>().expect("should be node line");
                (node.id.clone(), node)
            })
            .collect();

        Day8 {
            instructions,
            nodes,
        }
    }

    fn solution1(&self) -> String {
        let walker = Walker::make_from_id("AAA");
        let result = walker.walk(&self, |id| id == "ZZZ");
        result.to_string()
    }

    fn solution2(&self) -> String {
        let walkers = self
            .nodes
            .keys()
            .filter(|node_id| node_id.chars().last().is_some_and(|c| c == 'A'))
            .map(|node_id| Walker::make_from_id(node_id))
            .collect::<Vec<_>>();
        let time_to_reach = walkers
            .into_iter()
            .map(|walker| walker.walk(&self, |id| id.chars().last().is_some_and(|c| c == 'Z')))
            .reduce(|t1, t2| lcm(t1, t2))
            .expect("there is at least one walker");
        time_to_reach.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day8_sol1() {
        let input = File::open("./inputs/day8/input_test.txt").expect("File not found");
        let day = Day8::make_day(input);
        assert_eq!(day.solution1(), "2");
    }

    #[test]
    fn test_day8_sol1_bis() {
        let input = File::open("./inputs/day8/input_test2.txt").expect("File not found");
        let day = Day8::make_day(input);
        assert_eq!(day.solution1(), "6");
    }

    #[test]
    fn test_day8_sol2() {
        let input = File::open("./inputs/day8/input_test3.txt").expect("File not found");
        let day = Day8::make_day(input);
        assert_eq!(day.solution2(), "6");
    }
}
