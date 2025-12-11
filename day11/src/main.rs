use std::collections::HashMap;

struct Node {
    value : String,
    next : Vec<String>
}

fn count_paths(node_by_value : &HashMap<String, &Node>) -> usize {
    return count_paths_from(node_by_value, "you");
}

fn count_paths_from(node_by_value : &HashMap<String, &Node>, name : &str) -> usize {
    if name == "out" {
        return 1;
    }

    return node_by_value.get(name).unwrap().next.iter()
        .map(|next_name| count_paths_from(node_by_value, next_name))
        .sum();
}

fn count_paths_p2(node_by_value : &HashMap<String, &Node>) -> usize {
    return count_paths_from_p2(node_by_value, "svr", false, false, &mut HashMap::new());
}

fn count_paths_from_p2(node_by_value : &HashMap<String, &Node>, name : &str, mut visited_dac : bool, mut visited_fft : bool, cache: &mut HashMap<(String, bool, bool), usize>) -> usize {
    let key = (name.to_string(), visited_dac, visited_fft);
    
    if let Some(&result) = cache.get(&key) {
        return result;
    }

    if name == "out" {
        let result = if visited_dac && visited_fft { 1 } else { 0 };
        cache.insert(key, result);
        return result;
    }

    if name == "dac" {
        visited_dac = true;
    }

    if name == "fft" {
        visited_fft = true;
    }

    let result = node_by_value.get(name).unwrap().next.iter()
        .map(|next_name| count_paths_from_p2(node_by_value, next_name, visited_dac, visited_fft, cache))
        .sum();
    
    cache.insert(key, result);
    result
}

fn main() {
    println!("Advent of Code 2025 - Day 11");
    // Your solution here
    let input = include_str!("../input.txt");

    let nodes = input.lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let value = parts.next().unwrap().to_string();
            let rest = parts.next().unwrap();

            let next = rest.split(" ")
                .map(|s| s.trim().to_string())
                .collect::<Vec<String>>();
            Node { value, next }
        })
        .collect::<Vec<Node>>();

    let node_by_value = nodes.iter()
        .map(|node| (node.value.clone(), node))
        .collect::<HashMap<String, &Node>>();

    let p1 = count_paths(&node_by_value);
    println!("Part 1: {}", p1);

    let p2 = count_paths_p2(&node_by_value);
    println!("Part 2: {}", p2);
}
