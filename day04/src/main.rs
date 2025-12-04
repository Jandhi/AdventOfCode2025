struct Grid {
    rolls: Vec<Vec<bool>>
}

impl Grid {
    fn count_forkliftable(&self) -> usize {
        let mut count = 0;
        for y in 0..self.rolls.len() {
            for x in 0..self.rolls[0].len() {
                if self.is_forkliftable(x, y) {
                    count += 1;
                }
            }
        }
        count
    }

    fn is_forkliftable(&self, x : usize, y : usize) -> bool {
        if !self.rolls[y][x] {
            return false;
        }

        let offsets = [(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (-1, 1), (1, -1), (1, 1)];
        offsets.iter()
            .map(|(dx, dy)| {
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx < 0 || ny < 0 || ny as usize >= self.rolls.len() || nx as usize >= self.rolls[0].len() {
                    0
                }  else if self.rolls[ny as usize][nx as usize] {
                    1
                } else {
                    0 
                }
            })
            .sum::<usize>() < 4
    }

    fn remove_forkliftables(&mut self) -> usize {
        let mut changed = true;
        let mut removed = 0;

        while changed {
            changed = false;

            for y in 0..self.rolls.len() {
                for x in 0..self.rolls[0].len() {
                    if self.is_forkliftable(x, y) {
                        self.rolls[y][x] = false;
                        changed = true;
                        removed += 1;
                    }
                }
            }
        }
        
        removed
    }
}

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        let rolls = input.lines()
            .map(|line| line.chars()
                .map(|char| match char {
                    '@' => true,
                    '.' => false,
                    _ => panic!("Invalid character in input"),
                })
                .collect()
            )
            .collect();

        Grid { rolls }
    }
}



fn main() {
    let input = include_str!("../input.txt");
    let mut grid : Grid = input.into();

    let p1 = grid.count_forkliftable();
    println!("Part 1: {}", p1);

    let p2 = grid.remove_forkliftables();
    println!("Part 2: {}", p2);
}
