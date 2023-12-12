use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;

use crate::days::Day;

#[derive(Debug)]
pub struct Day12 {
    lines: Vec<SpringLine>,
}

#[derive(Debug)]
struct SpringLine {
    groups: Vec<SpringElement>,
    goal: Vec<usize>,
}

impl SpringLine {
    fn find_ways_to_fit(&self) -> usize {
        let mut cache = HashMap::new();
        find_ways_to_fit_recursive(&self.groups, &self.goal, &mut cache)
    }

    fn find_ways_to_fit_unfold(&self) -> usize {
        let mut groups_with_sep = self.groups.clone();
        groups_with_sep.push(SpringElement::Unknown);
        let unfolded_groups = groups_with_sep
            .into_iter()
            .cycle()
            .take(5 * self.groups.len() + 4)
            .collect::<Vec<_>>();
        let unfolded_goal = self
            .goal
            .clone()
            .into_iter()
            .cycle()
            .take(5 * self.goal.len())
            .collect::<Vec<_>>();
        let mut cache = HashMap::new();
        find_ways_to_fit_recursive(&unfolded_groups, &unfolded_goal, &mut cache)
    }
}

enum BrokenSeqResult<'a> {
    FailureToForm,
    SuccessWithSpringsRemaining(&'a [SpringElement]),
    SuccessWithSpringsEnd,
}

fn form_broken_seq(springs: &[SpringElement], goal: usize) -> BrokenSeqResult {
    for index in 0..goal {
        match springs.get(index) {
            None | Some(SpringElement::Working) => {
                return BrokenSeqResult::FailureToForm;
            }
            Some(SpringElement::Broken) | Some(SpringElement::Unknown) => {}
        }
    }

    if springs
        .get(goal)
        .is_some_and(|spring| *spring == SpringElement::Broken)
    {
        BrokenSeqResult::FailureToForm
    } else {
        if springs.len() < goal + 1 {
            BrokenSeqResult::SuccessWithSpringsEnd
        } else {
            BrokenSeqResult::SuccessWithSpringsRemaining(&springs[goal + 1..])
        }
    }
}

type CacheKey = (Vec<SpringElement>, Vec<usize>);

fn find_ways_to_fit_recursive(
    mut left_springs: &[SpringElement],
    left_goals: &[usize],
    cache: &mut HashMap<CacheKey, usize>,
) -> usize {
    while left_springs
        .get(0)
        .is_some_and(|spring| *spring == SpringElement::Working)
    {
        left_springs = &left_springs[1..];
    }

    if left_springs.is_empty() {
        return left_goals.is_empty().into();
    }

    if left_goals.is_empty() {
        return left_springs
            .iter()
            .all(|spring| *spring != SpringElement::Broken)
            .into();
    }
    let key = (left_springs.to_vec(), left_goals.to_vec());

    if let Some(count) = cache.get(&key) {
        return *count;
    }

    let count_with_broken = match form_broken_seq(&left_springs, left_goals[0]) {
        BrokenSeqResult::FailureToForm => 0,
        BrokenSeqResult::SuccessWithSpringsRemaining(remaining_springs) => {
            find_ways_to_fit_recursive(remaining_springs, &left_goals[1..], cache)
        }
        BrokenSeqResult::SuccessWithSpringsEnd => {
            find_ways_to_fit_recursive(&[], &left_goals[1..], cache)
        }
    };

    let result = if left_springs[0] == SpringElement::Unknown {
        let count_with_working = find_ways_to_fit_recursive(&left_springs[1..], left_goals, cache);
        count_with_working + count_with_broken
    } else {
        count_with_broken
    };
    cache.insert(key, result);
    result
}

impl From<String> for SpringLine {
    fn from(value: String) -> Self {
        let (springs_str, goal_str) = value
            .split_once(' ')
            .expect("should separate springs and goals");
        let groups = springs_str.chars().map(|g| g.into()).collect();
        let goal = goal_str
            .split(',')
            .map(|nb_str| nb_str.parse().expect("should be a number"))
            .collect();

        SpringLine { groups, goal }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum SpringElement {
    Unknown,
    Broken,
    Working,
}

impl From<char> for SpringElement {
    fn from(value: char) -> Self {
        match value {
            '#' => SpringElement::Broken,
            '?' => SpringElement::Unknown,
            '.' => SpringElement::Working,
            _ => panic!("wrong char"),
        }
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
        let result = self
            .lines
            .iter()
            .map(|line| line.find_ways_to_fit())
            .sum::<usize>();
        result.to_string()
    }

    fn solution2(&self) -> String {
        let result = self
            .lines
            .iter()
            .map(|line| line.find_ways_to_fit_unfold())
            .sum::<usize>();
        result.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day12_sol1() {
        let input = File::open("./inputs/day12/input_test.txt").expect("File not found");
        let day = Day12::make_day(input);
        assert_eq!(day.solution1(), "21");
    }

    #[test]
    fn test_find_ways() {
        let g1: SpringLine = "#.??. 1".to_string().into();
        let mut cache = HashMap::new();
        let v = find_ways_to_fit_recursive(&g1.groups, &g1.goal, &mut cache);
        dbg!(cache);
        assert_eq!(v, 1);
    }

    #[test]
    fn test_day12_sol2() {
        let input = File::open("./inputs/day12/input_test.txt").expect("File not found");
        let day = Day12::make_day(input);
        assert_eq!(day.solution2(), "525152");
    }

    #[test]
    fn test_find_ways_2() {
        let g1: SpringLine = "????.#...#... 4,1,1".to_string().into();
        let v = g1.find_ways_to_fit_unfold();
        assert_eq!(v, 16);
    }
}
