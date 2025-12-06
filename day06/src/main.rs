use core::num;

#[derive(Clone, Copy, Debug)]
enum Operator {
    Add,
    Multiply,
}

#[derive(Debug)]
struct Problem {
    nums : Vec<u64>,
    operator : Operator,
}

impl Problem {
    fn compute(&self) -> u64 {
        match self.operator {
            Operator::Add => self.nums.iter().sum(),
            Operator::Multiply => self.nums.iter().product(),
        }
    }

    fn problems_p1(input : &str) -> Vec<Problem> {
        let lines = input.lines()
            .collect::<Vec<_>>();

        let rows = (0..lines.len() - 1)
            .map(|i| {
                lines[i].split_whitespace()
                    .map(|num| num.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>()
            })
            .collect::<Vec<Vec<u64>>>();

        let operators = lines[lines.len() - 1].split_whitespace()
            .map(|op| match op {
                "+" => Operator::Add,
                "*" => Operator::Multiply,
                _ => panic!("Invalid operator"),
            })
            .collect::<Vec<Operator>>();

        (0..operators.len())
            .map(|i| {
                let mut nums = vec![];
                
                for row in &rows {
                    nums.push(row[i]);
                }

                Problem {
                    nums,
                    operator: operators[i].clone(),
                }
            })
            .collect::<Vec<Problem>>()
    }

    fn problems_p2(input : &str) -> Vec<Problem> {
        // Implement part 2 logic here if needed
        let mut problems : Vec<Problem> = vec![];

        let lines = input.lines()
            .collect::<Vec<_>>();
        let operator_line = lines[lines.len() - 1].chars().collect::<Vec<char>>();
        let other_lines = &lines[..lines.len() - 1]
            .iter()
            .map(|line| 
                line.chars()
                    .collect::<Vec<char>>()
            )
            .collect::<Vec<Vec<char>>>();

        let mut curr_nums = vec![];
        let mut curr_operator : char = ' ';

        for i in 0..operator_line.len() {
            if operator_line[i] != ' ' && curr_operator != ' ' {
                problems.push(Problem {
                    nums: curr_nums.clone(),
                    operator: match curr_operator {
                        '+' => Operator::Add,
                        '*' => Operator::Multiply,
                        _ => panic!("Invalid operator"),
                    },
                });

                curr_nums.clear();
            }

            if operator_line[i] != ' ' {
                curr_operator = operator_line[i];
            }

            if other_lines.iter().all(|line| line[i] == ' ') {
                continue;
            }

            let my_num = other_lines.iter()
                .map(|line| line[i])
                .collect::<String>()
                .trim()
                .parse::<u64>()
                .unwrap();
            
            curr_nums.push(my_num);
        }

        problems.push(Problem {
            nums: curr_nums.clone(),
            operator: match curr_operator {
                '+' => Operator::Add,
                '*' => Operator::Multiply,
                _ => panic!("Invalid operator"),
            },
        });

        problems
    }
}

fn main() {
    println!("Advent of Code 2025 - Day 6");
    // Your solution here

    let input = include_str!("../input.txt").replace("\r", "");

    let problems_p1 = Problem::problems_p1(&input);

    let p1 : u64 = problems_p1.iter()
        .map(|problem| problem.compute())
        .sum();

    println!("Part 1: {}", p1);

    let problems_p2 = Problem::problems_p2(&input);

    let p2 : u64 = problems_p2.iter()
        .map(|problem| problem.compute())
        .sum();

    println!("Part 2: {}", p2);
}
