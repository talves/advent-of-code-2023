fn main() {
    let input = include_str!("../input.txt");
    let output = part1(input);
    dbg!(output);
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .replace("\n", "")
        .split(",")
        .map(|x| x.as_bytes().to_vec())
        .collect()
}

fn process(input: &str) -> u64 {
    let arr = parse_input(input);
    arr.iter()
        .map(|sequence| {
            let mut value = 0;
            for ascii_val in sequence {
                value += *ascii_val as u64;
                value *= 17;
                value %= 256;
            }
            dbg!(&value);
            value
        })
        .sum()
}

fn part1(input: &str) -> u64 {
    process(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_works() {
        let result = part1("HASH");
        assert_eq!(result, 52);
    }

    #[test]
    fn part1_works() {
        let result = part1(
            "rn=1,cm-,qp=3,cm=2,qp-,pc=4
,ot=9,ab=5,pc-,pc=6,ot=7",
        );
        assert_eq!(result, 1320);
    }
}
