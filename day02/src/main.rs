use std::{collections::HashSet, fs};

type TDigit = u64;

struct Range {
    start: TDigit,
    end: TDigit,
}

fn pow10(exp: TDigit) -> TDigit {
    10u64.pow(exp.try_into().unwrap())
}

impl Range {
    fn find_invalid_ids_p1(&self) -> Vec<TDigit> {
        let start_digit_count = self.start.to_string().len();
        let end_digit_count = self.end.to_string().len();

        let digit_counts = (start_digit_count..=end_digit_count)
            .filter(|digits| digits % 2 == 0)
            .collect::<Vec<_>>();
        
        let mut invalid_ids : Vec<TDigit> = vec![];

        for digit_count in digit_counts {
            let segment_length = digit_count / 2;

            let lower_bound = pow10(segment_length as TDigit - 1);
            let upper_bound = pow10(segment_length as TDigit) - 1;

            let start_top = self.start / pow10(segment_length as TDigit);
            let end_top = self.end / pow10(segment_length as TDigit);

            let range_start = lower_bound.max(start_top);
            let range_end = upper_bound.min(end_top);

            for half in range_start..=range_end {
                let bottom = half;
                let top = half * pow10(digit_count as TDigit / 2);
                let id = top + bottom;
                if id < self.start || id > self.end {
                    continue;
                }

                invalid_ids.push(id);
            }
        }
        
        invalid_ids
    }

    fn find_invalid_ids_p2(&self) -> Vec<TDigit> {
        let start_digit_count = self.start.to_string().len();
        let end_digit_count = self.end.to_string().len();

        let mut invalid_ids : HashSet<TDigit> = HashSet::new();

        for digit_count in start_digit_count..=end_digit_count {
            for divisor in 2..=digit_count {
                if digit_count % divisor != 0 {
                    continue;
                }

                let segment_length = digit_count / divisor;
                let lower_bound = pow10(segment_length as TDigit - 1);
                let upper_bound = pow10(segment_length as TDigit) - 1;

                let start_segment = self.start / pow10(digit_count as TDigit - segment_length as TDigit);
                let end_segment = self.end / pow10(digit_count as TDigit - segment_length as TDigit);

                let range_start = lower_bound.max(start_segment);
                let range_end = upper_bound.min(end_segment);

                for segment in range_start..=range_end {
                    let mut id: TDigit = 0;
                    for i in 0..divisor {
                        id += segment * pow10(segment_length as TDigit * (divisor as TDigit - i as TDigit - 1));
                    }

                    if id < self.start || id > self.end {
                        continue;
                    }

                    invalid_ids.insert(id);
                }
            }
        }

        invalid_ids.into_iter().collect()
    }
}

impl From<&str> for Range {
    fn from(s: &str) -> Self {
        let parts: Vec<&str> = s.trim().split('-').collect();
        let start = parts[0].parse().unwrap();
        let end = parts[1].parse().unwrap();
        Range { start, end }
    }
}

fn main() {
    println!("Advent of Code 2025 - Day 2");
    // Your solution here

    let input = include_str!("../input.txt");
    let ranges = input
        .split(",")
        .map(|part| Range::from(part))
        .collect::<Vec<Range>>();

    let p1 : TDigit = ranges.iter()
        .map(|range| range.find_invalid_ids_p1())
        .flatten()
        .sum();

    println!("Part 1: {}", p1);

    let p2 : TDigit = ranges.iter()
        .map(|range| range.find_invalid_ids_p2())
        .flatten()
        .sum();

    println!("Part 2: {}", p2);
} 