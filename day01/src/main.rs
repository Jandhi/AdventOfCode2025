use std::fs;

struct Dial {
    position: u32,
    zero_count_p1: u32,
    zero_count_p2: u32,
}

impl Dial {
    fn new() -> Self {
        Dial { position: 50, zero_count_p1: 0, zero_count_p2: 0 }
    }

    fn ingest_instruction(&mut self, instruction: &str) {
        let direction = &instruction[0..1];
        let steps: u32 = instruction[1..].trim().parse().expect("Not a number");

        match direction {
            "R" => self.rotate_right(steps),
            "L" => self.rotate_left(steps),
            _ => (),
        }
    }

    fn rotate_right(&mut self, steps: u32) {
        self.zero_count_p2 += (self.position + steps) / 100;

        self.position = (self.position + steps) % 100;
        if self.position == 0 {
            self.zero_count_p1 += 1;
        }
    }

    fn rotate_left(&mut self, steps: u32) {
        self.zero_count_p2 += (steps + 100 - self.position) / 100;
        
        if self.position == 0 {
            self.zero_count_p2 -= 1;
        }
        
        self.position = (self.position + 100 - (steps % 100)) % 100;
        if self.position == 0 {
            self.zero_count_p1 += 1;
        }
    }
}

fn main() {
    println!("Advent of Code 2025 - Day 1");
    // Your solution here

    let input = fs::read_to_string("day01/input.txt").expect("Failed to read file");
    let lines: Vec<&str> = input.lines().collect();
    let mut dial = Dial::new();

    for line in lines {
        dial.ingest_instruction(line);
    }

    println!("Part 1: {}", dial.zero_count_p1);
    println!("Part 2: {}", dial.zero_count_p2);
}
