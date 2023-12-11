use std::fs::File;
use std::io::BufRead;

use crate::days::Day;

type Coordinate = (usize, usize);

pub struct Day11 {
    galaxies: Vec<Coordinate>,
}

impl Day11 {
    fn get_result(&self, factor: usize) -> usize {
        let length = self.galaxies.len();
        let mut present_rows = Vec::with_capacity(length);
        let mut present_cols = Vec::with_capacity(length);
        for (row, col) in self.galaxies.iter() {
            present_rows.push(*row);
            present_cols.push(*col);
        }
        let rows_to_expand = get_missing_values(present_rows);
        let cols_to_expand = get_missing_values(present_cols);

        let mut result = 0;
        for i in 0..self.galaxies.len() {
            for j in i + 1..self.galaxies.len() {
                result += distance(
                    self.galaxies[i],
                    self.galaxies[j],
                    &rows_to_expand,
                    &cols_to_expand,
                    factor,
                )
            }
        }
        result
    }
}
fn get_missing_values(mut vec: Vec<usize>) -> Vec<usize> {
    vec.sort_unstable();
    let min = vec[0];
    let max = vec[vec.len() - 1];
    (min + 1..max)
        .filter(|v| vec.binary_search(v).is_err())
        .collect()
}

fn distance(
    p1: Coordinate,
    p2: Coordinate,
    rows_to_expand: &Vec<usize>,
    cols_to_expand: &Vec<usize>,
    factor: usize,
) -> usize {
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    let rows_between = number_of_expanded_between(&rows_to_expand, x1, x2) * (factor - 1);
    let cols_between = number_of_expanded_between(&cols_to_expand, y1, y2) * (factor - 1);
    x1.abs_diff(x2) + y1.abs_diff(y2) + rows_between + cols_between
}

fn number_of_expanded_between(expanded: &Vec<usize>, r1: usize, r2: usize) -> usize {
    let row_min = r1.min(r2);
    let row_max = r1.max(r2);
    expanded
        .iter()
        .skip_while(|row| **row < row_min)
        .take_while(|row| **row < row_max)
        .collect::<Vec<_>>()
        .len()
}
impl Day for Day11 {
    fn make_day(file: File) -> Self {
        let mut galaxies = Vec::new();
        std::io::BufReader::new(file)
            .lines()
            .enumerate()
            .for_each(|(row, line)| {
                line.expect("doc should have lines")
                    .chars()
                    .enumerate()
                    .for_each(|(col, c)| {
                        if c == '#' {
                            galaxies.push((row, col));
                        }
                    })
            });
        Day11 { galaxies }
    }

    fn solution1(&self) -> String {
        let result = self.get_result(2);
        result.to_string()
    }

    fn solution2(&self) -> String {
        let result = self.get_result(1000000);
        result.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day11_sol1() {
        let input = File::open("./inputs/day11/input_test.txt").expect("File not found");
        let day = Day11::make_day(input);
        assert_eq!(day.solution1(), "374");
    }

    #[test]
    fn test_day11_sol2() {
        let input = File::open("./inputs/day11/input_test.txt").expect("File not found");
        let day = Day11::make_day(input);
        assert_eq!(day.get_result(100), 8410);
    }
}
