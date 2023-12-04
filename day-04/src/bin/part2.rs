fn main() {
    let input = include_str!("../input.txt");
    let output = part2(input);
    dbg!(output);
}

#[derive(Debug)]
struct Card {
    winning_numbers: Vec<u32>,
    copies: u32,
    wins: u32,
}

impl Card {
    fn new(input: &str) -> Card {
        Card {
            winning_numbers: input
                .trim()
                .split(' ')
                .map(|num| num.parse::<u32>().unwrap_or(0))
                .filter(|num| num > &0)
                .collect(),
            copies: 0,
            wins: 0,
        }
    }
    fn set_wins_from_str(&mut self, input: &str) -> u32 {
        let nums = input
            .trim()
            .split(' ')
            .map(|num| num.parse::<u32>().unwrap_or(0))
            .filter(|num| num > &0);
        for num in nums {
            for winner in self.winning_numbers.iter() {
                if num == *winner {
                    self.add_win();
                    break;
                }
            }
        }
        self.wins
    }
    fn add_win(&mut self) -> () {
        self.wins += 1;
    }
    fn set_copies(&mut self, copies: u32) -> () {
        self.copies = copies;
    }
}

fn process(input: &str) -> u32 {
    let mut cards = input
        .lines()
        .enumerate()
        .map(|(_id, line)| {
            let parts = line.split(':').collect::<Vec<&str>>()[1]
                .split("|")
                .collect::<Vec<&str>>();
            let card_winners = parts[0];
            let card_input = parts[1];
            let mut card = Card::new(card_winners);
            let _wins = card.set_wins_from_str(card_input);
            card
        })
        .collect::<Vec<Card>>();
    let mut copy_tracker = vec![0; cards.len()];
    for (i, card) in cards.iter().enumerate() {
        if card.wins > 0 {
            for idx in i + 1..=(i + card.wins as usize) {
                // one copy for each of the wins wins
                // also we get the number of copies for each of our own instances
                copy_tracker[idx] = copy_tracker[idx] + 1 + copy_tracker[i];
            }
        }
    }
    for i in 1..cards.len() {
        cards[i].set_copies(copy_tracker[i]);
    }
    dbg!(&cards);
    cards.iter().map(|card| 1 + card.copies).sum()
}

fn part2(input: &str) -> u32 {
    process(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_works() {
        let result = part2(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        assert_eq!(result, 30);
    }

    #[test]
    fn part2_new_card() {
        let card = Card::new("41 48 83 86 17");
        assert_eq!(card.winning_numbers, vec![41, 48, 83, 86, 17]);
    }

    #[test]
    fn part2_points() {
        let mut card = Card::new("41 48 83 86 17");
        assert_eq!(card.set_wins_from_str("83 86  6 31 17  9 48 53"), 4);
    }
}
