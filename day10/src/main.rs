use core::panic;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Machine {
    goal : Vec<bool>,
    buttons : Vec<Vec<usize>>,
    joltage_goal : Vec<usize>,
}

struct P1SearchNode {
    lights : Vec<bool>,
    presses : usize,
}

impl P1SearchNode {
    fn is_solved(&self, machine : &Machine) -> bool {
        self.lights == machine.goal
    }

    fn press(&self, machine : &Machine, button_index : usize) -> P1SearchNode {
        let mut new_lights = self.lights.clone();
        for &light_index in &machine.buttons[button_index] {
            new_lights[light_index] = !new_lights[light_index];
        }

        P1SearchNode {
            lights: new_lights,
            presses: self.presses + 1,
        }
    }
}

struct P2SearchNode {
    joltages : Vec<usize>,
    presses : usize,
}

impl P2SearchNode {
    fn is_solved(&self, goal : &Vec<usize>) -> bool {
        &self.joltages == goal
    }

    fn is_invalid(&self, goal : &Vec<usize>) -> bool {
        for (joltage, goal_joltage) in self.joltages.iter().zip(goal.iter()) {
            if joltage > goal_joltage {
                return true;
            }
        }
        false
    }

    fn press(&self, machine : &Machine, button_index : usize) -> P2SearchNode {
        let mut new_joltages = self.joltages.clone();
        for &joltage_index in &machine.buttons[button_index] {
            new_joltages[joltage_index] += 1;
        }

        P2SearchNode {
            joltages: new_joltages,
            presses: self.presses + 1,
        }
    }
}

impl Machine {
    pub fn minimum_presses_p1(&self) -> usize {
        let mut queue = std::collections::VecDeque::new();
        let mut visited = std::collections::HashSet::new();

        let initial_node = P1SearchNode {
            lights: vec![false; self.goal.len()],
            presses: 0,
        };
        visited.insert(initial_node.lights.clone());
        queue.push_back(initial_node);
        
        while queue.len() > 0 {
            let node = queue.pop_front().unwrap();

            if node.is_solved(self) {
                return node.presses;
            }

            for index in 0..self.buttons.len() {
                let new_machine = node.press(self, index);

                if !visited.contains(&new_machine.lights) {
                    visited.insert(new_machine.lights.clone());
                    queue.push_back(new_machine);
                }
            }
        }

        panic!("No solution found");
    }

    pub fn minimum_presses_p2(&self) -> usize {
        let mut queue = std::collections::VecDeque::new();
        let mut visited = std::collections::HashSet::new();

        let initial_node = P2SearchNode {
            joltages: vec![0; self.joltage_goal.len()],
            presses: 0,
        };
        visited.insert(initial_node.joltages.clone());
        queue.push_back(initial_node);
        
        while queue.len() > 0 {
            let node = queue.pop_front().unwrap();

            if node.is_solved(&self.joltage_goal) {
                return node.presses;
            }

            for index in 0..self.buttons.len() {
                let new_machine = node.press(self, index);

                if !visited.contains(&new_machine.joltages) && !new_machine.is_invalid(&self.joltage_goal) {
                    visited.insert(new_machine.joltages.clone());
                    queue.push_back(new_machine);
                }
            }
        }

        panic!("No solution found");
    }
}

impl From<&str> for Machine {
    fn from(s: &str) -> Self {
        let parts = s.trim().split(" ").collect::<Vec<&str>>();

        let goal_lights = parts[0][1..parts[0].len()-1]
            .chars()
            .map(|ch| ch == '#')
            .collect::<Vec<bool>>();

        let mut buttons = vec![];
        for part in &parts[1..parts.len()-1] {
            buttons.push(
                part[1..part.len()-1]
                    .split(",")
                    .map(|num| num.parse().unwrap())
                    .collect()
            );
        }

        let joltages = parts[parts.len() - 1][1..parts[parts.len() - 1].len()-1]
            .split(",")
            .map(|num| num.parse().unwrap())
            .collect::<Vec<usize>>();

        Machine { 
            goal: goal_lights, 
            buttons, 
            joltage_goal: joltages 
        }
    }
}

fn main() {
    println!("Advent of Code 2025 - Day 10");
    // Your solution here

    let input = include_str!("../input.txt");
    let machines = input.lines()
        .map(|line| Machine::from(line))
        .collect::<Vec<Machine>>();

    let p1 = machines.iter()
        .map(|machine| machine.minimum_presses_p1() as i64)
        .sum::<i64>();

    println!("Part 1: {}", p1);

    let p2 = machines.iter()
        .map(|machine| {
            let solution = machine.minimum_presses_p2() as i64;
            println!("Machine solution: {}", solution);
            solution
        })
        .sum::<i64>();

    println!("Part 2: {}", p2);
}
