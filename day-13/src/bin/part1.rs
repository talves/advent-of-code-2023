use std::str::FromStr;

fn main() {
    let input = include_str!("../input.txt");
    let output = part1(input);
    dbg!(output);
}

fn process(input: &str) -> u64 {
    let canvas: Canvas = Canvas::from_str(input).unwrap();
    canvas
        .patterns
        .iter()
        .enumerate()
        .map(|(i, pattern)| {
            let count = pattern.get_count();
            if count == 0 {
                panic!("Count is zero! Impossible #{}", i);
            }
            count
        })
        .sum()
}

fn part1(input: &str) -> u64 {
    process(input)
}

#[derive(Debug, PartialEq, Clone)]
struct Pattern {
    horizontal: Vec<String>,
    vertical: Vec<String>,
}

impl Pattern {
    fn new() -> Pattern {
        Pattern {
            horizontal: Vec::new(),
            vertical: Vec::new(),
        }
    }
    fn build_vertical(&mut self) {
        self.vertical = Vec::new();
        let len_of_vertical = self.horizontal[0].len();
        (0..len_of_vertical).for_each(|horizontal_idx| {
            let mut vertical: String = "".to_owned();
            self.horizontal.iter().for_each(|horizontal| {
                vertical = [
                    vertical.clone(),
                    horizontal[horizontal_idx..=horizontal_idx].to_owned(),
                ]
                .concat();
            });
            self.vertical.push(vertical);
        });
    }
    fn mirrored_count(v: &Vec<String>) -> usize {
        let window_match = v.windows(2);
        let matches = window_match
            .into_iter()
            .enumerate()
            .map(|(i, x)| (i, x))
            .filter(|(i, x)| x[0] == x[1])
            .map(|(i, _x)| i);
        let mut size = 0;
        for match_idx in matches {
            // Only enters here if there is a match on the find_map
            let left_idx = match_idx;
            let right_idx = match_idx + 1;
            let mut is_mirrored = true;
            // dbg!(&left_idx);
            // dbg!(&right_idx);
            // dbg!(&v.len() - 1);
            if right_idx != v.len() - 1 {
                let bound = if left_idx < (v.len() - 1 - right_idx) {
                    left_idx
                } else {
                    v.len() - 1 - right_idx
                };
                for idx in 1..=bound {
                    if v[left_idx - idx] != v[right_idx + idx] {
                        is_mirrored = false;
                        break;
                    }
                }
            }
            // Check to see if the match_idx has surrounding mirrored matches for all bounds
            if is_mirrored {
                size = left_idx + 1;
                break;
            }
        }
        size
    }

    fn get_count(&self) -> u64 {
        match Self::mirrored_count(&self.horizontal) {
            0 => Self::mirrored_count(&self.vertical) as u64,
            horizontal_count => 100 * horizontal_count as u64,
        }
    }
}

struct Canvas {
    patterns: Vec<Pattern>,
}

impl Canvas {
    fn new() -> Canvas {
        Canvas {
            patterns: Vec::new(),
        }
    }
    fn sum(&self) -> u64 {
        self.patterns
            .iter()
            .map(|pattern| pattern.get_count())
            .sum()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseCanvasError;

impl FromStr for Canvas {
    type Err = ParseCanvasError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        // Parse our input into our Canvas
        let mut patterns: Vec<Pattern> = Vec::new();
        let mut work_pattern: Pattern = Pattern::new();
        input.lines().for_each(|line| {
            if line.is_empty() {
                // store the horizontal in the last Pattern and build the vertical then add the Pattern to the Canvas
                work_pattern.build_vertical();
                patterns.push(work_pattern.clone());
                work_pattern = Pattern::new();
            } else {
                work_pattern.horizontal.push(line.to_owned());
            }
        });
        work_pattern.build_vertical();
        patterns.push(work_pattern.clone());

        Ok(Canvas { patterns })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let result = part1(
            "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
        );
        assert_eq!(result, 405);
    }

    #[test]
    fn parse_works() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

        let result = Canvas::from_str(input).unwrap();
        assert_eq!(result.patterns[0].horizontal.len(), 7);
        assert_eq!(result.patterns[0].vertical.len(), 9);
        assert_eq!(result.patterns[1].horizontal.len(), 7);
        assert_eq!(result.patterns[1].vertical.len(), 9);
        assert_eq!(result.patterns[0].vertical[0], "#.##..#");
        assert_eq!(result.patterns[0].vertical[8], "..##...");
        assert_eq!(result.patterns[1].vertical[0], "##.##.#");
        assert_eq!(result.patterns[1].vertical[8], "###..##");
    }

    #[test]
    fn count_works() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#

..##.##.##...
##.#......###
..########...
#####..######
..#......#...
.####..####..
#....##....##
....#..#.....
#...####...##
##.#....#.###
#...#..#...##
#####..######
##..#..#..###
.#...##...#..
..#.#..#.#...";

        let result = Canvas::from_str(input).unwrap();

        assert_eq!(result.patterns[0].get_count(), 5);
        assert_eq!(result.patterns[1].get_count(), 400);
        assert_eq!(result.patterns[2].get_count(), 12);
    }
}
