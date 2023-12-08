fn main() {
    let input = include_str!("../input.txt");
    let output = part2(input);
    dbg!(output);
}

#[derive(PartialEq, Debug)]
enum HandType {
    Five,
    Four,
    Full,
    Three,
    Twopair,
    Onepair,
    High,
}

#[derive(Debug)]
struct Hand<'a> {
    cards: &'a str,
    bid: u32,
    hand_type: HandType,
    strength: u32,
}

impl<'a> Hand<'a> {
    fn new(cards: &'a str, bid: u32) -> Hand {
        let hand_type: HandType = get_hand_type(&cards);
        let strength: u32 = get_strength(&cards);
        Hand {
            cards,
            bid,
            hand_type,
            strength,
        }
    }
}

fn get_hand_type_from_char(c: char, s: &String) -> HandType {
    match s.matches(c).count() {
        5 => HandType::Five,
        4 => HandType::Four,
        3 => HandType::Three,
        2 => HandType::Onepair,
        1 => HandType::High,
        _ => HandType::High,
    }
}

fn get_hand_type(cards: &str) -> HandType {
    if !cards.len() == 5 {
        panic!("Must have 5 cards in a hand");
    };
    let mut remaining = cards.to_string();
    let mut last_type = HandType::High;
    let mut hand_type = HandType::High;
    let mut joker_type: Option<HandType> = None;
    while remaining.len() > 0 {
        let c: char = remaining.chars().next().unwrap();
        if c == 'J' {
            joker_type = Some(get_hand_type_from_char(c, &remaining));
            remaining = remaining.replace(c, "");
        } else {
            last_type = match hand_type {
                HandType::Five => HandType::Five,
                HandType::Four => HandType::Four,
                HandType::Full => HandType::Full,
                HandType::Three => HandType::Three,
                HandType::Twopair => HandType::Twopair,
                HandType::Onepair => HandType::Onepair,
                HandType::High => HandType::High,
            };
            hand_type = get_hand_type_from_char(c, &remaining);
            remaining = remaining.replace(c, "");
            match hand_type {
                HandType::Five => {}
                HandType::Four => {}
                HandType::Full => {}
                HandType::Three => {
                    if last_type == HandType::Onepair {
                        hand_type = HandType::Full;
                    }
                }
                HandType::Twopair => {}
                HandType::Onepair => {
                    if last_type == HandType::Onepair {
                        hand_type = HandType::Twopair;
                    } else if last_type == HandType::Three {
                        hand_type = HandType::Full;
                    }
                }
                HandType::High => {
                    hand_type = match last_type {
                        HandType::Five => HandType::Five,
                        HandType::Four => HandType::Four,
                        HandType::Full => HandType::Full,
                        HandType::Three => HandType::Three,
                        HandType::Twopair => HandType::Twopair,
                        HandType::Onepair => HandType::Onepair,
                        HandType::High => HandType::High,
                    }
                }
            };
        }
    }
    match joker_type {
        Some(jt) => {
            hand_type = match jt {
                HandType::Five => HandType::Five,
                HandType::Four => HandType::Five,
                HandType::Full => HandType::Full,
                HandType::Three => match hand_type {
                    HandType::Five => HandType::Five,
                    HandType::Four => HandType::Four,
                    HandType::Full => HandType::Full,
                    HandType::Three => HandType::Three,
                    HandType::Twopair => HandType::Twopair,
                    HandType::Onepair => HandType::Five,
                    HandType::High => HandType::Four,
                },
                HandType::Twopair => HandType::Twopair,
                HandType::Onepair => match hand_type {
                    HandType::Five => HandType::Five,
                    HandType::Four => HandType::Four,
                    HandType::Full => HandType::Full,
                    HandType::Three => HandType::Five,
                    HandType::Twopair => HandType::Twopair,
                    HandType::Onepair => HandType::Four,
                    HandType::High => HandType::Three,
                },
                HandType::High => match hand_type {
                    HandType::Five => HandType::Five,
                    HandType::Four => HandType::Five,
                    HandType::Full => HandType::Full,
                    HandType::Three => HandType::Four,
                    HandType::Twopair => HandType::Full,
                    HandType::Onepair => HandType::Three,
                    HandType::High => HandType::Onepair,
                },
            }
        }
        None => {}
    }
    // Can't use back references for regex crate. See: https://codereview.stackexchange.com/questions/258836/comparing-poker-hands-in-rust
    // let five = Regex::new(r"(.)\1{4}.*#.*").unwrap();
    // let four = Regex::new(r"(.)\1{3}.*#.*").unwrap();
    // let full = Regex::new(r"((.)\2{2}(.)\3{1}#.*|(.)\4{1}(.)\5{2}#.*)").unwrap();
    // let three = Regex::new(r"(.)\1{2}.*#.*").unwrap();
    // let twopair = Regex::new(r"(.)\1{1}.*(.)\2{1}.*#.*").unwrap();
    // let onepair = Regex::new(r"(.)\1{1}.*#.*").unwrap();
    // if five.is_match(cards) {
    //     return HandType::Five;
    // } else if four.is_match(cards) {
    //     return HandType::Four;
    // } else if full.is_match(cards) {
    //     return HandType::Full;
    // } else if three.is_match(cards) {
    //     return HandType::Three;
    // } else if twopair.is_match(cards) {
    //     return HandType::Twopair;
    // } else if onepair.is_match(cards) {
    //     return HandType::Onepair;
    // } else {
    //     return HandType::High;
    // };
    hand_type
}

fn get_strength(cards: &str) -> u32 {
    let mut power = format!("");
    cards.chars().for_each(|c| {
        let next = match c {
            'A' => "13",
            'K' => "12",
            'Q' => "11",
            'T' => "10",
            '9' => "09",
            '8' => "08",
            '7' => "07",
            '6' => "06",
            '5' => "05",
            '4' => "04",
            '3' => "03",
            '2' => "02",
            'J' => "01",
            _ => "00",
        };
        power = format!("{}{}", power, next);
    });
    power.parse::<u32>().unwrap_or(0)
}

fn parse_input(input: &str) -> impl Iterator<Item = Hand> + '_ {
    input.lines().map(|line| {
        let parts: Vec<&str> = line.split(' ').collect();
        let cards = parts[0].trim();
        let bid = match parts[1].trim().parse::<u32>() {
            Ok(x) => x,
            Err(e) => {
                panic!("Could not parse bid: {e}")
            }
        };
        Hand::new(cards, bid)
    })
}

fn build_hands<'a>(hands: impl Iterator<Item = Hand<'a>>) -> impl Iterator<Item = Hand<'a>> {
    let mut five: Vec<Hand> = vec![];
    let mut four: Vec<Hand> = vec![];
    let mut full: Vec<Hand> = vec![];
    let mut three: Vec<Hand> = vec![];
    let mut two_pair: Vec<Hand> = vec![];
    let mut one_pair: Vec<Hand> = vec![];
    let mut high: Vec<Hand> = vec![];

    for hand in hands {
        match hand.hand_type {
            HandType::Five => five.push(hand),
            HandType::Four => four.push(hand),
            HandType::Full => full.push(hand),
            HandType::Three => three.push(hand),
            HandType::Twopair => two_pair.push(hand),
            HandType::Onepair => one_pair.push(hand),
            HandType::High => high.push(hand),
        }
    }
    five.sort_by_key(|x| x.strength);
    four.sort_by_key(|x| x.strength);
    full.sort_by_key(|x| x.strength);
    three.sort_by_key(|x| x.strength);
    two_pair.sort_by_key(|x| x.strength);
    one_pair.sort_by_key(|x| x.strength);
    high.sort_by_key(|x| x.strength);
    // change high to low
    // five.reverse();
    // four.reverse();
    // full.reverse();
    // three.reverse();
    // two_pair.reverse();
    // one_pair.reverse();
    // high.reverse();

    high.into_iter()
        .chain(one_pair.into_iter())
        .chain(two_pair.into_iter())
        .chain(three.into_iter())
        .chain(full.into_iter())
        .chain(four.into_iter())
        .chain(five.into_iter())
}

fn process(input: &str) -> u32 {
    let orig_hands = parse_input(input);
    let hands = build_hands(orig_hands);
    let total_winnings: u32 = hands
        .enumerate()
        .map(|(i, hand)| hand.bid * (i as u32 + 1))
        .sum();
    // dbg!(hands.collect::<Vec<Hand>>());
    total_winnings
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
            "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
        );
        assert_eq!(result, 5905);
    }

    #[test]
    fn hand_type_works() {
        assert_eq!(get_hand_type("J2T3K"), HandType::Onepair);
        assert_eq!(get_hand_type("KKKKA"), HandType::Four);
        assert_eq!(get_hand_type("AAJTT"), HandType::Full);
        assert_eq!(get_hand_type("45455"), HandType::Full);
        assert_eq!(get_hand_type("KTJJT"), HandType::Four);
        assert_eq!(get_hand_type("JKTJT"), HandType::Four);
        assert_eq!(get_hand_type("KKKKJ"), HandType::Five);
        assert_eq!(get_hand_type("JJ234"), HandType::Three);
        assert_eq!(get_hand_type("3K232"), HandType::Twopair);
        assert_eq!(get_hand_type("3J232"), HandType::Full);
    }

    #[test]
    fn strength_works() {
        assert_eq!(get_strength("32T3K"), 302100312);
        assert_eq!(get_strength("AAKTT"), 1313121010);
        assert_eq!(get_strength("45455"), 405040505);
        assert_eq!(get_strength("32K4T"), 302120410);
    }
}
