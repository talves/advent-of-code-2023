use std::{str::FromStr, time::Instant};

fn main() {
    let input = include_str!("../input.txt");
    let output = part2(input);
    dbg!(output);
}

fn process(input: &str) -> u64 {
    let report = Report::from_str(input).unwrap();
    // report.iterate_sum(5)
    // report.sum()
    report.sum_build(5)
}

fn part2(input: &str) -> u64 {
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
    fn sum(&self) -> u64 {
        let mut line_count = 0;
        self.lines
            .iter()
            .map(|line| {
                line_count += 1;
                println!("Line {:?}: {:?}", line_count, &line.original);
                let start = Instant::now();
                let sum_count = line.get_count();
                let duration = start.elapsed();
                // dbg!(&sum_count);
                println!("Count: {:?} Time elapsed: {:?}", sum_count, duration);
                sum_count
            })
            .sum()
    }
    fn sum_build(&self, count: usize) -> u64 {
        let mut line_count = 0;
        self.lines
            .iter()
            .map(|line| {
                // build the new input str for the line
                let mut arrangement_str = line.arrangement[0].to_string();
                (1..line.arrangement.len()).for_each(|x| {
                    arrangement_str =
                        [&arrangement_str, ",", &line.arrangement[x].to_string()].concat();
                });
                let mut input = "";
                let binding = [input, &line.original, " "].concat().repeat(count);
                let binding = binding.trim().replace(" ", "?");
                input = &binding;
                let binding = [&arrangement_str, " "].concat().repeat(count);
                let binding = binding.trim().replace(" ", ",");
                let binding = [input, " ", &binding].concat();
                input = &binding;
                dbg!(&input);
                let new_line = &Report::from_str(input).unwrap().lines[0];
                line_count += 1;
                println!("Line {:?}: {:?}", line_count, &line.original);
                let start = Instant::now();
                let sum_count = new_line.get_count();
                let duration = start.elapsed();
                // dbg!(&sum_count);
                println!("Count: {:?} Time elapsed: {:?}", sum_count, duration);
                sum_count
            })
            .sum()
    }
    fn iterate_sum(&self, count: usize) -> u64 {
        let mut line_count = 0;
        self.lines
            .iter()
            .map(|line| {
                line_count += 1;
                println!("Line {:?}: {:?}", line_count, &line.original);
                let start = Instant::now();
                let sum_count = line.iterate_sum(count);
                let duration = start.elapsed();
                // dbg!(&sum_count);
                println!("Count: {:?} Time elapsed: {:?}", sum_count, duration);
                sum_count
            })
            .sum()
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

            // this feature is unstable at this time
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
    fn iterate_sum(&self, count: usize) -> u64 {
        fn get_sum(
            line: &ReportLine,
            count: usize,
            original: &String,
            original_arrangement: &String,
        ) -> Vec<u64> {
            if count == 0 {
                vec![0u64]
            } else if count == 1 {
                let pcount = line.get_count() as u64;
                // println!("End of permutations {}", pcount);
                vec![pcount]
            } else {
                let count = count - 1;
                line.get_unique_permutations()
                    .iter()
                    .map(|permutation| {
                        // dbg!(&permutation);
                        let mut str_permutation = line.original.to_string();
                        permutation.iter().enumerate().for_each(|(idx, x)| {
                            let ch = match *x {
                                0u8 => "#",
                                1u8 => ".",
                                _ => "",
                            };
                            str_permutation
                                .replace_range(line.unknown[idx]..=line.unknown[idx], ch);
                            // we should not need this next line, but keeping it for debug
                            // input = input.replace("?", ".");
                        });

                        // let mut input = [original, "?", &str_permutation].concat();
                        let mut input = [&str_permutation, "?", original].concat();
                        let mut arrangement_str = line.arrangement[0].to_string();
                        (1..line.arrangement.len()).for_each(|x| {
                            arrangement_str =
                                [&arrangement_str, ",", &line.arrangement[x].to_string()].concat();
                        });
                        input = [
                            input,
                            " ".to_owned(),
                            arrangement_str,
                            ",".to_owned(),
                            original_arrangement.to_string(),
                        ]
                        .concat();
                        // dbg!(&input);
                        let report = Report::from_str(&input).unwrap();
                        report
                            .lines
                            .iter()
                            .map(|line| {
                                get_sum(line, count, original, original_arrangement)
                                    .iter()
                                    .sum::<u64>()
                            })
                            .sum::<u64>()
                    })
                    .collect::<Vec<u64>>()
            }
        }
        let mut arrangement_str = self.arrangement[0].to_string();
        (1..self.arrangement.len()).for_each(|x| {
            arrangement_str = [&arrangement_str, ",", &self.arrangement[x].to_string()].concat();
        });
        // let line: ReportLine = ReportLine {
        //     original: self.original.clone(),
        //     sequence: self.sequence.clone(),
        //     arrangement: self.arrangement.clone(),
        //     unknown: self.unknown.clone(),
        //     damaged: self.damaged.clone(),
        // };
        // let input = &[
        //     self.original.clone(),
        //     "?".to_owned(),
        //     self.original.clone(),
        //     " ".to_owned(),
        //     arrangement_str.clone(),
        //     ",".to_owned(),
        //     arrangement_str.clone(),
        // ]
        // .concat();
        // dbg!(&input);
        let line = &Report::from_str(&[&self.original, " ", &arrangement_str.clone()].concat())
            .unwrap()
            .lines[0];
        get_sum(&line, count, &self.original, &arrangement_str)
            .iter()
            .sum()
    }

    fn get_unique_permutations(&self) -> Vec<Vec<u8>> {
        let damaged_count = self.damaged.len();
        let unknown_count = self.unknown.len();
        let damaged_needed: usize = self.arrangement.iter().map(|x| x).sum::<usize>();
        let damaged_missing: usize = damaged_needed - damaged_count;
        let skips_needed: usize = unknown_count - damaged_missing;

        // dbg!(&self.original);
        // dbg!(format!("{:?}", &self.arrangement));
        // dbg!(&damaged_count);
        // dbg!(&unknown_count);
        // dbg!(&damaged_needed);
        // dbg!(&damaged_missing);
        // dbg!(&skips_needed);

        // Create a mapping of our needed damaged fill using index of &str example: "##."
        // We are going to use u8 (1 byte) instead of the char for faster and less memory
        // '#' == Damaged == 0, '.' == Operational == 1
        let mut char_combination: Vec<u8> = Vec::new();
        (0..damaged_missing).for_each(|_| char_combination.push(0u8));
        (0..skips_needed).for_each(|_| char_combination.push(1u8));

        // The sequence we are looking for from original
        let sequence_needed: Vec<(Spring, usize)> = self
            .arrangement
            .iter()
            .map(|count| (Spring::Damaged, *count))
            .collect::<Vec<(Spring, usize)>>();
        // dbg!(format!("{:?}", &sequence_needed));
        let mut dedups_list: Vec<Vec<u8>> = Vec::new();

        // Get unique permutations of the chars needed above
        unique_permutations(char_combination)
            .iter()
            .for_each(|permutation| {
                if !dedups_list.contains(permutation) {
                    let mut workstr = self.original.to_string();
                    permutation.iter().enumerate().for_each(|(idx, x)| {
                        let ch = match *x {
                            0u8 => "#",
                            1u8 => ".",
                            _ => "",
                        };
                        workstr.replace_range(self.unknown[idx]..=self.unknown[idx], ch);
                        // we should not need this next line, but keeping it for debug
                        // workstr = workstr.replace("?", ".");
                    });
                    // dbg!(&workstr);
                    let sequence = Self::get_sequence_from(&workstr)
                        .iter()
                        .filter(|(spring, _count)| *spring == Spring::Damaged)
                        .map(|tuple| *tuple)
                        .collect::<Vec<(Spring, usize)>>();
                    // dbg!(format!("{:?}", &sequence));
                    if sequence == sequence_needed {
                        dedups_list.push(permutation.clone());
                    }
                }
            });
        // dbg!(&dedups_list);
        dedups_list
    }
    fn get_count(&self) -> u64 {
        Self::get_unique_permutations(&self)
            .len()
            .try_into()
            .unwrap()
    }

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

    //     #[test]
    //     fn part2_works() {
    //         let input = "???.### 1,1,3
    // .??..??...?##. 1,1,3
    // ?#?#?#?#?#?#?#? 1,3,1,6
    // ????.#...#... 4,1,1
    // ????.######..#####. 1,6,5
    // ?###???????? 3,2,1";
    //         let result = part2(input);
    //         assert_eq!(result, 21);
    //     }

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
        dbg!(&result.lines[0].original);
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
?###???????? 3,2,1
";
        //         let input = "???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3
        // .??..??...?##.?.??..??...?##.?.??..??...?##.?.??..??...?##.?.??..??...?##. 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3
        // ?#?#?#?#?#?#?#???#?#?#?#?#?#?#???#?#?#?#?#?#?#???#?#?#?#?#?#?#???#?#?#?#?#?#?#? 1,3,1,6,1,3,1,6,1,3,1,6,1,3,1,6,1,3,1,6
        // ????.#...#...?????.#...#...?????.#...#...?????.#...#...?????.#...#... 4,1,1,4,1,1,4,1,1,4,1,1,4,1,1
        // ????.######..#####.?????.######..#####.?????.######..#####.?????.######..#####.?????.######..#####. 1,6,5,1,6,5,1,6,5,1,6,5,1,6,5
        // ?###??????????###??????????###??????????###??????????###???????? 3,2,1,3,2,1,3,2,1,3,2,1,3,2,1";
        let result = Report::from_str(input).unwrap();
        assert_eq!(result.lines.len(), 6);
        assert_eq!(result.lines[0].get_count(), 1);
        assert_eq!(result.lines[1].get_count(), 4);
        assert_eq!(result.lines[2].get_count(), 1);
        assert_eq!(result.lines[3].get_count(), 1);
        assert_eq!(result.lines[4].get_count(), 4);
        assert_eq!(result.lines[5].get_count(), 10);
        // assert_eq!(result.lines[6].get_count(), 1);
        // assert_eq!(result.lines[7].get_count(), 16384);
        // assert_eq!(result.lines[8].get_count(), 1);
        // assert_eq!(result.lines[9].get_count(), 16);
        // assert_eq!(result.lines[10].get_count(), 2500);
        // assert_eq!(result.lines[11].get_count(), 506250);
    }

    #[test]
    fn report_sum_works() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        let result = Report::from_str(input).unwrap();
        assert_eq!(result.lines.len(), 6);
        assert_eq!(result.lines[0].iterate_sum(5), 1);
        assert_eq!(result.lines[1].iterate_sum(5), 16384);
        assert_eq!(result.lines[2].iterate_sum(5), 1);
        assert_eq!(result.lines[3].iterate_sum(5), 16);
        assert_eq!(result.lines[4].iterate_sum(5), 2500);
        assert_eq!(result.lines[5].iterate_sum(5), 506250);
    }
}
