use std::{collections::HashSet, path::Display};

struct BatteryBank {
    cells: Vec<u64>,
}

impl BatteryBank {
    fn best2(&self) -> u64 {
        let best_first = self.cells[..self.cells.len()-1]
            .iter()
            .enumerate()
            // Choose the larger number, and in case of a tie, the one with the lower index
            .max_by(|(i1, num1), (i2, num2)| {
                if num1 == num2 {
                    i2.cmp(i1)
                } else {
                    num1.cmp(num2)
                }
            })
            .map(|(i, _num)| i)
            .unwrap();

        let best_second = self.cells[best_first + 1..]
            .iter()
            .enumerate()
            .max_by_key(|(_i, num)| **num)
            .map(|(i, _num)| i + best_first + 1)
            .unwrap();

        self.cells[best_first] * 10 + self.cells[best_second]
    }

    fn best12(&self) -> u64 {
        let mut indices = vec![];
        let mut last_allowed_index = 0;

        for place in 0..12 {
            let best = self.cells[last_allowed_index..self.cells.len() - (11 - place)]
                .iter()
                .enumerate()
                .max_by(|(i1, num1), (i2, num2)| {
                    if num1 == num2 {
                        i2.cmp(&i1)
                    } else {
                        num1.cmp(num2)
                    }
                })
                .map(|(i, _num)| i + last_allowed_index)
                .unwrap();

            indices.push(best);
            last_allowed_index = best + 1;
        }

        let mut result = 0;
        for index in indices {
            result = result * 10 + self.cells[index];
        }
        result
    }
}

impl std::fmt::Display for BatteryBank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for cell in &self.cells {
            write!(f, "{}", cell)?;
        }
        Ok(())
    }
}

fn main() {
    println!("Advent of Code 2025 - Day 3");
    // Your solution here

    let input = include_str!("../input.txt");

    let banks = input
        .lines()
        .map(|line| {
            let cells = line
                .chars()
                .map(|char| char.to_digit(10).unwrap() as u64)
                .collect();
            BatteryBank { cells }
        })
        .collect::<Vec<BatteryBank>>();

    let p1 : u64 = banks.iter()
        .map(|bank| bank.best2())
        .sum();

    println!("Part 1: {}", p1);

    let p2 : u64 = banks.iter()
        .map(|bank| bank.best12())
        .sum();

    println!("Part 2: {}", p2);
}
