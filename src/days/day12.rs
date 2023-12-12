use std::fs::File;
use std::io::BufRead;

use crate::days::Day;

#[derive(Debug)]
pub struct Day12 {
    lines: Vec<SpringLine>,
}

#[derive(Debug)]
struct SpringLine {
    groups: Vec<ContiguousGroup>,
    goal: Vec<usize>,
}

impl SpringLine {
    fn find_ways_to_fit(&self, layouts: &Vec<Vec<SpringLayout>>) -> usize {
        let mut curr_goal_idx = 0;
        for group in self.groups.iter() {
            /*let curr_goal = self.goal[curr_goal_idx];
            let compatible_layouts = layouts[group.]*/
        }
        0
    }
}

impl From<String> for SpringLine {
    fn from(value: String) -> Self {
        let (springs_str, goal_str) = value
            .split_once(' ')
            .expect("should separate springs and goals");
        let groups = springs_str
            .split('.')
            .filter(|r| !r.is_empty())
            .map(|g| g.into())
            .collect();
        let goal = goal_str
            .split(',')
            .map(|nb_str| nb_str.parse().expect("should be a number"))
            .collect();

        SpringLine { groups, goal }
    }
}

#[derive(Debug)]
struct ContiguousGroup {
    elements: Vec<(GroupElement, usize)>,
}

impl From<&str> for ContiguousGroup {
    fn from(value: &str) -> Self {
        let mut chars = value.chars();
        let mut elements = vec![];
        let mut actual_group_size = 1;
        let mut actual_group_element: GroupElement =
            chars.next().expect("should have a char").into();
        for c in chars {
            let new_group = c.into();
            if new_group == actual_group_element {
                actual_group_size += 1;
            } else {
                elements.push((actual_group_element, actual_group_size));
                actual_group_size = 1;
                actual_group_element = new_group;
            }
        }
        elements.push((actual_group_element, actual_group_size));
        ContiguousGroup { elements }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum GroupElement {
    Unknown,
    Spring,
}

impl From<char> for GroupElement {
    fn from(value: char) -> Self {
        match value {
            '#' => GroupElement::Spring,
            '?' => GroupElement::Unknown,
            _ => panic!("wrong char"),
        }
    }
}
#[derive(Debug)]
struct SpringLayout {
    size: usize,
    touch_left: bool,
    touch_right: bool,
    spring_groups: Vec<usize>,
}

impl SpringLayout {
    fn compute_size_one_layouts() -> Vec<Self> {
        vec![
            SpringLayout {
                size: 1,
                touch_left: false,
                touch_right: false,
                spring_groups: vec![],
            },
            SpringLayout {
                size: 1,
                touch_left: true,
                touch_right: true,
                spring_groups: vec![1],
            },
        ]
    }

    fn compute_next_size_layouts(size_layout: &Vec<SpringLayout>) -> Vec<SpringLayout> {
        let previous_size = size_layout[0].size;
        size_layout
            .iter()
            .flat_map(|layout| {
                let mut extended_groups = layout.spring_groups.clone();
                if layout.touch_right {
                    *extended_groups
                        .last_mut()
                        .expect("there is a last element if touch_right is true") += 1;
                } else {
                    extended_groups.push(1)
                }
                let extend_with_spring = SpringLayout {
                    size: previous_size + 1,
                    touch_left: layout.touch_left,
                    touch_right: true,
                    spring_groups: extended_groups,
                };
                let extend_without_spring = SpringLayout {
                    size: previous_size + 1,
                    touch_left: layout.touch_left,
                    touch_right: false,
                    spring_groups: layout.spring_groups.clone(),
                };
                vec![extend_with_spring, extend_without_spring]
            })
            .collect()
    }

    fn compute_all_layouts_below_size(size: usize) -> Vec<Vec<SpringLayout>> {
        let mut result = Vec::with_capacity(size);
        result.push(SpringLayout::compute_size_one_layouts());
        for _ in 2..=size {
            let curr_layouts = SpringLayout::compute_next_size_layouts(result.last().unwrap());
            result.push(curr_layouts);
        }
        result.into_iter().collect()
    }
}

impl Day for Day12 {
    fn make_day(file: File) -> Self {
        let lines = std::io::BufReader::new(file)
            .lines()
            .map(|line| line.expect("doc should have lines").into())
            .collect();
        Day12 { lines }
    }

    fn solution1(&self) -> String {
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
    fn test_day12_sol1() {
        let input = File::open("./inputs/day12/input_test.txt").expect("File not found");
        let day = Day12::make_day(input);
        assert_eq!(day.solution1(), "sol1");
    }

    #[test]
    fn test_day12_sol2() {
        let input = File::open("./inputs/day12/input_test.txt").expect("File not found");
        let day = Day12::make_day(input);
        assert_eq!(day.solution2(), "sol2");
    }
}
