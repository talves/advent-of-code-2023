use std::str::FromStr;

fn main() {
    let input = include_str!("../input.txt");
    let output = part1(input);
    dbg!(output);
}

fn process(input: &str) -> u64 {
    let report = Report::from_str(input).unwrap();
    report
        .lines
        .iter()
        .map(|line| line.get_arrangement_count())
        .sum()
}

fn part1(input: &str) -> u64 {
    process(input)
}

#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(u8)]
enum Spring {
    Damaged = b'#',
    Operational = b'.',
    Unknown = b'?',
}
impl Spring {
    // Spring::from('|') == Ok(Spring::Verticle)
    pub fn from(c: char) -> Result<Spring, ()> {
        match c {
            '#' => Ok(Spring::Damaged),
            '.' => Ok(Spring::Operational),
            '?' => Ok(Spring::Unknown),
            _ => Err(()),
        }
    }
}
impl FromStr for Spring {
    type Err = ();

    fn from_str(s: &str) -> Result<Spring, ()> {
        match s {
            "#" => Ok(Spring::Damaged),
            "." => Ok(Spring::Operational),
            "?" => Ok(Spring::Unknown),
            _ => Err(()),
        }
    }
} // example: Springs::Verticle.into() == '|'
impl Into<char> for Spring {
    fn into(self) -> char {
        self as u8 as char
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
struct SpringLocation(usize, usize);

struct Report {
    lines: Vec<ReportLine>,
}

impl Report {
    fn new() -> Report {
        Report { lines: Vec::new() }
    }
}

#[derive(Debug, PartialEq)]

struct ReportLine {
    original: String,
    // each section in sequence for: (spring condition, len)
    // the sum of usize should always be the len of the line
    sequence: Vec<(Spring, usize)>,
    // arrangement of the damaged springs
    arrangement: Vec<usize>,
    unknown: Vec<usize>,
    damaged: Vec<usize>,
}

fn unique_permutations<T: Clone>(items: Vec<T>) -> Vec<Vec<T>>
where
    T: Ord,
{
    if items.len() == 1 {
        vec![items]
    } else {
        let mut output: Vec<Vec<T>> = vec![];

        // Obtain a list of the unique elements.
        // Sorting and deduping should be faster than using a hashset for most small n.
        let mut unique_items = items.clone();
        unique_items.sort();
        unique_items.dedup();
        for first in unique_items {
            let mut remaining_elements = items.clone();

            // this feature is unstable
            // remaining_elements.remove_item(first);

            let index = remaining_elements.iter().position(|x| *x == first).unwrap();
            remaining_elements.remove(index);

            for mut permutation in unique_permutations(remaining_elements) {
                permutation.insert(0, first.clone());
                output.push(permutation);
            }
        }
        output
    }
}

impl ReportLine {
    fn get_sequence_from(line: &String) -> Vec<(Spring, usize)> {
        let mut seq_count: usize = 0;
        // We'll use S as a start previous char placebo
        let mut sequence: Vec<(Spring, usize)> = Vec::new();
        let mut last_spring_type: Spring = Spring::Operational;
        let mut prev_char: char = 'S';
        line.chars().for_each(|c| {
            if prev_char != c {
                if prev_char != 'S' {
                    // sequence has changed, store previous, start new
                    sequence.push((last_spring_type, seq_count));
                }
                prev_char = c.clone();
                seq_count = 0;
            }
            seq_count += 1;
            match Spring::from(c) {
                Ok(spring_type) => match spring_type {
                    Spring::Damaged => {
                        last_spring_type = Spring::Damaged;
                    }
                    Spring::Operational => {
                        last_spring_type = Spring::Operational;
                    }
                    Spring::Unknown => {
                        last_spring_type = Spring::Unknown;
                    }
                },
                Err(_) => panic!("invalid input"),
            };
        });
        sequence.push((last_spring_type, seq_count));
        sequence
    }
    fn get_arrangement_count(&self) -> u64 {
        let damaged_count = self.damaged.len();
        let unknown_count = self.unknown.len();
        let damaged_needed: usize = self.arrangement.iter().map(|x| x).sum::<usize>();
        let damaged_missing: usize = damaged_needed - damaged_count;
        let skips_needed: usize = unknown_count - damaged_missing;

        // dbg!(&self.original);
        // dbg!(&damaged_count);
        // dbg!(&unknown_count);
        // dbg!(&damaged_needed);
        // dbg!(&damaged_missing);
        // dbg!(&skips_needed);

        // Create a mapping of our needed damaged fill using index of &str example: "##."
        let mut char_combination: Vec<&str> = Vec::new();
        (0..damaged_missing).for_each(|_| char_combination.push("#"));
        (0..skips_needed).for_each(|_| char_combination.push("."));

        let permutations = unique_permutations(self.unknown.clone());
        dbg!(format!("{:?}", &permutations.len()));

        let sequence_needed: Vec<(Spring, usize)> = self
            .arrangement
            .iter()
            .map(|count| (Spring::Damaged, *count))
            .collect::<Vec<(Spring, usize)>>();
        // dbg!(format!("{:?}", &sequence_needed));
        let mut dedups_list: Vec<String> = Vec::new();
        let count: u64 = permutations
            .iter()
            .map(|combo| {
                let mut workstr = self.original.to_string();
                combo.iter().enumerate().for_each(|(idx, pos)| {
                    workstr.replace_range(pos..=pos, char_combination[idx]);
                    // we should not need this next line, but keeping it for debug
                    // workstr = workstr.replace("?", ".");
                });
                if dedups_list.contains(&workstr) {
                    0u64
                } else {
                    let sequence = Self::get_sequence_from(&workstr)
                        .iter()
                        .filter(|(spring, _count)| *spring == Spring::Damaged)
                        .map(|tuple| *tuple)
                        .collect::<Vec<(Spring, usize)>>();
                    // dbg!(format!("{:?}", &sequence));
                    if sequence == sequence_needed {
                        dedups_list.push(workstr);
                        1u64
                    } else {
                        0u64
                    }
                }
            })
            .sum();

        dbg!(format!("{:?}", &dedups_list.len()));
        // dbg!(&count);
        count
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseReportError;

impl FromStr for Report {
    type Err = ParseReportError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        // Parse our input into our Report
        let lines: Vec<ReportLine> = input
            .lines()
            .map(|line| {
                let parts = line.split(' ').collect::<Vec<&str>>();
                let mut new_line: ReportLine = ReportLine::new();
                new_line.original = parts[0].to_string();
                new_line.arrangement = parts[1]
                    .split(',')
                    .collect::<Vec<&str>>()
                    .iter()
                    .map(|num| num.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();
                let mut damaged: Vec<usize> = Vec::new();
                let mut unknown: Vec<usize> = Vec::new();
                let mut seq_count: usize = 0;
                // We'll use S as a start previous char placebo
                let mut prev_char: char = 'S';
                let mut sequence: Vec<(Spring, usize)> = Vec::new();
                let mut last_spring_type: Spring = Spring::Operational;
                parts[0].chars().for_each(|c| {
                    if prev_char != c {
                        if prev_char != 'S' {
                            // sequence has changed, store previous, start new
                            sequence.push((last_spring_type, seq_count));
                        }
                        prev_char = c.clone();
                    }
                    seq_count += 1;
                    match Spring::from(c) {
                        Ok(spring_type) => match spring_type {
                            Spring::Damaged => {
                                last_spring_type = Spring::Damaged;
                                damaged.push(seq_count - 1);
                            }
                            Spring::Operational => {
                                last_spring_type = Spring::Operational;
                            }
                            Spring::Unknown => {
                                last_spring_type = Spring::Unknown;
                                unknown.push(seq_count - 1);
                            }
                        },
                        Err(_) => panic!("invalid input"),
                    };
                });
                sequence.push((last_spring_type, seq_count));
                new_line.sequence = sequence;
                new_line.unknown = unknown;
                new_line.damaged = damaged;
                new_line
            })
            .collect::<Vec<ReportLine>>();

        Ok(Report { lines })
    }
}

impl ReportLine {
    fn new() -> ReportLine {
        ReportLine {
            original: "".to_string(),
            sequence: Vec::new(),
            arrangement: Vec::new(),
            unknown: Vec::new(),
            damaged: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        let result = part1(input);
        assert_eq!(result, 21);
    }

    #[test]
    fn from_str_works() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        let result = Report::from_str(input).unwrap();
        assert_eq!(result.lines.len(), 6);
        assert_eq!(result.lines[0].arrangement, vec![1, 1, 3]);
        assert_eq!(result.lines[1].arrangement, vec![1, 1, 3]);
        assert_eq!(result.lines[2].arrangement, vec![1, 3, 1, 6]);
        assert_eq!(result.lines[3].arrangement, vec![4, 1, 1]);
        assert_eq!(result.lines[4].arrangement, vec![1, 6, 5]);
        assert_eq!(result.lines[5].arrangement, vec![3, 2, 1]);
        assert_eq!(result.lines[0].sequence[0], (Spring::Unknown, 3));
        assert_eq!(result.lines[0].sequence[1], (Spring::Operational, 4));
        assert_eq!(result.lines[0].sequence[2], (Spring::Damaged, 7));
        assert_eq!(result.lines[4].sequence[2], (Spring::Damaged, 11));
    }

    #[test]
    fn count_works() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        let result = Report::from_str(input).unwrap();
        assert_eq!(result.lines.len(), 6);
        assert_eq!(result.lines[0].get_arrangement_count(), 1);
        assert_eq!(result.lines[1].get_arrangement_count(), 4);
        assert_eq!(result.lines[2].get_arrangement_count(), 1);
        assert_eq!(result.lines[3].get_arrangement_count(), 1);
        assert_eq!(result.lines[4].get_arrangement_count(), 4);
        assert_eq!(result.lines[5].get_arrangement_count(), 10);
    }
}
