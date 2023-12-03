use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");

    let output = part2(input);
    dbg!(output);
}

#[derive(Debug)]
struct Grab<'a> {
    id: &'a str,
    qty: i32,
}
impl Grab<'_> {
    /// Creates a new Grab.
    fn new<'a>(id: &'a str, qty: &'a str) -> Grab<'a> {
        Grab {
            id,
            qty: qty.parse::<i32>().unwrap_or(0),
        }
    }
}

fn process(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            // bag contained only 12 red cubes, 13 green cubes, and 14 blue cubes
            let parts: Vec<&str> = line.split(':').collect();
            let grabbed_cubes = parts[1]
                .split(';')
                .map(|revealed| {
                    revealed
                        .trim()
                        .split(',')
                        .map(|cube| {
                            let grab = cube.trim().split(' ').collect::<Vec<&str>>();
                            Grab::new(grab[1], grab[0])
                        })
                        .collect()
                })
                .collect::<Vec<Vec<Grab>>>();
            // dbg!((id, &grabbed_cubes));
            let mut power: i32 = 1;
            let mut available_cubes = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
            for cubes_pick in grabbed_cubes {
                for cubes in cubes_pick {
                    if let Some(qty) = available_cubes.get_mut(&cubes.id) {
                        if *qty < cubes.qty {
                            *qty = cubes.qty;
                        }
                    }
                }
            }
            for val in available_cubes.values() {
                power *= val;
            }
            power
        })
        .sum()
}

fn part2(input: &str) -> i32 {
    process(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_works() {
        let result = part2(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
",
        );
        assert_eq!(result, 2286);
    }
}
