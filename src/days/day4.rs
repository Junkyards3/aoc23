use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::str::FromStr;

use crate::days::Day;

pub struct Day4 {
    cards: Vec<Card>,
}

type CardNumber = u32;

#[derive(Debug)]
struct Card {
    winning_numbers: Vec<CardNumber>,
    found_numbers: Vec<CardNumber>,
}

impl Card {
    fn compute_number_of_winning_in_found_numbers(&self) -> u32 {
        self.winning_numbers
            .iter()
            .filter(|winning_number| self.found_numbers.contains(winning_number))
            .count() as u32
    }

    fn compute_score(&self) -> u32 {
        match self.compute_number_of_winning_in_found_numbers() {
            0 => 0,
            n => 2u32.pow(n - 1),
        }
    }
}
impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let first_split = s.split(&[':', '|']).skip(1).collect::<Vec<_>>();
        let winning_numbers = first_split[0]
            .split_whitespace()
            .map(|nb_str| {
                nb_str
                    .parse()
                    .expect("number in winning numbers part expected")
            })
            .collect();
        let found_numbers = first_split[1]
            .split_whitespace()
            .map(|nb_str| {
                nb_str
                    .parse()
                    .expect("number in found numbers part expected")
            })
            .collect();
        Ok(Card {
            winning_numbers,
            found_numbers,
        })
    }
}
impl Day for Day4 {
    fn make_day(file: File) -> Self {
        let cards = std::io::BufReader::new(file)
            .lines()
            .map(|line| line.expect("doc should have lines").parse().unwrap())
            .collect();
        Day4 { cards }
    }

    fn solution1(&self) -> String {
        let result: u32 = self.cards.iter().map(|card| card.compute_score()).sum();
        result.to_string()
    }

    fn solution2(&self) -> String {
        let mut nb_cards: HashMap<usize, u32> = HashMap::with_capacity(self.cards.len());
        for (card_index, card) in self.cards.iter().enumerate() {
            let nb_card = *nb_cards.entry(card_index).or_insert(1);
            (1..=card.compute_number_of_winning_in_found_numbers()).for_each(|card_offset| {
                *nb_cards
                    .entry(card_index + card_offset as usize)
                    .or_insert(1) += nb_card;
            })
        }
        let result: u32 = nb_cards.values().sum();
        result.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day4_sol1() {
        let input = File::open("./inputs/day4/input_test.txt").expect("File not found");
        let day4 = Day4::make_day(input);
        assert_eq!(day4.solution1(), "13");
    }

    #[test]
    fn test_day4_sol2() {
        let input = File::open("./inputs/day4/input_test.txt").expect("File not found");
        let day4 = Day4::make_day(input);
        assert_eq!(day4.solution2(), "30");
    }
}
