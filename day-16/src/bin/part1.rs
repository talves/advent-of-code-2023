use std::{collections::HashMap, fmt::Display};

fn main() {
    let input = include_str!("../input.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]

enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]

enum Mirror {
    Horizontal,
    Vertical,
    TiltRight,
    TiltLeft,
    Empty,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]

enum Light {
    Exists,
    Energized,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Location {
    x: u16,
    y: u16,
}

impl Location {
    fn new() -> Location {
        Location { x: 0, y: 0 }
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn print_layout(layout: &HashMap<Location, Mirror>, bounds: &Location) {
    for y in 1..=bounds.y {
        for x in 1..=bounds.x {
            let location = Location { x, y };
            print!(
                "{}",
                match layout.get(&location) {
                    Some(mirror) => match mirror {
                        Mirror::Empty => ".",
                        Mirror::Horizontal => "-",
                        Mirror::Vertical => "|",
                        Mirror::TiltLeft => "\\",
                        Mirror::TiltRight => "/",
                    },
                    None => {
                        panic!("Bad mirror stored: {}", location);
                    }
                }
            );
        }
        println!("");
    }
}
fn print_energized(energized: &HashMap<Location, Light>, bounds: &Location) {
    for y in 1..=bounds.y {
        for x in 1..=bounds.x {
            let location = Location { x, y };
            print!(
                "{}",
                match energized.get(&location) {
                    Some(light) => match light {
                        Light::Exists => "+",
                        Light::Energized => "#",
                    },
                    None => {
                        "."
                    }
                }
            );
        }
        println!("");
    }
}

fn parse_input(input: &str) -> (Location, HashMap<Location, Mirror>) {
    let bounds = Location {
        x: input.lines().next().unwrap().len() as u16,
        y: input.lines().count() as u16,
    };
    let mut layout: HashMap<Location, Mirror> = HashMap::new();

    let mut x: u16 = 0;
    let mut y: u16 = 0;
    input.lines().for_each(|line| {
        y += 1; // start a new row
        x = 0; // reset col count
        line.chars().for_each(|c| {
            x += 1;
            let location = Location { x, y };
            match c {
                '.' => layout.insert(location, Mirror::Empty),
                '|' => layout.insert(location, Mirror::Vertical),
                '-' => layout.insert(location, Mirror::Horizontal),
                '/' => layout.insert(location, Mirror::TiltRight),
                '\\' => layout.insert(location, Mirror::TiltLeft),
                _ => panic!(
                    "{}",
                    format!("Invalid char: {} ({},{})", c, location.x, location.y)
                ),
            };
        });
    });
    (bounds, layout)
}

fn process(input: &str) -> u64 {
    let bounds: Location;
    let layout: HashMap<Location, Mirror>;
    let mut energized: HashMap<Location, Light> = HashMap::new();
    let mut completed: HashMap<(Location, Direction), Light> = HashMap::new();

    (bounds, layout) = parse_input(input);

    print_layout(&layout, &bounds);

    // Start Here
    let mut start_light_beam = Some((Location { x: 1, y: 1 }, Direction::Right));

    // Create a vec to store the split light source (unprocessed)
    let mut unprocessed: Vec<(Location, Direction)> = vec![start_light_beam.unwrap()];

    while unprocessed.len() > 0 {
        dbg!(&unprocessed.len());
        let current_location: Location = unprocessed[0].0;
        let current_direction: Direction = unprocessed[0].1;
        match start_light_beam {
            Some(_light_beam) => {
                // Just continue we are on a path
            }
            None => {
                // Set to the new unprocessed split value
                start_light_beam = Some((current_location.clone(), current_direction.clone()))
            }
        }
        unprocessed.remove(0);
        let mirror = match layout.get(&current_location) {
            Some(mirror) => {
                // We mark light in this energized map and mark it Energized if it already has a light beam
                energized
                    .entry(current_location)
                    .and_modify(|x| *x = Light::Energized)
                    .or_insert(Light::Exists);
                mirror
            }
            None => {
                panic!("Bad mirror stored: {}", current_location)
            }
        };
        println!(
            "location: {} direction: {:?} mirror: {:?} length: {}",
            &current_location,
            &current_direction,
            &mirror,
            unprocessed.len()
        );
        let result = process_light_beam(&mirror, &current_location, &current_direction, &bounds);
        match result {
            (None, None) => {
                // store the start light beam location as completed path, so we don't take it again (cache)
                match start_light_beam {
                    Some((start_location, start_direction)) => {
                        // Add the start light beam to the completed with the light value
                        // We should only traverse a path once, but this will help us debug
                        completed
                            .entry((start_location, start_direction))
                            .and_modify(|x| *x = Light::Energized)
                            .or_insert(Light::Exists);
                    }
                    None => {
                        if unprocessed.len() > 0 {
                            // This should not happen, so let's panic a warning
                            panic!("Warning: This condition should not happen. Start light Beam should exist until we are finished")
                        }
                    }
                };
                // We reset start light beam option to none to force a new start of the beam path.
                start_light_beam = None;
            }
            (None, Some(light_beam)) | (Some(light_beam), None) => {
                // Still on a single path, so we check to see if this path has been completed from here
                // If the path has been completed already from here we mark the start completed and move on
                match completed.get(&(light_beam.0, light_beam.1)) {
                    Some(light) => {
                        if *light == Light::Energized {
                            // This path has been completed, so we move on and store our start as completed!
                            match start_light_beam {
                                Some((start_location, start_direction)) => {
                                    // Add the start light beam to the completed with the light value
                                    // We should only traverse a path once, but this will help us debug
                                    completed
                                        .entry((start_location, start_direction))
                                        .and_modify(|x| *x = Light::Energized)
                                        .or_insert(Light::Exists);
                                    // We reset start light beam option to none to force a new start of the beam path.
                                    start_light_beam = None;
                                }
                                None => {}
                            };
                        } else {
                            // We let it go through twice for energizing
                            unprocessed.insert(0, light_beam)
                        }
                    }
                    None => {
                        // no path from here completed yet, so we put our next light back in for processing
                        unprocessed.insert(0, light_beam)
                    }
                }
            }
            (Some(light_beam), Some(split_light_beam)) => {
                // This is a split light beam, so we will store this path as completed and enter the split as new entries
                // store the start light beam location as completed path, so we don't take it again (cache)
                match start_light_beam {
                    Some((start_location, start_direction)) => {
                        // Add the start light beam to the completed with the light value
                        // We should only traverse a path once, but this will help us debug
                        completed
                            .entry((start_location, start_direction))
                            .and_modify(|x| *x = Light::Energized)
                            .or_insert(Light::Exists);
                    }
                    None => {
                        if unprocessed.len() > 0 {
                            // This should not happen, so let's panic a warning
                            panic!("Warning: This condition should not happen. Start light Beam should exist until we are finished")
                        }
                    }
                };
                // We reset start light beam option to none to force a new start of the beam path.
                start_light_beam = None;
                // If the path has been completed already from here in either direction we handle wether we need to continue
                let first_split_light = match completed.get(&(light_beam.0, light_beam.1)) {
                    Some(light) => *light,
                    None => Light::Exists,
                };
                let second_split_light =
                    match completed.get(&(split_light_beam.0, split_light_beam.1)) {
                        Some(light) => *light,
                        None => Light::Exists,
                    };
                if first_split_light == Light::Energized {
                    // first spit is completed
                    if second_split_light == Light::Energized {
                        // Nothing to do, both splits are completed
                    } else {
                        // process the next split
                        unprocessed.insert(0, split_light_beam);
                    }
                } else if second_split_light == Light::Energized {
                    // The second split is completed, so only process the first split
                    unprocessed.insert(0, light_beam);
                } else {
                    // both paths need to be processed as a start light beam
                    unprocessed.insert(0, light_beam);
                    unprocessed.insert(1, split_light_beam);
                }
            }
        }
    }

    println!("---------");
    print_energized(&energized, &bounds);

    energized.len() as u64
}

fn process_light_beam(
    mirror: &Mirror,
    location: &Location,
    direction: &Direction,
    bounds: &Location,
) -> (Option<(Location, Direction)>, Option<(Location, Direction)>) {
    match mirror {
        Mirror::Empty => {
            let next_location: Option<Location> = match direction {
                Direction::Left => {
                    if location.x - 1 > 0 {
                        Some(Location {
                            x: location.x - 1,
                            y: location.y,
                        })
                    } else {
                        None
                    }
                }
                Direction::Right => {
                    if location.x + 1 <= bounds.x {
                        Some(Location {
                            x: location.x + 1,
                            y: location.y,
                        })
                    } else {
                        None
                    }
                }
                Direction::Up => {
                    if location.y - 1 > 0 {
                        Some(Location {
                            x: location.x,
                            y: location.y - 1,
                        })
                    } else {
                        None
                    }
                }
                Direction::Down => {
                    if location.y + 1 <= bounds.y {
                        Some(Location {
                            x: location.x,
                            y: location.y + 1,
                        })
                    } else {
                        None
                    }
                }
            };
            match next_location {
                Some(new_location) => (Some((new_location, *direction)), None),
                None => {
                    // We are complete with processing this path
                    (None, None)
                }
            }
        }
        Mirror::Horizontal => {
            match direction {
                Direction::Left => {
                    if location.x - 1 > 0 {
                        (
                            Some((
                                Location {
                                    x: location.x - 1,
                                    y: location.y,
                                },
                                Direction::Left,
                            )),
                            None,
                        )
                    } else {
                        (None, None)
                    }
                }
                Direction::Right => {
                    if location.x + 1 <= bounds.x {
                        (
                            Some((
                                Location {
                                    x: location.x + 1,
                                    y: location.y,
                                },
                                Direction::Right,
                            )),
                            None,
                        )
                    } else {
                        (None, None)
                    }
                }
                Direction::Down | Direction::Up => {
                    // Head Left
                    let left = if location.x - 1 > 0 {
                        Some((
                            Location {
                                x: location.x - 1,
                                y: location.y,
                            },
                            Direction::Left,
                        ))
                    } else {
                        None
                    };
                    // Head Right
                    let right = if location.x + 1 <= bounds.x {
                        Some((
                            Location {
                                x: location.x + 1,
                                y: location.y,
                            },
                            Direction::Right,
                        ))
                    } else {
                        None
                    };
                    (left, right)
                }
            }
        }
        Mirror::Vertical => {
            match direction {
                Direction::Up => {
                    if location.y - 1 > 0 {
                        (
                            Some((
                                Location {
                                    x: location.x,
                                    y: location.y - 1,
                                },
                                Direction::Up,
                            )),
                            None,
                        )
                    } else {
                        (None, None)
                    }
                }
                Direction::Down => {
                    if location.y + 1 <= bounds.y {
                        (
                            Some((
                                Location {
                                    x: location.x,
                                    y: location.y + 1,
                                },
                                Direction::Down,
                            )),
                            None,
                        )
                    } else {
                        (None, None)
                    }
                }
                Direction::Right | Direction::Left => {
                    // Head Up
                    let up = if location.y - 1 > 0 {
                        Some((
                            Location {
                                x: location.x,
                                y: location.y - 1,
                            },
                            Direction::Up,
                        ))
                    } else {
                        None
                    };
                    // Head Down
                    let down = if location.y + 1 <= bounds.y {
                        Some((
                            Location {
                                x: location.x,
                                y: location.y + 1,
                            },
                            Direction::Down,
                        ))
                    } else {
                        None
                    };
                    (up, down)
                }
            }
        }
        Mirror::TiltLeft => {
            let next_location: Option<(Location, Direction)> = match direction {
                Direction::Right => {
                    // Head down
                    if location.y + 1 <= bounds.y {
                        Some((
                            Location {
                                x: location.x,
                                y: location.y + 1,
                            },
                            Direction::Down,
                        ))
                    } else {
                        None
                    }
                }
                Direction::Left => {
                    // Head up
                    if location.y - 1 > 0 {
                        Some((
                            Location {
                                x: location.x,
                                y: location.y - 1,
                            },
                            Direction::Up,
                        ))
                    } else {
                        None
                    }
                }
                Direction::Up => {
                    // Head Left
                    if location.x - 1 > 0 {
                        Some((
                            Location {
                                x: location.x - 1,
                                y: location.y,
                            },
                            Direction::Left,
                        ))
                    } else {
                        None
                    }
                }
                Direction::Down => {
                    // Head Right
                    if location.x + 1 <= bounds.x {
                        Some((
                            Location {
                                x: location.x + 1,
                                y: location.y,
                            },
                            Direction::Right,
                        ))
                    } else {
                        None
                    }
                }
            };
            match next_location {
                Some((new_location, new_direction)) => (Some((new_location, new_direction)), None),
                None => {
                    // We are complete with processing this path
                    (None, None)
                }
            }
        }
        Mirror::TiltRight => {
            let next_location: Option<(Location, Direction)> = match direction {
                Direction::Right => {
                    // Head Up
                    if location.y - 1 > 0 {
                        Some((
                            Location {
                                x: location.x,
                                y: location.y - 1,
                            },
                            Direction::Up,
                        ))
                    } else {
                        None
                    }
                }
                Direction::Left => {
                    // Head down
                    if location.y + 1 <= bounds.y {
                        Some((
                            Location {
                                x: location.x,
                                y: location.y + 1,
                            },
                            Direction::Down,
                        ))
                    } else {
                        None
                    }
                }
                Direction::Up => {
                    // Head Right
                    if location.x + 1 <= bounds.x {
                        Some((
                            Location {
                                x: location.x + 1,
                                y: location.y,
                            },
                            Direction::Right,
                        ))
                    } else {
                        None
                    }
                }
                Direction::Down => {
                    // Head Left
                    if location.x - 1 > 0 {
                        Some((
                            Location {
                                x: location.x - 1,
                                y: location.y,
                            },
                            Direction::Left,
                        ))
                    } else {
                        None
                    }
                }
            };
            match next_location {
                Some((new_location, new_direction)) => (Some((new_location, new_direction)), None),
                None => {
                    // We are complete with processing this path
                    (None, None)
                }
            }
        }
    }
}

fn part1(input: &str) -> u64 {
    process(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let result = part1(
            r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....",
        );
        assert_eq!(result, 46);
    }
}
