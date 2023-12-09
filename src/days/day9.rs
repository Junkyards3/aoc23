use std::fs::File;
use std::io::BufRead;

use crate::days::Day;

type Number = i32;

pub struct Day9 {
    data: Vec<Sequence>,
    binomials: Binomial,
}

struct Sequence {
    values: Vec<Number>,
}

impl Sequence {
    fn compute_higher_differences(&self, binomial: &Binomial) -> Vec<Number> {
        let mut result: Vec<Number> = (0..self.values.len() - 1)
            .zip([1, -1].into_iter().cycle())
            .into_iter()
            .map(|(k, minus_power)| {
                (0..=k)
                    .zip([minus_power, -minus_power].into_iter().cycle())
                    .map(|(i, minus_one_power)| {
                        self.values[i] * binomial.coefficients[k][i] * minus_one_power
                    })
                    .sum::<Number>()
            })
            .collect();
        while result.last().is_some_and(|v| *v == 0) {
            result.pop();
        }
        result
    }

    fn compute_next_value(&self, binomial: &Binomial) -> Number {
        let higher_differences = self.compute_higher_differences(binomial);
        let n = self.values.len();
        higher_differences
            .into_iter()
            .enumerate()
            .map(|(k, delta)| delta * binomial.coefficients[n][k])
            .sum()
    }

    fn reverse(&self) -> Self {
        let mut values_reverse = self.values.clone();
        values_reverse.reverse();
        Self {
            values: values_reverse,
        }
    }
}

#[derive(Debug)]
struct Binomial {
    coefficients: Vec<Vec<Number>>,
}

impl Binomial {
    fn construct_from_n_value(n: usize) -> Binomial {
        let mut coefficients = Vec::with_capacity(n + 1);
        coefficients.push(vec![1]);
        for k in 1..=n {
            let mut line = Vec::with_capacity(k + 1);
            line.push(1);
            for j in 1..k {
                line.push(coefficients[k - 1][j - 1] + coefficients[k - 1][j])
            }
            line.push(1);
            coefficients.push(line);
        }
        Binomial { coefficients }
    }
}
impl Day for Day9 {
    fn make_day(file: File) -> Self {
        let data: Vec<Sequence> = std::io::BufReader::new(file)
            .lines()
            .map(|line| {
                let values = line
                    .expect("doc should have lines")
                    .split_whitespace()
                    .map(|nb_str| nb_str.parse().expect("should be num"))
                    .collect::<Vec<_>>();
                Sequence { values }
            })
            .collect();
        let numbers_needed = 1 + data
            .iter()
            .map(|line| line.values.len())
            .max()
            .expect("there is a line");
        let binomials = Binomial::construct_from_n_value(numbers_needed);
        Day9 { data, binomials }
    }

    fn solution1(&self) -> String {
        let result = self
            .data
            .iter()
            .map(|seq| seq.compute_next_value(&self.binomials))
            .sum::<Number>();
        result.to_string()
    }

    fn solution2(&self) -> String {
        let result = self
            .data
            .iter()
            .map(|seq| seq.reverse().compute_next_value(&self.binomials))
            .sum::<Number>();
        result.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_next_value() {
        let seq = Sequence {
            values: vec![1, 3, 6, 10, 15, 21],
        };
        let binomial = Binomial::construct_from_n_value(7);
        assert_eq!(seq.compute_next_value(&binomial), 28);
    }

    #[test]
    fn test_compute_previous_value() {
        let seq = Sequence {
            values: vec![10, 13, 16, 21, 30, 45],
        }
        .reverse();
        let binomial = Binomial::construct_from_n_value(7);
        assert_eq!(seq.compute_next_value(&binomial), 5);
    }

    #[test]
    fn test_day9_sol1() {
        let input = File::open("./inputs/day9/input_test.txt").expect("File not found");
        let day = Day9::make_day(input);
        assert_eq!(day.solution1(), "114");
    }

    #[test]
    fn test_day9_sol2() {
        let input = File::open("./inputs/day9/input_test.txt").expect("File not found");
        let day = Day9::make_day(input);
        assert_eq!(day.solution2(), "2");
    }
}
