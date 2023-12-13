use std::{collections::HashMap, str::FromStr};

fn main() {
    let input = include_str!("../input.txt");
    let output = part2(input);
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

struct Tracker<'a> {
    input_map: &'a HashMap<PipeLocation, Pipe>,
    tunnel: HashMap<PipeLocation, Pipe>,
    right: HashMap<PipeLocation, Option<Pipe>>,
    left: HashMap<PipeLocation, Option<Pipe>>,
}

impl Tracker<'_> {
    fn new(input_map: &HashMap<PipeLocation, Pipe>) -> Tracker {
        Tracker {
            input_map,
            tunnel: HashMap::new(),
            right: HashMap::new(),
            left: HashMap::new(),
        }
    }
    fn add_right_left(&mut self, pipe_location: &PipeLocation, direction: &str) {
        if let Some(_loc) = self.tunnel.get(pipe_location) {
            // This position is part of the tunnel, so ignore it
            self.left.remove(pipe_location);
            self.right.remove(pipe_location);
        } else {
            if direction == "right" {
                self.right.insert(*pipe_location, None);
                self.left.remove(pipe_location);
            } else {
                self.left.insert(*pipe_location, None);
                self.right.remove(pipe_location);
            }
        }
    }
    fn assign(&mut self, location: &Location, last: &PipeLocation, forward: Option<bool>) {
        let is_forward = forward.unwrap_or(true);
        let current = self.input_map.get(&location.point).unwrap();
        if *current == Pipe::Starting {
            self.tunnel.insert(location.point, *current);
            return;
        }
        // check to see if this location was added earlier to right or left
        // if let Some(_loc) = self.right.get(&location.point) {}
        // if let Some(_loc) = self.left.get(&location.point) {}
        // insert the current as a tunnel location
        self.tunnel.insert(location.point, *current);
        // Remove it from right
        self.right.remove(&location.point);
        // Remove it from left
        self.left.remove(&location.point);

        let east = if let Some(east) = &location.east {
            *east
        } else {
            PipeLocation(0, 0)
        };
        let west = if let Some(west) = &location.west {
            *west
        } else {
            PipeLocation(0, 0)
        };
        let north = if let Some(north) = &location.north {
            *north
        } else {
            PipeLocation(0, 0)
        };
        let south = if let Some(south) = &location.south {
            *south
        } else {
            PipeLocation(0, 0)
        };
        match current {
            Pipe::Vertical => {
                // We only care about east and west
                if *last == north {
                    // Moving south
                    if is_forward {
                        self.add_right_left(&east, "left");
                        self.add_right_left(&west, "right");
                    } else {
                        self.add_right_left(&east, "right");
                        self.add_right_left(&west, "left");
                    }
                } else {
                    // Moving north
                    if is_forward {
                        self.add_right_left(&east, "right");
                        self.add_right_left(&west, "left");
                    } else {
                        self.add_right_left(&east, "left");
                        self.add_right_left(&west, "right");
                    }
                }
            }
            Pipe::Horizontal => {
                // We only care about north and south
                if *last == east {
                    // Moving west
                    if is_forward {
                        self.add_right_left(&south, "left");
                        self.add_right_left(&north, "right");
                    } else {
                        self.add_right_left(&south, "right");
                        self.add_right_left(&north, "left");
                    }
                } else {
                    // Moving east
                    if is_forward {
                        self.add_right_left(&south, "right");
                        self.add_right_left(&north, "left");
                    } else {
                        self.add_right_left(&south, "left");
                        self.add_right_left(&north, "right");
                    }
                }
            }
            Pipe::BendNE => {
                // We only care about west and south
                if *last == north {
                    // Moving east
                    if is_forward {
                        self.add_right_left(&south, "right");
                        self.add_right_left(&west, "right");
                    } else {
                        self.add_right_left(&south, "left");
                        self.add_right_left(&west, "left");
                    }
                } else {
                    // Moving north
                    if is_forward {
                        self.add_right_left(&south, "left");
                        self.add_right_left(&west, "left");
                    } else {
                        self.add_right_left(&south, "right");
                        self.add_right_left(&west, "right");
                    }
                }
            }
            Pipe::BendNW => {
                // We only care about east and south
                if *last == north {
                    // Moving west
                    if is_forward {
                        self.add_right_left(&south, "left");
                        self.add_right_left(&east, "left");
                    } else {
                        self.add_right_left(&south, "right");
                        self.add_right_left(&east, "right");
                    }
                } else {
                    // Moving east
                    if is_forward {
                        self.add_right_left(&south, "right");
                        self.add_right_left(&east, "right");
                    } else {
                        self.add_right_left(&south, "left");
                        self.add_right_left(&east, "left");
                    }
                }
            }
            Pipe::BendSW => {
                // We only care about east and north
                if *last == south {
                    // Moving west
                    if is_forward {
                        self.add_right_left(&north, "right");
                        self.add_right_left(&east, "right");
                    } else {
                        self.add_right_left(&north, "left");
                        self.add_right_left(&east, "left");
                    }
                } else {
                    // Moving south
                    if is_forward {
                        self.add_right_left(&north, "left");
                        self.add_right_left(&east, "left");
                    } else {
                        self.add_right_left(&north, "right");
                        self.add_right_left(&east, "right");
                    }
                }
            }
            Pipe::BendSE => {
                // We only care about west and north
                if *last == south {
                    // Moving east
                    if is_forward {
                        self.add_right_left(&north, "left");
                        self.add_right_left(&west, "left");
                    } else {
                        self.add_right_left(&north, "right");
                        self.add_right_left(&west, "right");
                    }
                } else {
                    // Moving south
                    if is_forward {
                        self.add_right_left(&north, "right");
                        self.add_right_left(&west, "right");
                    } else {
                        self.add_right_left(&north, "left");
                        self.add_right_left(&west, "left");
                    }
                }
            }
            Pipe::Ground => {
                panic!("There should be no ground in our paths");
            }
            Pipe::Starting => {
                // We don't have a previous here, so we do a test for all directions
            }
        }
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

fn process_part2(input: &str) -> u32 {
    let (start_location, bound_location, loop_map) = parse_input(input);
    let start = set_point(&start_location, &bound_location);
    let paths = start.get_start_paths(&loop_map); // These path locations are the first step each way

    //determine forward path
    let mut path_one = set_point(&paths[0], &bound_location);

    let mut count: u32 = 1;

    let mut last_location = start_location;

    let mut still_searching = true;

    let mut tracker: Tracker = Tracker::new(&loop_map);
    tracker.assign(&start, &start_location, Some(true));

    while still_searching {
        let next_one = path_one.get_path(&last_location, &loop_map);
        tracker.assign(&path_one, &last_location, Some(true));
        last_location = path_one.point;
        path_one = set_point(&next_one, &bound_location);
        still_searching = path_one.point != start_location;
        count += 1;
    }

    dbg!(format!("{:?}", path_one.point));
    dbg!(&count);
    // TODO: Need to determine what the last location was and match to last to find out if right or left are inside loop
    // TODO: Fill in the missing surrounded ground. Should just add whole grid with ground, then make sure there are none next to right and left :)
    dbg!(&tracker.right.len());
    dbg!(&tracker.left.len());
    // dbg!(&tracker.left);
    // dbg!(&paths);
    // dbg!(&bound_location);
    // dbg!(&loop_map);
    tracker.left.len() as u32
}

fn part1(input: &str) -> u32 {
    process(input)
}

fn part2(input: &str) -> u32 {
    process_part2(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_works() {
        let result = part2(
            "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........",
        );
        assert_eq!(result, 4);
        let result = part2(
            "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........",
        );
        assert_eq!(result, 4);
        assert_eq!(result, 4);
        let result = part2(
            "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L------J.
..........",
        );
        assert_eq!(result, 99);

        let result = part2(
            ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...",
        );
        assert_eq!(result, 8);
    }

    #[test]
    fn part1_works() {
        let result = part1(
            "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........",
        );
        assert_eq!(result, 23);
    }

    #[test]
    fn check_pipe() {
        let result = Pipe::from('|');
        assert_eq!(result, Ok(Pipe::Vertical));
        let result = Pipe::from_str("|");
        assert_eq!(result, Ok(Pipe::Vertical));
    }
}
