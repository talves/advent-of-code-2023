use std::{collections::HashMap, str::FromStr};

fn main() {
    let input = include_str!("../input.txt");
    let output = part1(input);
    dbg!(output);
}

// The pipes are arranged in a two-dimensional grid of tiles:

// | is a vertical pipe connecting north and south.
// - is a horizontal pipe connecting east and west.
// L is a 90-degree bend connecting north and east.
// J is a 90-degree bend connecting north and west.
// 7 is a 90-degree bend connecting south and west.
// F is a 90-degree bend connecting south and east.
// . is ground; there is no pipe in this tile.
// S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.

#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(u8)]
enum Pipe {
    Vertical = b'|',
    Horizontal = b'-',
    BendNE = b'L',
    BendNW = b'J',
    BendSW = b'7',
    BendSE = b'F',
    Ground = b'.',
    Starting = b'S',
}
impl Pipe {
    // Pipe::from('|') == Ok(Pipe::Verticle)
    pub fn from(c: char) -> Result<Pipe, ()> {
        match c {
            '|' => Ok(Pipe::Vertical),
            '-' => Ok(Pipe::Horizontal),
            'L' => Ok(Pipe::BendNE),
            'J' => Ok(Pipe::BendNW),
            '7' => Ok(Pipe::BendSW),
            'F' => Ok(Pipe::BendSE),
            '.' => Ok(Pipe::Ground),
            'S' => Ok(Pipe::Starting),
            _ => Err(()),
        }
    }
}
impl FromStr for Pipe {
    type Err = ();

    fn from_str(s: &str) -> Result<Pipe, ()> {
        match s {
            "|" => Ok(Pipe::Vertical),
            "-" => Ok(Pipe::Horizontal),
            "L" => Ok(Pipe::BendNE),
            "J" => Ok(Pipe::BendNW),
            "7" => Ok(Pipe::BendSW),
            "F" => Ok(Pipe::BendSE),
            "." => Ok(Pipe::Ground),
            "S" => Ok(Pipe::Starting),
            _ => Err(()),
        }
    }
} // example: Pipes::Verticle.into() == '|'
impl Into<char> for Pipe {
    fn into(self) -> char {
        self as u8 as char
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
struct PipeLocation(usize, usize);

#[derive(Debug)]
struct Location {
    point: PipeLocation,
    north: Option<PipeLocation>,
    south: Option<PipeLocation>,
    east: Option<PipeLocation>,
    west: Option<PipeLocation>,
}

impl Location {
    fn new() -> Location {
        Location {
            point: PipeLocation(0, 0),
            north: None,
            south: None,
            east: None,
            west: None,
        }
    }
    fn set(&mut self, point: &PipeLocation, bounds: &PipeLocation) {
        self.point = PipeLocation(point.0, point.1);
        self.north = if point.1 > 1 {
            Some(PipeLocation(point.0, point.1 - 1))
        } else {
            None
        };
        self.south = if point.1 < bounds.1 {
            Some(PipeLocation(point.0, point.1 + 1))
        } else {
            None
        };
        self.east = if point.0 < bounds.0 {
            Some(PipeLocation(point.0 + 1, point.1))
        } else {
            None
        };
        self.west = if point.0 > 1 {
            Some(PipeLocation(point.0 - 1, point.1))
        } else {
            None
        };
    }
    fn get_start_paths(&self, map: &HashMap<PipeLocation, Pipe>) -> Vec<PipeLocation> {
        let mut paths: Vec<PipeLocation> = Vec::new();
        match &self.north {
            Some(loc) => match map.get(loc) {
                Some(pipe) => match pipe {
                    Pipe::Vertical | Pipe::BendSE | Pipe::BendSW => {
                        paths.push(*loc);
                    }
                    Pipe::Ground | Pipe::BendNE | Pipe::BendNW | Pipe::Horizontal => {}
                    Pipe::Starting => {
                        panic!("More than one starting point")
                    }
                },
                None => {}
            },
            None => {}
        }
        match &self.south {
            Some(loc) => match map.get(loc) {
                Some(pipe) => match pipe {
                    Pipe::Vertical | Pipe::BendNE | Pipe::BendNW => {
                        paths.push(*loc);
                    }
                    Pipe::Ground | Pipe::BendSE | Pipe::BendSW | Pipe::Horizontal => {}
                    Pipe::Starting => {
                        panic!("More than one starting point")
                    }
                },
                None => {}
            },
            None => {}
        }
        match &self.west {
            Some(loc) => match map.get(loc) {
                Some(pipe) => match pipe {
                    Pipe::Horizontal | Pipe::BendNE | Pipe::BendSE => {
                        paths.push(*loc);
                    }
                    Pipe::Ground | Pipe::BendSW | Pipe::BendNW | Pipe::Vertical => {}
                    Pipe::Starting => {
                        panic!("More than one starting point")
                    }
                },
                None => {}
            },
            None => {}
        }
        match &self.east {
            Some(loc) => match map.get(loc) {
                Some(pipe) => match pipe {
                    Pipe::Horizontal | Pipe::BendNW | Pipe::BendSW => {
                        paths.push(*loc);
                    }
                    Pipe::Ground | Pipe::BendSE | Pipe::BendNE | Pipe::Vertical => {}
                    Pipe::Starting => {
                        panic!("More than one starting point")
                    }
                },
                None => {}
            },
            None => {}
        }
        paths
    }
    fn get_path(
        &self,
        last_location: &PipeLocation,
        map: &HashMap<PipeLocation, Pipe>,
    ) -> PipeLocation {
        let mut new_location: PipeLocation = PipeLocation(0, 0);
        if let Some(this_pipe) = map.get(&self.point) {
            if let Some(loc) = &self.north {
                if loc != last_location {
                    match map.get(loc) {
                        Some(pipe) => match pipe {
                            Pipe::Starting | Pipe::Vertical | Pipe::BendSE | Pipe::BendSW => {
                                if [Pipe::Vertical, Pipe::BendNE, Pipe::BendNW].contains(this_pipe)
                                {
                                    new_location = *loc;
                                }
                            }
                            Pipe::Ground | Pipe::BendNE | Pipe::BendNW | Pipe::Horizontal => {}
                        },
                        None => {}
                    }
                }
            };
            if let Some(loc) = &self.south {
                if loc != last_location {
                    match map.get(loc) {
                        Some(pipe) => match pipe {
                            Pipe::Starting | Pipe::Vertical | Pipe::BendNE | Pipe::BendNW => {
                                if [Pipe::Vertical, Pipe::BendSE, Pipe::BendSW].contains(this_pipe)
                                {
                                    new_location = *loc;
                                }
                            }
                            Pipe::Ground | Pipe::BendSE | Pipe::BendSW | Pipe::Horizontal => {}
                        },
                        None => {}
                    }
                }
            };
            if let Some(loc) = &self.west {
                if loc != last_location {
                    match map.get(loc) {
                        Some(pipe) => match pipe {
                            Pipe::Starting | Pipe::Horizontal | Pipe::BendNE | Pipe::BendSE => {
                                if [Pipe::Horizontal, Pipe::BendNW, Pipe::BendSW]
                                    .contains(this_pipe)
                                {
                                    new_location = *loc;
                                }
                            }
                            Pipe::Ground | Pipe::BendSW | Pipe::BendNW | Pipe::Vertical => {}
                        },
                        None => {}
                    }
                }
            };
            if let Some(loc) = &self.east {
                if loc != last_location {
                    match map.get(loc) {
                        Some(pipe) => match pipe {
                            Pipe::Starting | Pipe::Horizontal | Pipe::BendNW | Pipe::BendSW => {
                                if [Pipe::Horizontal, Pipe::BendNE, Pipe::BendSE]
                                    .contains(this_pipe)
                                {
                                    new_location = *loc;
                                }
                            }
                            Pipe::Ground | Pipe::BendSE | Pipe::BendNE | Pipe::Vertical => {}
                        },
                        None => {}
                    }
                }
            };
        };

        if new_location == PipeLocation(0, 0) {
            dbg!(format!("problem {:?}", &new_location));
            dbg!(format!("        {:?}", &self));
            dbg!(format!("        {:?}", last_location));
            panic!("Every point should be mapped");
        }

        new_location
    }
}

fn parse_input(input: &str) -> (PipeLocation, PipeLocation, HashMap<PipeLocation, Pipe>) {
    let mut start_location: PipeLocation = PipeLocation(0, 0);
    let mut path_loop: HashMap<PipeLocation, Pipe> = HashMap::default();
    path_loop.insert(PipeLocation(0, 0), Pipe::Ground);
    let mut x_bound: usize = 0;
    let mut y_bound: usize = 0;
    input.lines().for_each(|line| {
        y_bound += 1;
        x_bound = 0;
        line.chars().for_each(|pipe| {
            x_bound += 1;
            match Pipe::from(pipe) {
                Ok(value) => {
                    if value == Pipe::Starting {
                        start_location = PipeLocation(x_bound, y_bound);
                    }
                    path_loop.insert(PipeLocation(x_bound, y_bound), value);
                }
                Err(_) => panic!("invalid input line"),
            };
        });
    });
    (start_location, PipeLocation(x_bound, y_bound), path_loop)
}

fn set_point(point: &PipeLocation, bound: &PipeLocation) -> Location {
    let mut location = Location::new();
    location.set(point, bound);
    location
}

fn process(input: &str) -> u32 {
    let (start_location, bound_location, loop_map) = parse_input(input);
    let start = set_point(&start_location, &bound_location);
    let paths = start.get_start_paths(&loop_map); // These path locations are the first step each way

    let mut path_one = set_point(&paths[0], &bound_location);
    let mut path_two = set_point(&paths[1], &bound_location);
    let mut count: u32 = 1;

    let mut last_location_one = start_location;
    let mut last_location_two = start_location;

    let mut still_searching = true;

    while still_searching {
        let next_one = path_one.get_path(&last_location_one, &loop_map);
        let next_two = path_two.get_path(&last_location_two, &loop_map);
        last_location_one = path_one.point;
        last_location_two = path_two.point;
        path_one = set_point(&next_one, &bound_location);
        path_two = set_point(&next_two, &bound_location);
        still_searching = path_one.point != path_two.point;
        count += 1;
    }

    dbg!(format!("{:?}", path_one.point));
    dbg!(format!("{:?}", path_two.point));
    // dbg!(&count);
    // dbg!(&paths);
    // dbg!(&bound_location);
    // dbg!(&loop_map);
    count
}

fn part1(input: &str) -> u32 {
    process(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let result = part1(
            ".....
.S-7.
.|.|.
.L-J.
.....",
        );
        assert_eq!(result, 4);
        let result = part1(
            "..F7.
.FJ|.
SJ.L7
|F--J
LJ...",
        );
        assert_eq!(result, 8);
        let result = part1(
            "-L|F7
7S-7|
L|7||
-L-J|
L|-JF",
        );
        assert_eq!(result, 4);
        let result = part1(
            "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ",
        );
        assert_eq!(result, 8);
    }

    #[test]
    fn check_pipe() {
        let result = Pipe::from('|');
        assert_eq!(result, Ok(Pipe::Vertical));
        let result = Pipe::from_str("|");
        assert_eq!(result, Ok(Pipe::Vertical));
    }
}
