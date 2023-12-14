use std::{collections::HashMap, str::FromStr};

fn main() {
    let input = include_str!("../input.txt");
    let output = part2(input);
    dbg!(output);
}
#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(u8)]
enum Space {
    Galaxy = b'#',
    Empty = b'.',
}
impl Space {
    // Space::from('|') == Ok(Space::Verticle)
    pub fn from(c: char) -> Result<Space, ()> {
        match c {
            '#' => Ok(Space::Galaxy),
            '.' => Ok(Space::Empty),
            _ => Err(()),
        }
    }
}
impl FromStr for Space {
    type Err = ();

    fn from_str(s: &str) -> Result<Space, ()> {
        match s {
            "#" => Ok(Space::Galaxy),
            "." => Ok(Space::Empty),
            _ => Err(()),
        }
    }
} // example: Spaces::Verticle.into() == '|'
impl Into<char> for Space {
    fn into(self) -> char {
        self as u8 as char
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
struct GalaxyLocation(usize, usize);

struct Expansion {
    x_cols: Vec<usize>,
    y_rows: Vec<usize>,
}

impl Expansion {
    fn new() -> Expansion {
        Expansion {
            x_cols: Vec::new(),
            y_rows: Vec::new(),
        }
    }
}

struct Universe {
    galaxies: Vec<GalaxyLocation>,
    galaxy_map: HashMap<GalaxyLocation, Space>,
    expansion: Expansion,
}

impl Universe {
    fn new() -> Universe {
        Universe {
            galaxies: Vec::new(),
            galaxy_map: HashMap::new(),
            expansion: Expansion::new(),
        }
    }
    fn get_pairs(&self) -> Vec<(GalaxyLocation, GalaxyLocation)> {
        self.galaxies
            .iter()
            .enumerate()
            .flat_map(|(i, a)| self.galaxies[i + 1..].iter().map(move |b| (*a, *b)))
            .collect()
    }
    fn get_total_distance(&self, expansion_multiplier: &u64) -> u64 {
        // do the distance addition when crossing expansion rows or columns
        self.get_pairs()
            .iter()
            .map(|(first, second)| {
                let (larger_x, smaller_x) = if second.0 > first.0 {
                    (second.0 as u64, first.0 as u64)
                } else {
                    (first.0 as u64, second.0 as u64)
                };
                let (larger_y, smaller_y) = if second.1 > first.1 {
                    (second.1 as u64, first.1 as u64)
                } else {
                    (first.1 as u64, second.1 as u64)
                };
                let mut total: u64 = 0;
                // add any expansion distance
                total += self
                    .expansion
                    .x_cols
                    .iter()
                    .map(|empty_row| {
                        if larger_x > *empty_row as u64 && smaller_x < *empty_row as u64 {
                            // Removes one for the current row
                            *expansion_multiplier - 1
                        } else {
                            0u64
                        }
                    })
                    .sum::<u64>();
                total += self
                    .expansion
                    .y_rows
                    .iter()
                    .map(|empty_row| {
                        if larger_y > *empty_row as u64 && smaller_y < *empty_row as u64 {
                            // Removes one for the current column
                            *expansion_multiplier - 1
                        } else {
                            0u64
                        }
                    })
                    .sum::<u64>();
                total += larger_x - smaller_x + larger_y - smaller_y;
                total
            })
            .sum()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseUniverseError;

impl FromStr for Universe {
    type Err = ParseUniverseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        // Parse our input into our Universe
        let mut galaxies: Vec<GalaxyLocation> = Vec::new();
        let mut galaxy_map: HashMap<GalaxyLocation, Space> = HashMap::new();
        let mut x_bound: usize = 0;
        let mut y_bound: usize = 0;
        input.lines().for_each(|line| {
            y_bound += 1;
            x_bound = 0;
            line.chars().for_each(|space| {
                x_bound += 1;
                match Space::from(space) {
                    Ok(space) => {
                        if space == Space::Galaxy {
                            galaxies.push(GalaxyLocation(x_bound, y_bound));
                        }
                        galaxy_map.insert(GalaxyLocation(x_bound, y_bound), space);
                    }
                    Err(_) => panic!("invalid input"),
                };
            });
        });
        let end_bound = GalaxyLocation(x_bound, y_bound);
        // Find the empty gallaxies and double the distance to create the expansion map
        let mut empty_y: Vec<usize> = Vec::new();
        let mut empty_x: Vec<usize> = Vec::new();
        dbg!(&end_bound.0);
        dbg!(&end_bound.1);
        for y in (1 as usize)..=end_bound.1 {
            let mut no_galaxy: bool = true;
            for x in (1 as usize)..=end_bound.0 {
                match galaxy_map.get(&GalaxyLocation(x, y)) {
                    Some(space) => match space {
                        Space::Galaxy => {
                            no_galaxy = false;
                            break;
                        }
                        Space::Empty => {}
                    },
                    None => {
                        panic!("Expected mapping for {:?}", GalaxyLocation(x, y))
                    }
                }
            }
            if no_galaxy {
                empty_y.push(y);
            }
        }
        for x in 1 as usize..=end_bound.0 {
            let mut no_galaxy: bool = true;
            for y in 1 as usize..=end_bound.1 {
                match galaxy_map.get(&GalaxyLocation(x, y)) {
                    Some(space) => match space {
                        Space::Galaxy => {
                            no_galaxy = false;
                            break;
                        }
                        Space::Empty => {}
                    },
                    None => {
                        panic!("Expected mapping for {:?}", GalaxyLocation(x, y))
                    }
                }
            }
            if no_galaxy {
                empty_x.push(x);
            }
        }
        dbg!(&empty_x);
        dbg!(&empty_y);

        Ok(Universe {
            galaxies,
            galaxy_map,
            expansion: Expansion {
                x_cols: empty_x,
                y_rows: empty_y,
            },
        })
    }
}

fn process(input: &str, multiplier: &u64) -> u64 {
    let universe: Universe = Universe::from_str(input).unwrap();
    // let pairs: Vec<(GalaxyLocation, GalaxyLocation)> = universe.get_pairs();
    // dbg!(&pairs.len());
    universe.get_total_distance(multiplier)
}

fn part2(input: &str) -> u64 {
    process(input, &1000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_works() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        let response = process(input, &2);
        assert_eq!(response, 374);
        let response = process(input, &10);
        assert_eq!(response, 1030);
        let response = process(input, &100);
        assert_eq!(response, 8410);
    }

    #[test]
    fn from_str_works() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        let universe: Universe = Universe::from_str(input).unwrap();
        assert_eq!(universe.galaxies.len(), 9);
    }
}
