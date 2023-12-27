use std::str::FromStr;

fn main() {
    let input = include_str!("../input.txt");
    let output = part1(input);
    dbg!(output);
}

fn process(input: &str) -> u64 {
    let mut platform: Platform = Platform::from_str(input).unwrap();
    // Debugging output for visualization
    platform.rows.iter().for_each(|row| {
        row.rocks.iter().for_each(|x| match x.shape {
            RockShape::Rounded => print!("O"),
            RockShape::Cubed => print!("#"),
            RockShape::Empty => print!("."),
        });
        println!("");
    });
    dbg!(platform.load());
    platform.tilt();
    dbg!(platform.load());
    platform.load()
}

fn part1(input: &str) -> u64 {
    process(input)
}

#[derive(Debug, PartialEq, Clone)]

enum RockShape {
    Rounded,
    Cubed,
    Empty,
}

#[derive(Debug, PartialEq, Clone)]

struct Rock {
    shape: RockShape,
}

#[derive(Debug, PartialEq, Clone)]

struct RockRow {
    load_value: usize,
    rocks: Vec<Rock>,
}

impl RockRow {
    fn new() -> RockRow {
        RockRow {
            load_value: 0,
            rocks: Vec::new(),
        }
    }
    fn get_load(&self) -> usize {
        self.rocks
            .iter()
            .map(|rock| match rock.shape {
                RockShape::Rounded => self.load_value,
                RockShape::Cubed => 0,
                _ => 0,
            })
            .sum::<usize>()
    }
}

#[derive(Debug, PartialEq, Clone)]

struct Platform {
    rows: Vec<RockRow>,
}

impl Platform {
    fn new() -> Platform {
        Platform { rows: Vec::new() }
    }
    fn load(&self) -> u64 {
        self.rows.iter().map(|row| row.get_load()).sum::<usize>() as u64
    }
    fn tilt(&mut self) -> &mut Self {
        let mut new_platform = self.clone();

        for current_row_idx in 0..new_platform.rows.len() - 1 {
            let current_row = new_platform.rows[current_row_idx].clone();
            current_row
                .rocks
                .iter()
                .enumerate()
                .for_each(|(rock_idx, rock)| {
                    match rock.shape {
                        RockShape::Empty => {
                            // Find next not empty rock
                            let mut next_idx = current_row_idx + 1;
                            while new_platform.rows[next_idx].rocks[rock_idx].shape
                                == RockShape::Empty
                                && next_idx + 1 < new_platform.rows.len()
                            {
                                next_idx += 1;
                            }
                            // check if this next rock is round, and replace if true
                            if new_platform.rows[next_idx].rocks[rock_idx].shape
                                == RockShape::Rounded
                            {
                                new_platform.rows[current_row_idx].rocks[rock_idx].shape =
                                    RockShape::Rounded;
                                new_platform.rows[next_idx].rocks[rock_idx].shape =
                                    RockShape::Empty;
                            }
                        }
                        RockShape::Rounded | RockShape::Cubed => {}
                    };
                });
        }

        self.rows = new_platform.rows;
        self
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParsePlatformError;

impl FromStr for Platform {
    type Err = ParsePlatformError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        // Parse our input into our Platform
        let mut rock_rows: Vec<RockRow> = Vec::new();
        let mut work_row: RockRow = RockRow::new();
        let length = input.lines().count();
        input.lines().enumerate().for_each(|(idx, line)| {
            work_row.load_value = length - idx;
            line.chars().for_each(|ch| match ch {
                'O' => work_row.rocks.push(Rock {
                    shape: RockShape::Rounded,
                }),
                '#' => work_row.rocks.push(Rock {
                    shape: RockShape::Cubed,
                }),
                '.' => work_row.rocks.push(Rock {
                    shape: RockShape::Empty,
                }),
                _ => panic!("Invalid input at row:{} with char:'{}'", idx, ch),
            });
            rock_rows.push(work_row.to_owned());
            work_row = RockRow::new();
        });

        Ok(Platform { rows: rock_rows })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let result = part1(
            "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
        );
        assert_eq!(result, 136);
    }
}
