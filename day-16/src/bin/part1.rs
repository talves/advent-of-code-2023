use std::{collections::HashMap, fmt::Display};

fn main() {
    let input = include_str!("../input.txt");
    let output = part1(input);
    dbg!(output);
}

enum Mirror {
    Horizontal,
    Vertical,
    TiltRight,
    TiltLeft,
    Empty,
}

enum Light {
    Exists,
    Energized,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Location {
    x: u16,
    y: u16,
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct Contraption {
    layout: HashMap<Location, Mirror>,
    energized: HashMap<Location, Light>,
}

impl Contraption {
    fn print_layout(&self, bounds: &Location) {
        for y in 1..bounds.y {
            for x in 1..=bounds.x {
                let location = Location { x, y };
                print!(
                    "{}",
                    match self.layout.get(&location) {
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
}

fn parse_input(input: &str) -> Contraption {
    let mut contraption = Contraption {
        layout: HashMap::new(),
        energized: HashMap::new(),
    };
    let mut x: u16 = 0;
    let mut y: u16 = 0;
    input.lines().for_each(|line| {
        y += 1; // start a new row
        x = 0; // reset col count
        line.chars().for_each(|c| {
            x += 1;
            let location = Location { x, y };
            match c {
                '.' => contraption.layout.insert(location, Mirror::Empty),
                '|' => contraption.layout.insert(location, Mirror::Vertical),
                '-' => contraption.layout.insert(location, Mirror::Horizontal),
                '/' => contraption.layout.insert(location, Mirror::TiltRight),
                '\\' => contraption.layout.insert(location, Mirror::TiltLeft),
                _ => panic!(
                    "{}",
                    format!("Invalid char: {} ({},{})", c, location.x, location.y)
                ),
            };
        });
    });
    contraption
}

fn process(input: &str) -> u64 {
    let bounds = Location {
        x: input.lines().next().unwrap().len() as u16,
        y: input.lines().count() as u16,
    };
    let contraption = parse_input(input);
    contraption.print_layout(&bounds);

    todo!()
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
