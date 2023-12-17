use std::cmp::max;
use std::fs::File;
use std::io::BufRead;

use ndarray::Array2;
use rayon::prelude::*;

use crate::days::Day;

type Coordinate = (isize, isize);

pub struct Day16 {
    contraptions: Array2<Option<Contraption>>,
}

impl Day16 {
    fn compute_energized_cells(&self, start_ray: Ray) -> usize {
        let (nb_rows, nb_cols) = self.contraptions.dim();
        let (nb_rows, nb_cols) = (nb_rows as usize, nb_cols as usize);
        let mut rays_cache = Array2::default(self.contraptions.dim());
        let mut energized = Array2::default(self.contraptions.dim());
        let mut current_rays = vec![start_ray];
        while !current_rays.is_empty() {
            let mut new_rays = Vec::with_capacity(2 * current_rays.len());
            for ray in current_rays.iter() {
                if let Some((r, c)) = ray.get_next_cell(nb_rows, nb_cols) {
                    if let Some(contraption) = self.contraptions[(r as usize, c as usize)] {
                        match contraption.get_leaving_directions(ray.direction) {
                            LeavingDirections::One(dir) => {
                                add_if_not_in_cache(
                                    Ray {
                                        direction: dir,
                                        pos: (r, c),
                                    },
                                    &mut rays_cache,
                                    &mut new_rays,
                                );
                            }
                            LeavingDirections::Two { dir1, dir2 } => {
                                add_if_not_in_cache(
                                    Ray {
                                        direction: dir1,
                                        pos: (r, c),
                                    },
                                    &mut rays_cache,
                                    &mut new_rays,
                                );
                                add_if_not_in_cache(
                                    Ray {
                                        direction: dir2,
                                        pos: (r, c),
                                    },
                                    &mut rays_cache,
                                    &mut new_rays,
                                );
                            }
                        }
                    } else {
                        add_if_not_in_cache(
                            Ray {
                                direction: ray.direction,
                                pos: (r, c),
                            },
                            &mut rays_cache,
                            &mut new_rays,
                        );
                    }
                    energized[(r as usize, c as usize)] = true;
                }
            }
            current_rays = new_rays;
        }
        energized.iter().filter(|c| **c).count()
    }
}
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Contraption {
    SplitterVertical,
    SplitterHorizontal,
    MirrorRight,
    MirrorLeft,
}

impl Contraption {
    fn get_from_char(ch: char) -> Option<Contraption> {
        match ch {
            '|' => Some(Contraption::SplitterVertical),
            '-' => Some(Contraption::SplitterHorizontal),
            '\\' => Some(Contraption::MirrorRight),
            '/' => Some(Contraption::MirrorLeft),
            _ => None,
        }
    }

    fn get_leaving_directions(&self, ray_direction: Direction) -> LeavingDirections {
        match (self, ray_direction) {
            (Contraption::SplitterVertical, Direction::Left)
            | (Contraption::SplitterVertical, Direction::Right) => LeavingDirections::Two {
                dir1: Direction::Up,
                dir2: Direction::Down,
            },
            (Contraption::SplitterVertical, dir) => LeavingDirections::One(dir),
            (Contraption::SplitterHorizontal, Direction::Up)
            | (Contraption::SplitterHorizontal, Direction::Down) => LeavingDirections::Two {
                dir1: Direction::Left,
                dir2: Direction::Right,
            },
            (Contraption::SplitterHorizontal, dir) => LeavingDirections::One(dir),
            (Contraption::MirrorRight, Direction::Up) => LeavingDirections::One(Direction::Left),
            (Contraption::MirrorRight, Direction::Down) => LeavingDirections::One(Direction::Right),
            (Contraption::MirrorRight, Direction::Left) => LeavingDirections::One(Direction::Up),
            (Contraption::MirrorRight, Direction::Right) => LeavingDirections::One(Direction::Down),
            (Contraption::MirrorLeft, Direction::Up) => LeavingDirections::One(Direction::Right),
            (Contraption::MirrorLeft, Direction::Down) => LeavingDirections::One(Direction::Left),
            (Contraption::MirrorLeft, Direction::Left) => LeavingDirections::One(Direction::Down),
            (Contraption::MirrorLeft, Direction::Right) => LeavingDirections::One(Direction::Up),
        }
    }
}

enum LeavingDirections {
    One(Direction),
    Two { dir1: Direction, dir2: Direction },
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct Ray {
    direction: Direction,
    pos: Coordinate,
}

impl Ray {
    fn get_next_cell(&self, nb_rows: usize, nb_cols: usize) -> Option<Coordinate> {
        let (r, c) = self.pos;
        let (new_row, new_col) = match self.direction {
            Direction::Up => (r - 1, c),
            Direction::Down => (r + 1, c),
            Direction::Left => (r, c - 1),
            Direction::Right => (r, c + 1),
        };
        if (0..nb_rows as isize).contains(&new_row) && (0..nb_cols as isize).contains(&new_col) {
            Some((new_row, new_col))
        } else {
            None
        }
    }
}

fn add_if_not_in_cache(ray: Ray, cache: &mut Array2<Vec<Direction>>, new_rays: &mut Vec<Ray>) {
    let (x, y) = ray.pos;
    let cached_dirs = cache.get_mut((x as usize, y as usize)).unwrap();
    if !cached_dirs.contains(&ray.direction) {
        cached_dirs.push(ray.direction);
        new_rays.push(ray);
    }
}

impl Day for Day16 {
    fn make_day(file: File) -> Self {
        let data: Vec<Vec<Option<Contraption>>> = std::io::BufReader::new(file)
            .lines()
            .map(|line| {
                line.expect("doc should have lines")
                    .chars()
                    .map(|ch| Contraption::get_from_char(ch))
                    .collect()
            })
            .collect();
        let rows = data.len();
        let cols = data[0].len();
        let flattened: Vec<_> = data.into_iter().flatten().collect();
        let contraptions = Array2::from_shape_vec((rows, cols), flattened).unwrap();
        Day16 { contraptions }
    }

    fn solution1(&self) -> String {
        self.compute_energized_cells(Ray {
            direction: Direction::Right,
            pos: (0, -1),
        })
        .to_string()
    }

    fn solution2(&self) -> String {
        let (nb_rows, nb_cols) = self.contraptions.dim();
        let (nb_rows, nb_cols) = (nb_rows as usize, nb_cols as usize);
        let energized_rows = (0..nb_rows)
            .into_par_iter()
            .map(|row| {
                max(
                    self.compute_energized_cells(Ray {
                        direction: Direction::Right,
                        pos: (row as isize, -1),
                    }),
                    self.compute_energized_cells(Ray {
                        direction: Direction::Left,
                        pos: (row as isize, nb_cols as isize),
                    }),
                )
            })
            .max()
            .unwrap();
        let energized_cols = (0..nb_cols)
            .into_par_iter()
            .map(|col| {
                max(
                    self.compute_energized_cells(Ray {
                        direction: Direction::Down,
                        pos: (-1, col as isize),
                    }),
                    self.compute_energized_cells(Ray {
                        direction: Direction::Up,
                        pos: (nb_rows as isize, col as isize),
                    }),
                )
            })
            .max()
            .unwrap();
        max(energized_cols, energized_rows).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day16_sol1() {
        let input = File::open("./inputs/day16/input_test.txt").expect("File not found");
        let day = Day16::make_day(input);
        assert_eq!(day.solution1(), "46");
    }

    #[test]
    fn test_day16_sol2() {
        let input = File::open("./inputs/day16/input_test.txt").expect("File not found");
        let day = Day16::make_day(input);
        assert_eq!(day.solution2(), "51");
    }
}
