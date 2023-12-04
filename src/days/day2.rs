use std::cmp::max;
use std::fs::File;
use std::io::BufRead;
use std::str::FromStr;

use crate::days::Day;

#[derive(Debug)]
pub struct Day2 {
    games: Vec<Game>,
}

#[derive(Debug)]
struct Game {
    grabs: Vec<CubeGrab>,
    id: u32,
}

impl Game {
    fn get_minimum_dice_bag(&self) -> [u32; 3] {
        let mut dice_bag = [0; 3];
        for grab in self.grabs.iter() {
            dice_bag
                .iter_mut()
                .zip(grab.nb_cubes_by_color.iter())
                .for_each(|(dice_bag_amount, grab_amount)| {
                    *dice_bag_amount = max(*dice_bag_amount, *grab_amount);
                });
        }
        dice_bag
    }

    fn get_power(&self) -> u32 {
        self.get_minimum_dice_bag().iter().product()
    }
}

#[derive(Debug)]
struct CubeGrab {
    nb_cubes_by_color: [u32; 3],
}

impl CubeGrab {
    fn is_acceptable(&self, red: u32, green: u32, blue: u32) -> bool {
        red >= self.nb_cubes_by_color[CubeColor::Red as usize]
            && green >= self.nb_cubes_by_color[CubeColor::Green as usize]
            && blue >= self.nb_cubes_by_color[CubeColor::Blue as usize]
    }
}

enum CubeColor {
    Red = 0,
    Green = 1,
    Blue = 2,
}

impl FromStr for Game {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let identifier_split = s.split(':').collect::<Vec<_>>();
        let id = identifier_split[0]
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let grabs = identifier_split[1]
            .split(';')
            .map(|grab| grab.parse().unwrap())
            .collect();
        Ok(Game { grabs, id })
    }
}

impl FromStr for CubeGrab {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let colors_split = s.split(',');
        let mut nb_cubes_by_color = [0; 3];
        for color in colors_split {
            let mut nb_split = color.split_whitespace();
            let nb = nb_split.next().unwrap().parse().unwrap();
            let color = match nb_split.next().unwrap() {
                "red" => CubeColor::Red,
                "blue" => CubeColor::Blue,
                "green" => CubeColor::Green,
                _ => return Err("wrong color".to_string()),
            };
            nb_cubes_by_color[color as usize] = nb;
        }
        let result = CubeGrab { nb_cubes_by_color };
        Ok(result)
    }
}

impl Day for Day2 {
    fn make_day(file: File) -> Self {
        let games = std::io::BufReader::new(file)
            .lines()
            .map(|line| line.expect("doc should have lines").parse().unwrap())
            .collect();
        Day2 { games }
    }

    fn solution1(&self) -> String {
        let result: u32 = self
            .games
            .iter()
            .filter(|game| game.grabs.iter().all(|grab| grab.is_acceptable(12, 13, 14)))
            .map(|game| game.id)
            .sum();
        result.to_string()
    }

    fn solution2(&self) -> String {
        let result: u32 = self.games.iter().map(|game| game.get_power()).sum();
        result.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day2_sol1() {
        let games = [
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ]
        .iter()
        .map(|line| line.parse().unwrap())
        .collect();
        let day2 = Day2 { games };
        assert_eq!(day2.solution1(), "8");
    }

    #[test]
    fn test_day2_sol2() {
        let games = [
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ]
        .iter()
        .map(|line| line.parse().unwrap())
        .collect();
        let day2 = Day2 { games };
        assert_eq!(day2.solution2(), "2286");
    }
}
