use std::fs::File;
use std::io::Read;

use crate::days::Day;

type Num = u32;

pub struct Day15 {
    steps: Vec<String>,
}

#[derive(Debug, Clone)]
struct Lens {
    label: String,
    focal_length: Num,
}

enum Instruction {
    Put(Lens),
    Remove { label: String },
}

impl Instruction {
    fn get_label(&self) -> &str {
        match self {
            Instruction::Put(lens) => &lens.label,
            Instruction::Remove { label } => &label,
        }
    }
}

impl From<&String> for Instruction {
    fn from(value: &String) -> Self {
        if let Some((label, length)) = value.split_once('=') {
            Instruction::Put(Lens {
                label: label.to_string(),
                focal_length: length.parse().unwrap(),
            })
        } else {
            Instruction::Remove {
                label: value[0..value.len() - 1].to_string(),
            }
        }
    }
}

fn hash_algorithm(s: &str) -> Num {
    s.chars()
        .fold(0, |current, ch| (17 * (current + (ch as Num))) & 255)
}

impl Day for Day15 {
    fn make_day(file: File) -> Self {
        let mut contents = String::new();
        let mut buf_reader = std::io::BufReader::new(file);
        buf_reader
            .read_to_string(&mut contents)
            .expect("could read");
        let steps = contents
            .trim()
            .split(',')
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        Day15 { steps }
    }

    fn solution1(&self) -> String {
        let result = self
            .steps
            .iter()
            .map(|step| hash_algorithm(step))
            .sum::<Num>();
        result.to_string()
    }

    fn solution2(&self) -> String {
        let mut boxes: Vec<Vec<Lens>> = vec![vec![]; 256];
        for step in self.steps.iter() {
            let instruction = Instruction::from(step);
            let hash = hash_algorithm(instruction.get_label());
            match instruction {
                Instruction::Put(lens_to_insert) => match boxes[hash as usize]
                    .iter()
                    .position(|lens| lens.label == lens_to_insert.label)
                {
                    Some(pos) => {
                        boxes[hash as usize][pos].focal_length = lens_to_insert.focal_length;
                    }
                    None => {
                        boxes[hash as usize].push(lens_to_insert);
                    }
                },
                Instruction::Remove { label } => {
                    boxes[hash as usize].retain(|lens| lens.label != label);
                }
            }
        }
        let result = boxes
            .iter()
            .enumerate()
            .flat_map(|(box_index, box_lenses)| {
                box_lenses
                    .iter()
                    .enumerate()
                    .map(move |(lens_index, lens)| {
                        (box_index as Num + 1) * (lens_index as Num + 1) * lens.focal_length
                    })
            })
            .sum::<Num>();
        result.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day15_sol1() {
        let input = File::open("./inputs/day15/input_test.txt").expect("File not found");
        let day = Day15::make_day(input);
        assert_eq!(day.solution1(), "1320");
    }

    #[test]
    fn test_day15_sol2() {
        let input = File::open("./inputs/day15/input_test.txt").expect("File not found");
        let day = Day15::make_day(input);
        assert_eq!(day.solution2(), "145");
    }

    #[test]
    fn test_hash() {
        assert_eq!(hash_algorithm("HASH"), 52);
        assert_eq!(hash_algorithm("rn=1"), 30);
    }
}
