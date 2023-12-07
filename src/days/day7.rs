use std::cmp::Ordering;
use std::fs::File;
use std::io::BufRead;
use std::str::FromStr;

use crate::days::Day;

const JOKER: u8 = 11;

type Card = u8;
type Bid = u64;

pub struct Day7 {
    hands: Vec<Hand>,
}

#[derive(Debug, Clone)]
struct Hand {
    cards: HandCards,
    bid: Bid,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct HandCards([Card; 5]);

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

impl HandCards {
    fn compute_hand_type(&self) -> HandType {
        let mut cards_map = vec![0; 15];
        for card in self.0.iter() {
            cards_map[*card as usize] += 1;
        }
        let mut retrieved_values = cards_map.iter().filter(|x| **x >= 1).collect::<Vec<_>>();
        retrieved_values.sort_unstable();
        retrieved_values.reverse();
        match (retrieved_values[0], retrieved_values.get(1).unwrap_or(&&0)) {
            (5, 0) => HandType::Five,
            (4, 1) => HandType::Four,
            (3, 2) => HandType::FullHouse,
            (3, 1) => HandType::Three,
            (2, 2) => HandType::TwoPair,
            (2, 1) => HandType::OnePair,
            (1, 1) => HandType::HighCard,
            _ => panic!("invalid hand"),
        }
    }

    fn compute_hand_type_joker(&self) -> HandType {
        let mut cards_map = vec![0; 15];
        let mut nb_jokers = 0;
        for card in self.0.iter() {
            if *card == JOKER {
                nb_jokers += 1;
            } else {
                cards_map[*card as usize] += 1;
            }
        }
        let mut retrieved_values = cards_map.iter().filter(|x| **x >= 1).collect::<Vec<_>>();
        retrieved_values.sort_unstable();
        retrieved_values.reverse();
        match (
            5.min(**retrieved_values.get(0).unwrap_or(&&0) + nb_jokers),
            retrieved_values.get(1).unwrap_or(&&0),
        ) {
            (5, 0) => HandType::Five,
            (4, 1) => HandType::Four,
            (3, 2) => HandType::FullHouse,
            (3, 1) => HandType::Three,
            (2, 2) => HandType::TwoPair,
            (2, 1) => HandType::OnePair,
            (1, 1) => HandType::HighCard,
            _ => panic!("invalid hand"),
        }
    }

    fn compare(&self, other: &Self, use_joker: bool) -> Ordering {
        let type_ordering = if use_joker {
            self.compute_hand_type_joker()
                .cmp(&other.compute_hand_type_joker())
        } else {
            self.compute_hand_type().cmp(&other.compute_hand_type())
        };
        if type_ordering.is_ne() {
            return type_ordering;
        } else {
            for (card1, card2) in self.0.iter().zip(other.0.iter()) {
                let card1_value = if *card1 == JOKER { 0 } else { *card1 };
                let card2_value = if *card2 == JOKER { 0 } else { *card2 };
                let card_ordering = card1_value.cmp(&card2_value);
                if card_ordering.is_ne() {
                    return card_ordering;
                }
            }
            type_ordering
        }
    }
}

impl FromStr for Hand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards_str, bid_str) = s.split_once(' ').ok_or("should have a whitespace")?;
        let mut cards = HandCards([0; 5]);
        cards_str
            .chars()
            .enumerate()
            .for_each(|(idx, c)| cards.0[idx] = char_to_card(c).expect("should be card char"));
        let bid = bid_str.parse().map_err(|_| "should be number")?;
        let hand = Hand { cards, bid };
        Ok(hand)
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    Three,
    FullHouse,
    Four,
    Five,
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
        let mut hands_sorted = self.hands.clone();
        hands_sorted.sort_unstable_by(|hand1, hand2| hand1.cards.compare(&hand2.cards, false));
        let result: u64 = hands_sorted
            .iter()
            .enumerate()
            .map(|(idx, hand)| (idx + 1) as u64 * hand.bid)
            .sum();
        result.to_string()
    }

    fn solution2(&self) -> String {
        let mut hands_sorted = self.hands.clone();
        hands_sorted.sort_unstable_by(|hand1, hand2| hand1.cards.compare(&hand2.cards, true));
        let result: u64 = hands_sorted
            .iter()
            .enumerate()
            .map(|(idx, hand)| (idx + 1) as u64 * hand.bid)
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
