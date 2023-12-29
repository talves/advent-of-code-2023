use std::{alloc::Layout, collections::HashMap, fmt::Display};

fn main() {
    let input = include_str!("../input.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug)]

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
    let mut layout: HashMap<Location, Mirror>;
    let mut energized: HashMap<Location, Light> = HashMap::new();
    (bounds, layout) = parse_input(input);

    fn process_light(
        location: &Location,
        direction: &Direction,
        layout: &mut HashMap<Location, Mirror>,
        energized: &mut HashMap<Location, Light>,
        bounds: &Location,
    ) {
        println!("location: {} direction: {:?}", location, direction);
        match layout.get(location) {
            Some(mirror) => {
                match mirror {
                    Mirror::Empty => {
                        // We mark light in this energized map and mark it Energized if it already has a light beam
                        energized
                            .entry(location.clone())
                            .and_modify(|x| *x = Light::Energized)
                            .or_insert(Light::Exists);

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
                            Some(new_location) => {
                                process_light(&new_location, direction, layout, energized, bounds)
                            }
                            None => {
                                // We are complete with processing this path
                            }
                        }
                    }
                    Mirror::Horizontal => {
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
                            Direction::Down => {
                                // Head Left
                                let new_x = location.x - 1;
                                if new_x > 0 {
                                    process_light(
                                        &Location {
                                            x: new_x,
                                            y: location.y,
                                        },
                                        &Direction::Left,
                                        layout,
                                        energized,
                                        bounds,
                                    )
                                };
                                // Head Right
                                let new_x = location.x + 1;
                                if new_x <= bounds.x {
                                    process_light(
                                        &Location {
                                            x: new_x,
                                            y: location.y,
                                        },
                                        &Direction::Right,
                                        layout,
                                        energized,
                                        bounds,
                                    )
                                };
                                None
                            }
                            Direction::Up => {
                                // process left path
                                let new_x = location.x - 1;
                                if new_x > 0 {
                                    process_light(
                                        &Location {
                                            x: new_x,
                                            y: location.y,
                                        },
                                        &Direction::Left,
                                        layout,
                                        energized,
                                        bounds,
                                    )
                                };
                                // Process right path
                                let new_x = location.x + 1;
                                if new_x <= bounds.x {
                                    process_light(
                                        &Location {
                                            x: new_x,
                                            y: location.y,
                                        },
                                        &Direction::Right,
                                        layout,
                                        energized,
                                        bounds,
                                    )
                                };
                                None
                            }
                        };
                        match next_location {
                            Some(new_location) => {
                                process_light(&new_location, direction, layout, energized, bounds)
                            }
                            None => {
                                // We are complete with processing this path
                            }
                        }
                    }
                    Mirror::Vertical => {
                        let next_location: Option<Location> = match direction {
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
                            Direction::Right => {
                                // Head Up
                                let new_y = location.y - 1;
                                if new_y > 0 {
                                    process_light(
                                        &Location {
                                            x: location.x,
                                            y: new_y,
                                        },
                                        &Direction::Up,
                                        layout,
                                        energized,
                                        bounds,
                                    )
                                };
                                // Head Down
                                let new_y = location.y + 1;
                                if new_y <= bounds.y {
                                    process_light(
                                        &Location {
                                            x: location.x,
                                            y: new_y,
                                        },
                                        &Direction::Down,
                                        layout,
                                        energized,
                                        bounds,
                                    )
                                };
                                None
                            }
                            Direction::Left => {
                                // Head Up
                                let new_y = location.y - 1;
                                if new_y > 0 {
                                    process_light(
                                        &Location {
                                            x: location.x,
                                            y: new_y,
                                        },
                                        &Direction::Up,
                                        layout,
                                        energized,
                                        bounds,
                                    )
                                };
                                // Head Down
                                let new_y = location.y + 1;
                                if new_y <= bounds.y {
                                    process_light(
                                        &Location {
                                            x: location.x,
                                            y: new_y,
                                        },
                                        &Direction::Down,
                                        layout,
                                        energized,
                                        bounds,
                                    )
                                };
                                None
                            }
                        };
                        match next_location {
                            Some(new_location) => {
                                process_light(&new_location, direction, layout, energized, bounds)
                            }
                            None => {
                                // We are complete with processing this path
                            }
                        }
                    }
                    Mirror::TiltLeft => {
                        let next_location: Option<(Location, Direction)> = match direction {
                            Direction::Right => {
                                // Head down
                                let new_y = location.y + 1;
                                if new_y <= bounds.y {
                                    Some((
                                        Location {
                                            x: location.x,
                                            y: new_y,
                                        },
                                        Direction::Down,
                                    ))
                                } else {
                                    None
                                }
                            }
                            Direction::Left => {
                                // Head up
                                let new_y = location.y - 1;
                                if new_y > 0 {
                                    Some((
                                        Location {
                                            x: location.x,
                                            y: new_y,
                                        },
                                        Direction::Up,
                                    ))
                                } else {
                                    None
                                }
                            }
                            Direction::Up => {
                                // Head Left
                                let new_x = location.x - 1;
                                if new_x > 0 {
                                    Some((
                                        Location {
                                            x: new_x,
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
                                let new_x = location.x + 1;
                                if new_x <= bounds.x {
                                    Some((
                                        Location {
                                            x: new_x,
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
                            Some((new_location, new_direction)) => process_light(
                                &new_location,
                                &new_direction,
                                layout,
                                energized,
                                bounds,
                            ),
                            None => {
                                // We are complete with processing this path
                            }
                        }
                    }
                    Mirror::TiltRight => {
                        let next_location: Option<(Location, Direction)> = match direction {
                            Direction::Right => {
                                // Head Up
                                let new_y = location.y - 1;
                                if new_y > 0 {
                                    Some((
                                        Location {
                                            x: location.x,
                                            y: new_y,
                                        },
                                        Direction::Up,
                                    ))
                                } else {
                                    None
                                }
                            }
                            Direction::Left => {
                                // Head down
                                let new_y = location.y + 1;
                                if new_y <= bounds.y {
                                    Some((
                                        Location {
                                            x: location.x,
                                            y: new_y,
                                        },
                                        Direction::Down,
                                    ))
                                } else {
                                    None
                                }
                            }
                            Direction::Up => {
                                // Head Right
                                let new_x = location.x + 1;
                                if new_x <= bounds.x {
                                    Some((
                                        Location {
                                            x: new_x,
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
                                let new_x = location.x - 1;
                                if new_x > 0 {
                                    Some((
                                        Location {
                                            x: new_x,
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
                            Some((new_location, new_direction)) => process_light(
                                &new_location,
                                &new_direction,
                                layout,
                                energized,
                                bounds,
                            ),
                            None => {
                                // We are complete with processing this path
                            }
                        }
                    }
                };
            }
            None => {
                panic!("Bad mirror stored: {}", location)
            }
        };
    }

    print_layout(&layout, &bounds);
    process_light(
        &Location { x: 1, y: 1 },
        &Direction::Right,
        &mut layout,
        &mut energized,
        &bounds,
    );
    println!("---------");
    print_energized(&energized, &bounds);

    energized.len() as u64
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
