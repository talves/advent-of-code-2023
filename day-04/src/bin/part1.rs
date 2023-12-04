fn main() {
    let input = include_str!("../input.txt");
    let output = part1(input);
    dbg!(output);
}

struct Game {
    winning_numbers: Vec<u32>,
}

impl Game {
    fn new(input: &str) -> Game {
        Game {
            winning_numbers: input
                .trim()
                .split(' ')
                .map(|num| num.parse::<u32>().unwrap_or(0))
                .filter(|num| num > &0)
                .collect(),
        }
    }
    fn get_points_from_str(&self, input: &str) -> u32 {
        let mut value: Option<u32> = None;

        let nums = input
            .trim()
            .split(' ')
            .map(|num| num.parse::<u32>().unwrap_or(0))
            .filter(|num| num > &0);
        for num in nums {
            self.winning_numbers.iter().for_each(|winner| {
                if num == *winner {
                    match value {
                        Some(val) => value = Some(val * 2),
                        None => value = Some(1),
                    }
                }
            })
        }
        value.unwrap_or(0)
    }
}

fn process(input: &str) -> u32 {
    let points = input.lines().enumerate().map(|(_id, line)| {
        let parts = line.split(':').collect::<Vec<&str>>()[1]
            .split("|")
            .collect::<Vec<&str>>();
        let game_winners = parts[0];
        let game_input = parts[1];
        let game = Game::new(game_winners);
        game.get_points_from_str(game_input)
    });
    dbg!(&points.clone().collect::<Vec<u32>>());
    points.sum()
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
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        assert_eq!(result, 13);
    }

    #[test]
    fn part1_new_game() {
        let game = Game::new("41 48 83 86 17");
        assert_eq!(game.winning_numbers, vec![41, 48, 83, 86, 17]);
    }

    #[test]
    fn part1_point() {
        let game = Game::new("41 48 83 86 17");
        assert_eq!(game.get_points_from_str("83 86  6 31 17  9 48 53"), 8);
    }
}
