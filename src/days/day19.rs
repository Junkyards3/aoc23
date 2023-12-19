use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::ops::Range;

use crate::days::Day;

type Num = u64;

#[derive(Debug)]
pub struct Day19 {
    workflows: HashMap<String, Workflow>,
    parts: Vec<Part>,
}

impl Day19 {
    fn does_accept(&self, part: &Part) -> bool {
        let mut next_workflow_id = "in".to_string();
        loop {
            let next_workflow = self.workflows.get(&next_workflow_id).unwrap();
            let result = next_workflow.apply_to(&part);
            match result {
                RuleResult::Accepted => {
                    return true;
                }
                RuleResult::Rejected => {
                    return false;
                }
                RuleResult::Workflow(new_id) => {
                    next_workflow_id = new_id;
                }
            }
        }
    }

    fn find_numbers_of_accepting(&self) -> i128 {
        let conditions_by_id: HashMap<String, Vec<AcceptingConditionsWithFollowUp>> = self
            .workflows
            .iter()
            .map(|(id, workflow)| (id.to_string(), workflow.find_conditions_for_accepting()))
            .collect();

        let mut cache: HashMap<String, Vec<Vec<ElementaryCondition>>> =
            HashMap::with_capacity(self.workflows.len());

        fn find_conditions_id(
            id: &str,
            conditions_by_id: &HashMap<String, Vec<AcceptingConditionsWithFollowUp>>,
            cache: &mut HashMap<String, Vec<Vec<ElementaryCondition>>>,
        ) -> Vec<Vec<ElementaryCondition>> {
            if let Some(cond) = cache.get(id) {
                return cond.clone();
            }

            let conditions = conditions_by_id.get(id).unwrap();
            let mut result = Vec::with_capacity(conditions.len());
            for accepting_conditions in conditions {
                match accepting_conditions.follow_up.clone() {
                    FollowUp::Accepted => {
                        result.push(accepting_conditions.conditions.clone());
                    }
                    FollowUp::Id(id) => {
                        let mut additional_conditions =
                            find_conditions_id(&id, &conditions_by_id, cache);
                        let to_add = accepting_conditions.conditions.clone();
                        for additional_condition in additional_conditions.iter_mut() {
                            additional_condition.append(&mut to_add.clone());
                        }
                        result.append(&mut additional_conditions);
                    }
                }
            }

            cache.insert(id.to_string(), result.clone());
            result
        }

        fn find_acceptable_ranges(conditions: &[ElementaryCondition]) -> RangePart {
            let mut result = HashMap::with_capacity(4);
            result.insert(Category::X, 1..4001);
            result.insert(Category::M, 1..4001);
            result.insert(Category::A, 1..4001);
            result.insert(Category::S, 1..4001);
            for condition in conditions.iter() {
                let categ = condition.category;
                let num = condition.compared_to;
                match condition.rule_kind {
                    ComparisonType::GreaterThan => result
                        .entry(categ)
                        .and_modify(|v| v.start = max(num + 1, v.start)),
                    ComparisonType::GreaterEqual => result
                        .entry(categ)
                        .and_modify(|v| v.start = max(num, v.start)),
                    ComparisonType::SmallerThan => result
                        .entry(categ)
                        .and_modify(|v| v.end = min(num - 1, v.end)),
                    ComparisonType::SmallerEqual => {
                        result.entry(categ).and_modify(|v| v.end = min(num, v.end))
                    }
                };
            }
            RangePart {
                x: result.get(&Category::X).unwrap().clone(),
                m: result.get(&Category::M).unwrap().clone(),
                a: result.get(&Category::A).unwrap().clone(),
                s: result.get(&Category::S).unwrap().clone(),
            }
        }

        let conditions = find_conditions_id("in", &conditions_by_id, &mut cache);
        let result = conditions
            .iter()
            .map(|one_conditions| find_acceptable_ranges(one_conditions))
            .powerset()
            .map(|subset| {
                RangePart::intersection_slice(&subset)
                    .map(|range_part| range_part.compute_size())
                    .unwrap_or(0) as i128
                    * if subset.len() % 2 == 0 { -1 } else { 1 }
            })
            .sum();

        result
    }
}
#[derive(Debug)]
struct Workflow {
    id: String,
    rules: Vec<Rule>,
    default_result: RuleResult,
}

impl Workflow {
    fn apply_to(&self, part: &Part) -> RuleResult {
        for rule in self.rules.iter() {
            let result = rule.apply_to(&part);
            if let ActualResult::Rule(rule_result) = result {
                return rule_result;
            }
        }
        return self.default_result.clone();
    }

    fn find_conditions_for_accepting(&self) -> Vec<AcceptingConditionsWithFollowUp> {
        let mut current_conditions = Vec::with_capacity(self.rules.len());
        let mut result = Vec::with_capacity(self.rules.len());
        for rule in self.rules.iter() {
            let valid_condition = rule.rule_kind.compute_same();
            let invalid_condition = rule.rule_kind.compute_opposite();
            let valid_condition = ElementaryCondition {
                category: rule.category,
                rule_kind: valid_condition,
                compared_to: rule.compared_to,
            };
            let invalid_condition = ElementaryCondition {
                category: rule.category,
                rule_kind: invalid_condition,
                compared_to: rule.compared_to,
            };
            match rule.result.clone() {
                RuleResult::Accepted => {
                    let mut all_conditions = current_conditions.clone();
                    all_conditions.push(valid_condition);
                    let accepting_condition = AcceptingConditionsWithFollowUp {
                        conditions: all_conditions,
                        follow_up: FollowUp::Accepted,
                    };
                    result.push(accepting_condition);
                }
                RuleResult::Workflow(id) => {
                    let mut all_conditions = current_conditions.clone();
                    all_conditions.push(valid_condition);
                    let accepting_condition = AcceptingConditionsWithFollowUp {
                        conditions: all_conditions,
                        follow_up: FollowUp::Id(id),
                    };
                    result.push(accepting_condition);
                }
                RuleResult::Rejected => {}
            }
            current_conditions.push(invalid_condition);
        }
        match self.default_result.clone() {
            RuleResult::Accepted => {
                let all_conditions = current_conditions.clone();
                let accepting_condition = AcceptingConditionsWithFollowUp {
                    conditions: all_conditions,
                    follow_up: FollowUp::Accepted,
                };
                result.push(accepting_condition);
            }
            RuleResult::Workflow(id) => {
                let all_conditions = current_conditions.clone();
                let accepting_condition = AcceptingConditionsWithFollowUp {
                    conditions: all_conditions,
                    follow_up: FollowUp::Id(id),
                };
                result.push(accepting_condition);
            }
            RuleResult::Rejected => {}
        }
        result
    }
}

impl From<&str> for Workflow {
    fn from(value: &str) -> Self {
        let (id, rules_part) = value.split_once('{').unwrap();
        let rules_and_default_result = rules_part.trim_end_matches('}').split(',').collect_vec();
        let default_result = rules_and_default_result.last().unwrap().to_string();
        let default_result = default_result.as_str().into();
        let nb_rules = rules_and_default_result.len() - 1;
        let rules = rules_and_default_result
            .into_iter()
            .take(nb_rules)
            .map(|rule_str| rule_str.into())
            .collect_vec();

        Workflow {
            id: id.to_string(),
            rules,
            default_result,
        }
    }
}

#[derive(Debug)]
struct Rule {
    rule_kind: RuleKind,
    compared_to: Num,
    category: Category,
    result: RuleResult,
}

impl Rule {
    fn apply_to(&self, part: &Part) -> ActualResult {
        let num = match self.category {
            Category::X => part.x,
            Category::M => part.m,
            Category::A => part.a,
            Category::S => part.s,
        };
        let condition_accepted = match self.rule_kind {
            RuleKind::GreaterThan => num > self.compared_to,
            RuleKind::SmallerThan => num < self.compared_to,
        };
        if condition_accepted {
            ActualResult::Rule(self.result.clone())
        } else {
            ActualResult::Continue
        }
    }
}

impl From<&str> for Rule {
    fn from(value: &str) -> Self {
        let (rule, result) = value.split_once(':').unwrap();
        let result = result.into();
        let category = match rule.chars().next().unwrap() {
            'x' => Category::X,
            'm' => Category::M,
            'a' => Category::A,
            's' => Category::S,
            _ => panic!("unknown category"),
        };
        let rule_kind = match rule.chars().nth(1).unwrap() {
            '>' => RuleKind::GreaterThan,
            '<' => RuleKind::SmallerThan,
            _ => panic!("unknown rule kind"),
        };
        let compared_to = rule[2..].parse().unwrap();
        Rule {
            rule_kind,
            compared_to,
            category,
            result,
        }
    }
}

#[derive(Debug)]
enum RuleKind {
    GreaterThan,
    SmallerThan,
}

impl RuleKind {
    fn compute_opposite(&self) -> ComparisonType {
        match self {
            RuleKind::GreaterThan => ComparisonType::SmallerEqual,
            RuleKind::SmallerThan => ComparisonType::GreaterEqual,
        }
    }

    fn compute_same(&self) -> ComparisonType {
        match self {
            RuleKind::GreaterThan => ComparisonType::GreaterThan,
            RuleKind::SmallerThan => ComparisonType::SmallerThan,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Category {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Clone)]
enum RuleResult {
    Accepted,
    Rejected,
    Workflow(String),
}

enum ActualResult {
    Rule(RuleResult),
    Continue,
}

impl From<&str> for RuleResult {
    fn from(value: &str) -> Self {
        match value {
            "R" => RuleResult::Rejected,
            "A" => RuleResult::Accepted,
            id => RuleResult::Workflow(id.to_string()),
        }
    }
}

#[derive(Debug)]
struct Part {
    x: Num,
    m: Num,
    a: Num,
    s: Num,
}

impl Part {
    fn get_score(&self) -> Num {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug, Clone)]
struct RangePart {
    x: Range<Num>,
    m: Range<Num>,
    a: Range<Num>,
    s: Range<Num>,
}

fn intersection(range1: &Range<Num>, range2: &Range<Num>) -> Range<Num> {
    max(range1.start, range2.start)..min(range1.end, range2.end)
}

impl RangePart {
    fn new() -> RangePart {
        RangePart {
            x: 1..4001,
            m: 1..4001,
            a: 1..4001,
            s: 1..4001,
        }
    }

    fn intersection(&self, range_part: &RangePart) -> RangePart {
        RangePart {
            x: intersection(&self.x, &range_part.x),
            m: intersection(&self.m, &range_part.m),
            a: intersection(&self.a, &range_part.a),
            s: intersection(&self.s, &range_part.s),
        }
    }

    fn compute_size(&self) -> u128 {
        self.x.try_len().unwrap() as u128
            * self.m.try_len().unwrap() as u128
            * self.a.try_len().unwrap() as u128
            * self.s.try_len().unwrap() as u128
    }

    fn intersection_slice(range_parts: &[RangePart]) -> Option<RangePart> {
        if range_parts.len() == 0 {
            None
        } else {
            Some(
                range_parts
                    .into_iter()
                    .fold(RangePart::new(), |r1, r2| r1.intersection(r2)),
            )
        }
    }
}

impl From<&str> for Part {
    fn from(value: &str) -> Self {
        let pattern: &[_] = &['{', '}'];
        let values = value
            .trim_matches(pattern)
            .split(',')
            .map(|assignment| assignment[2..].parse::<Num>().unwrap())
            .collect::<Vec<_>>();
        Part {
            x: values[0],
            m: values[1],
            a: values[2],
            s: values[3],
        }
    }
}

#[derive(Debug)]
struct AcceptingConditionsWithFollowUp {
    conditions: Vec<ElementaryCondition>,
    follow_up: FollowUp,
}

#[derive(Debug, Clone)]
enum FollowUp {
    Accepted,
    Id(String),
}

#[derive(Debug, Clone)]
struct ElementaryCondition {
    category: Category,
    rule_kind: ComparisonType,
    compared_to: Num,
}

#[derive(Debug, Copy, Clone)]
enum ComparisonType {
    GreaterThan,
    GreaterEqual,
    SmallerThan,
    SmallerEqual,
}

impl Day for Day19 {
    fn make_day(file: File) -> Self {
        let mut contents = String::new();
        let mut buf_reader = std::io::BufReader::new(file);
        buf_reader
            .read_to_string(&mut contents)
            .expect("could read");
        let (workflows, parts) = contents.split_once("\n\n").unwrap();
        let workflows = workflows
            .lines()
            .map(|workflow_str| {
                let workflow: Workflow = workflow_str.into();
                let id = workflow.id.clone();
                (id, workflow)
            })
            .collect();
        let parts = parts.lines().map(|part_str| part_str.into()).collect();
        Day19 { workflows, parts }
    }

    fn solution1(&self) -> String {
        let result = self
            .parts
            .iter()
            .filter(|part| self.does_accept(*part))
            .map(|part| part.get_score())
            .sum::<Num>();
        result.to_string()
    }

    fn solution2(&self) -> String {
        self.find_numbers_of_accepting().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day19_sol1() {
        let input = File::open("./inputs/day19/input_test.txt").expect("File not found");
        let day = Day19::make_day(input);
        assert_eq!(day.solution1(), "19114");
    }

    #[test]
    fn test_day19_sol2() {
        let input = File::open("./inputs/day19/input_test.txt").expect("File not found");
        let day = Day19::make_day(input);
        assert_eq!(day.solution2(), "167409079868000");
    }

    #[test]
    fn test_find_conditions_for_accepting() {
        let input = File::open("./inputs/day19/input_test.txt").expect("File not found");
        let day = Day19::make_day(input);
        dbg!(day.find_numbers_of_accepting());
    }
}
