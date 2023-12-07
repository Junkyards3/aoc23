use std::fs::File;
use std::io::BufRead;
use std::iter;
use std::str::FromStr;

use crate::days::Day;

const JOKER: u8 = 11;
const VALUE_TO_HEX_CLASSIC: [char; 15] = [
    ' ', ' ', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E',
];
const VALUE_TO_HEX_JOKER: [char; 15] = [
    ' ', ' ', '2', '3', '4', '5', '6', '7', '8', '9', 'A', '0', 'C', 'D', 'E',
];

type Card = u8;
type Bid = u64;

pub struct Day7 {
    hands: Vec<Hand>,
}

#[derive(Debug, Clone)]
struct Hand {
    cards: [Card; 5],
    bid: Bid,
}

fn char_to_card(c: char) -> Result<Card, ()> {
    if ('2'..='9').contains(&c) {
        c.to_digit(10).map(|digit| digit as Card).ok_or(())
    } else {
        let result = match c {
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => return Err(()),
        };
        Ok(result)
    }
}

fn compute_card_numbers_from_cards_map(cards_map: [usize; 15]) -> (usize, usize) {
    let mut retrieved_values = cards_map
        .into_iter()
        .filter(|x| *x >= 1)
        .collect::<Vec<usize>>();
    retrieved_values.sort_unstable();
    retrieved_values.reverse();
    let mut card_numbers = retrieved_values.into_iter().take(2).chain(iter::repeat(0));
    (card_numbers.next().unwrap(), card_numbers.next().unwrap())
}

impl Hand {
    fn compute_hand_hex_value(&self) -> String {
        self.cards
            .iter()
            .map(|d| VALUE_TO_HEX_CLASSIC[*d as usize])
            .collect::<String>()
    }

    fn compute_hand_hex_value_joker(&self) -> String {
        self.cards
            .iter()
            .map(|d| VALUE_TO_HEX_JOKER[*d as usize])
            .collect::<String>()
    }

    fn compute_hand_power(&self) -> u32 {
        let mut cards_map = [0; 15];
        for card in self.cards.iter() {
            cards_map[*card as usize] += 1;
        }
        let (max_card_number, second_max_card_number) =
            compute_card_numbers_from_cards_map(cards_map);
        let char_hand_type = get_hex_power_from_nb_cards(max_card_number, second_max_card_number);
        let hex_from_hand_values = self.compute_hand_hex_value();
        u32::from_str_radix(&format!("{}{}", char_hand_type, hex_from_hand_values), 16)
            .expect("is not hex number")
    }

    fn compute_hand_power_joker(&self) -> u32 {
        let mut cards_map = [0; 15];
        let mut nb_jokers = 0;
        for card in self.cards.iter() {
            if *card == JOKER {
                nb_jokers += 1;
            } else {
                cards_map[*card as usize] += 1;
            }
        }
        let (max_card_number, second_max_card_number) =
            compute_card_numbers_from_cards_map(cards_map);
        let char_hand_type =
            get_hex_power_from_nb_cards(max_card_number + nb_jokers, second_max_card_number);
        let hex_from_hand_values = self.compute_hand_hex_value_joker();
        u32::from_str_radix(&format!("{}{}", char_hand_type, hex_from_hand_values), 16)
            .expect("is not hex number")
    }
}

impl FromStr for Hand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards_str, bid_str) = s.split_once(' ').ok_or("should have a whitespace")?;
        let mut cards = [0; 5];
        cards_str
            .chars()
            .enumerate()
            .for_each(|(idx, c)| cards[idx] = char_to_card(c).expect("should be card char"));
        let bid = bid_str.parse().map_err(|_| "should be number")?;
        let hand = Hand { cards, bid };
        Ok(hand)
    }
}

fn get_hex_power_from_nb_cards(nb_max: usize, nb_second_max: usize) -> char {
    match (nb_max, nb_second_max) {
        (5, 0) => '6',
        (4, 1) => '5',
        (3, 2) => '4',
        (3, 1) => '3',
        (2, 2) => '2',
        (2, 1) => '1',
        (1, 1) => '0',
        _ => panic!("invalid card numbers for a hand"),
    }
}

impl Day for Day7 {
    fn make_day(file: File) -> Self {
        let hands = std::io::BufReader::new(file)
            .lines()
            .map(|line| {
                line.expect("doc should have lines")
                    .parse()
                    .expect("should be a hand with bid")
            })
            .collect();
        Day7 { hands }
    }

    fn solution1(&self) -> String {
        let mut hands_sorted = self
            .hands
            .iter()
            .map(|hand| (hand, hand.compute_hand_power()))
            .collect::<Vec<_>>();
        hands_sorted.sort_unstable_by_key(|(_, power)| *power);
        let result: u64 = hands_sorted
            .iter()
            .enumerate()
            .map(|(idx, (hand, _))| (idx + 1) as u64 * hand.bid)
            .sum();
        result.to_string()
    }

    fn solution2(&self) -> String {
        let mut hands_sorted = self
            .hands
            .iter()
            .map(|hand| (hand, hand.compute_hand_power_joker()))
            .collect::<Vec<_>>();
        hands_sorted.sort_unstable_by_key(|(_, power)| *power);
        let result: u64 = hands_sorted
            .iter()
            .enumerate()
            .map(|(idx, (hand, _))| (idx + 1) as u64 * hand.bid)
            .sum();
        result.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day7_sol1() {
        let input = File::open("./inputs/day7/input_test.txt").expect("File not found");
        let day = Day7::make_day(input);
        assert_eq!(day.solution1(), "6440");
    }

    #[test]
    fn test_day7_sol2() {
        let input = File::open("./inputs/day7/input_test.txt").expect("File not found");
        let day = Day7::make_day(input);
        assert_eq!(day.solution2(), "5905");
    }
}
