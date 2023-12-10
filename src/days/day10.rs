use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;

use crate::days::Day;

#[derive(Debug)]
pub struct Day10 {
    field: Vec<Vec<Tile>>,
    start_pos: (usize, usize),
}

impl Day10 {
    fn get_tile(&self, row: isize, col: isize) -> Option<&Tile> {
        if row < 0 || col < 0 {
            None
        } else {
            self.field
                .get(row as usize)
                .and_then(|v: &Vec<Tile>| v.get(col as usize))
        }
    }
}

#[derive(Debug)]
struct Walker {
    curr_pos: (usize, usize),
    is_still_walking: bool,
    last_move: Direction,
    curr_tile: Tile,
    id: String,
}

#[derive(Debug)]
struct VisitedTile {
    tile: Tile,
    pos: (usize, usize),
    walker_id: String,
}

#[derive(Debug)]
struct Walkers<'a> {
    valid_walkers: Vec<Walker>,
    invalid_walkers: Vec<Walker>,
    day: &'a Day10,
    nb_steps: u32,
    finished_steps: Option<u32>,
    visited_tiles: Vec<VisitedTile>,
}

impl<'a> Walkers<'a> {
    fn create_walkers(day: &'a Day10) -> Self {
        let mut visited_tiles = Vec::with_capacity(day.field.len() * day.field[0].len());
        let (valid_walkers, invalid_walkers): (Vec<Walker>, Vec<Walker>) = [
            Direction::West,
            Direction::South,
            Direction::East,
            Direction::North,
        ]
        .into_iter()
        .zip(["A", "B", "C", "D"].into_iter())
        .map(|(dir, id)| {
            let (row, col) = dir.to_next_tile(day.start_pos);
            let tile = day.get_tile(row, col);
            if tile.is_some_and(|tile| tile.connects_to(dir.opposite())) {
                Walker {
                    curr_pos: (row as usize, col as usize),
                    is_still_walking: true,
                    last_move: dir,
                    curr_tile: tile.unwrap().clone(),
                    id: id.to_string(),
                }
            } else {
                Walker {
                    curr_pos: day.start_pos,
                    is_still_walking: false,
                    last_move: dir,
                    curr_tile: Tile::Start,
                    id: id.to_string(),
                }
            }
        })
        .partition(|walker| walker.is_still_walking);

        for walker in valid_walkers.iter() {
            visited_tiles.push(VisitedTile {
                tile: walker.curr_tile,
                pos: walker.curr_pos,
                walker_id: walker.id.to_string(),
            });
            visited_tiles.push(VisitedTile {
                tile: Tile::Start,
                pos: day.start_pos,
                walker_id: walker.id.to_string(),
            });
        }
        Walkers {
            valid_walkers,
            invalid_walkers,
            day,
            nb_steps: 1,
            finished_steps: None,
            visited_tiles,
        }
    }

    fn walk(&mut self) {
        let (still_valid, mut new_invalid) = self
            .valid_walkers
            .iter()
            .map(|walker| {
                let dir_to_take = walker
                    .curr_tile
                    .other_direction(walker.last_move.opposite())
                    .expect("tile should be pipe");
                let (row, col) = dir_to_take.to_next_tile(walker.curr_pos);
                let tile = self.day.get_tile(row, col);
                if tile.is_some_and(|tile| tile.connects_to(dir_to_take.opposite())) {
                    Walker {
                        curr_pos: (row as usize, col as usize),
                        is_still_walking: true,
                        last_move: dir_to_take,
                        curr_tile: tile.unwrap().clone(),
                        id: walker.id.to_string(),
                    }
                } else {
                    Walker {
                        curr_pos: walker.curr_pos,
                        is_still_walking: false,
                        last_move: walker.last_move,
                        curr_tile: walker.curr_tile,
                        id: walker.id.to_string(),
                    }
                }
            })
            .partition(|walker| walker.is_still_walking);

        self.valid_walkers = still_valid;
        self.invalid_walkers.append(&mut new_invalid);

        self.nb_steps += 1;

        if self.valid_walkers.len() < 2 {
            panic!("there should be a loop");
        }

        for walker in self.valid_walkers.iter() {
            self.visited_tiles.push(VisitedTile {
                tile: walker.curr_tile,
                pos: walker.curr_pos,
                walker_id: walker.id.to_string(),
            });
        }

        for i in 0..self.valid_walkers.len() {
            for j in i + 1..self.valid_walkers.len() {
                if self.valid_walkers[i].curr_pos == self.valid_walkers[j].curr_pos {
                    self.finished_steps = Some(self.nb_steps)
                }
            }
        }
    }

    fn walk_until_finished(&mut self) -> u32 {
        while self.finished_steps.is_none() {
            self.walk();
        }
        self.finished_steps.unwrap()
    }

    fn walk_until_loop_build(&mut self) -> HashMap<(usize, usize), Tile> {
        while self.finished_steps.is_none() {
            self.walk();
        }
        let valid_ids = self
            .valid_walkers
            .iter()
            .map(|walker| walker.id.clone())
            .collect::<Vec<_>>();
        let mut map_tiles: HashMap<(usize, usize), Tile> = self
            .visited_tiles
            .iter()
            .filter(|visited_tile| valid_ids.contains(&visited_tile.walker_id))
            .map(|visited_tile| (visited_tile.pos, visited_tile.tile))
            .collect();

        //transform start into pipe
        let start_pipe_dirs = [
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::East,
        ]
        .into_iter()
        .filter(|dir| {
            let (r, c) = dir.to_next_tile(self.day.start_pos);
            if r < 0 || c < 0 {
                false
            } else {
                map_tiles.contains_key(&(r as usize, c as usize))
            }
        })
        .collect::<Vec<_>>();
        map_tiles.insert(
            self.day.start_pos,
            Tile::Pipe((start_pipe_dirs[0], start_pipe_dirs[1])),
        );
        map_tiles
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    fn to_next_tile(&self, pos: (usize, usize)) -> (isize, isize) {
        let (r, c) = (pos.0 as isize, pos.1 as isize);
        match self {
            Direction::North => (r - 1, c),
            Direction::West => (r, c - 1),
            Direction::South => (r + 1, c),
            Direction::East => (r, c + 1),
        }
    }

    fn opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::West => Direction::East,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Tile {
    Pipe((Direction, Direction)),
    Ground,
    Start,
}

impl Tile {
    fn connects_to(&self, dir: Direction) -> bool {
        match self {
            Tile::Pipe((d1, d2)) => *d1 == dir || *d2 == dir,
            _ => false,
        }
    }

    fn other_direction(&self, dir: Direction) -> Option<Direction> {
        match self {
            Tile::Pipe((d1, d2)) => {
                if dir == *d1 {
                    Some(*d2)
                } else if dir == *d2 {
                    Some(*d1)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '|' => Tile::Pipe((Direction::North, Direction::South)),
            '-' => Tile::Pipe((Direction::West, Direction::East)),
            'L' => Tile::Pipe((Direction::North, Direction::East)),
            'J' => Tile::Pipe((Direction::North, Direction::West)),
            '7' => Tile::Pipe((Direction::West, Direction::South)),
            'F' => Tile::Pipe((Direction::South, Direction::East)),
            '.' => Tile::Ground,
            'S' => Tile::Start,
            c => panic!("incorrect tile : {}", c),
        }
    }
}

fn is_inside(walls: &HashMap<(usize, usize), Tile>, pos: (usize, usize)) -> bool {
    if walls.contains_key(&pos) {
        return false;
    }

    let mut is_crossing = false;
    let (r, c) = pos;
    let mut coming_from: Option<Direction> = None;

    for k in (0..c).rev() {
        if let Some(Tile::Pipe((d1, d2))) = walls.get(&(r, k)) {
            match (*d1, *d2) {
                (Direction::North, Direction::South) | (Direction::South, Direction::North) => {
                    is_crossing = !is_crossing;
                }
                (Direction::East, Direction::West) | (Direction::West, Direction::East) => {}
                (Direction::North, _) | (_, Direction::North) => {
                    if let Some(d) = coming_from {
                        if d == Direction::South {
                            is_crossing = !is_crossing
                        }
                        coming_from = None
                    } else {
                        coming_from = Some(Direction::North)
                    }
                }
                (Direction::South, _) | (_, Direction::South) => {
                    if let Some(d) = coming_from {
                        if d == Direction::North {
                            is_crossing = !is_crossing
                        }
                        coming_from = None
                    } else {
                        coming_from = Some(Direction::South)
                    }
                }
                x => panic!("{:?} cannot happen", x),
            }
        }
    }
    is_crossing
}

impl Day for Day10 {
    fn make_day(file: File) -> Self {
        let mut start_pos = (0, 0);
        let field: Vec<Vec<Tile>> = std::io::BufReader::new(file)
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.expect("doc should have lines")
                    .chars()
                    .enumerate()
                    .map(|(x, c)| {
                        if c == 'S' {
                            start_pos = (y, x)
                        }
                        c.into()
                    })
                    .collect::<Vec<Tile>>()
            })
            .collect();
        Day10 { field, start_pos }
    }

    fn solution1(&self) -> String {
        let mut walkers = Walkers::create_walkers(&self);
        let result = walkers.walk_until_finished();
        result.to_string()
    }

    fn solution2(&self) -> String {
        let mut walkers = Walkers::create_walkers(&self);
        let map = walkers.walk_until_loop_build();
        let mut result = 0;
        for r in 0..self.field.len() {
            for c in 0..self.field[0].len() {
                if is_inside(&map, (r, c)) {
                    result += 1;
                }
            }
        }
        result.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day10_sol1() {
        let input = File::open("./inputs/day10/input_test.txt").expect("File not found");
        let day = Day10::make_day(input);
        assert_eq!(day.solution1(), "8");
    }

    #[test]
    fn test_day10_sol2() {
        let input = File::open("./inputs/day10/input_test.txt").expect("File not found");
        let day = Day10::make_day(input);
        assert_eq!(day.solution2(), "sol2");
    }

    #[test]
    fn test_day10_sol2_2() {
        let input = File::open("./inputs/day10/input_test2.txt").expect("File not found");
        let day = Day10::make_day(input);
        assert_eq!(day.solution2(), "4");
    }
}
