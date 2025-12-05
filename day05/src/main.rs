struct IngredientDatabase {
    fresh_ranges : Vec<(u64, u64)>,
    ingredients : Vec<u64>
}

impl From<&str> for IngredientDatabase {
    fn from(value: &str) -> Self {
        let cleaned = value.replace("\r", "");
        let parts: Vec<&str> = cleaned.split("\n\n").collect();

        let fresh_ranges = parts[0]
            .lines()
            .map(|line| {
                let range_parts: Vec<&str> = line.split('-').collect();
                let start: u64 = range_parts[0].trim().parse().expect("Invalid range start");
                let end: u64 = range_parts[1].trim().parse().expect("Invalid range end");
                (start, end)
            })
            .collect();

        let ingredients = parts[1]
            .lines()
            .map(|line| line.trim().parse().expect("Invalid ingredient number"))
            .collect();

        Self {
            fresh_ranges,
            ingredients,
        }
    }
    
}

impl IngredientDatabase {
    fn is_fresh(&self, ingredient: u64) -> bool {
        self.fresh_ranges.iter()
            .any(|&(start, end)| ingredient >= start && ingredient <= end)
    }

    fn count_fresh_ingredients(&self) -> usize {
        self.ingredients.iter()
            .filter(|&&ing| self.is_fresh(ing))
            .count()
    }

    fn count_all_fresh_ingredients(&self) -> usize {
        let mut fresh_ranges : Vec<(u64, u64)> = vec![];

        for &(mut start, mut end) in &self.fresh_ranges {
            let mut insert = true;
            
            let mut i = 0;
            while i < fresh_ranges.len() {
                let (other_start, other_end) = fresh_ranges[i];

                if other_start <= start && end <= other_end {
                    // Current range is fully covered
                    insert = false;
                    break;
                } else if start <= other_start && other_end <= end {
                    // Current range fully covers existing range
                    fresh_ranges.remove(i);
                } else if start <= other_end && other_start <= end {
                    // Ranges overlap, merge them
                    start = other_start.min(start);
                    end = other_end.max(end);
                    fresh_ranges.remove(i);
                } else {
                    i += 1;
                }
            }

            if insert {
                fresh_ranges.push((start, end));
            }
        }

        fresh_ranges.iter()
            .map(|(start, end)| end - start + 1)
            .sum::<u64>() as usize
    }
}

fn main() {
    println!("Advent of Code 2025 - Day 5");
    
    let input = include_str!("../input.txt");
    let db : IngredientDatabase = input.into();

    println!("Part 1: {}", db.count_fresh_ingredients());
    println!("Part 2: {}", db.count_all_fresh_ingredients());
}
