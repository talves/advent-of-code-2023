use std::{collections::HashMap, str::FromStr};

fn main() {
    let input = include_str!("../input.txt");
    let output = part1(input);
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

struct Universe {
    galaxies: Vec<GalaxyLocation>,
    galaxy_map: HashMap<GalaxyLocation, Space>,
    expansion_map: HashMap<GalaxyLocation, Space>,
}

impl Universe {
    fn new() -> Universe {
        Universe {
            galaxies: Vec::new(),
            galaxy_map: HashMap::new(),
            expansion_map: HashMap::new(),
        }
    }
    fn get_pairs(&self) -> Vec<(GalaxyLocation, GalaxyLocation)> {
        self.galaxies
            .iter()
            .enumerate()
            .flat_map(|(i, a)| self.galaxies[i + 1..].iter().map(move |b| (*a, *b)))
            .collect()
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
                        galaxy_map.insert(GalaxyLocation(x_bound, y_bound), space);
                    }
                    Err(_) => panic!("invalid input"),
                };
            });
        });
        let end_bound = GalaxyLocation(x_bound, y_bound);
        // Find the empty gallaxies and double the distance to create the expansion map
        let mut expansion_map: HashMap<GalaxyLocation, Space> = HashMap::new();
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
        let mut col_x: usize = 0;
        let mut row_y: usize = 0;
        for x in 1 as usize..=end_bound.0 {
            col_x += 1;
            for y in 1 as usize..=end_bound.1 {
                row_y += 1;
                let space = *galaxy_map.get(&GalaxyLocation(x, y)).unwrap();
                if space == Space::Galaxy {
                    galaxies.push(GalaxyLocation(col_x, row_y));
                }
                expansion_map.insert(GalaxyLocation(col_x, row_y), space);
                if empty_y.contains(&y) {
                    row_y += 1;
                    expansion_map.insert(GalaxyLocation(col_x, row_y), Space::Empty);
                }
            }
            row_y = 0;
            if empty_x.contains(&x) {
                col_x += 1;
                for new_y in 1 as usize..=end_bound.1 {
                    expansion_map.insert(GalaxyLocation(col_x, new_y), Space::Empty);
                }
            }
        }

        Ok(Universe {
            galaxies,
            galaxy_map,
            expansion_map,
        })
    }
}

fn process(input: &str) -> i32 {
    let universe: Universe = Universe::from_str(input).unwrap();
    let pairs: Vec<(GalaxyLocation, GalaxyLocation)> = universe.get_pairs();
    dbg!(&pairs.len());
    pairs
        .iter()
        .map(|(first, second)| {
            let x_dist = if second.0 > first.0 {
                (second.0 - first.0) as i32
            } else {
                (first.0 - second.0) as i32
            };
            let y_dist = if second.1 > first.1 {
                (second.1 - first.1) as i32
            } else {
                (first.1 - second.1) as i32
            };
            x_dist + y_dist
        })
        .sum()
}

fn part1(input: &str) -> i32 {
    process(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
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
        let response = part1(input);
        assert_eq!(response, 374);
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
