use std::collections::{HashMap, HashSet};

use crate::common::Pos;

mod common;

fn get_box(line : &str) -> Pos<i32> {
    let parts: Vec<&str> = line.trim().split(',').collect();
    let x: i32 = parts[0].parse().unwrap();
    let y: i32 = parts[1].parse().unwrap();
    let z: i32 = parts[2].parse().unwrap();
    Pos::new(x, y, z)
}

fn connect(bx1 : Pos<i32>, bx2 : Pos<i32>, circuits : &mut Vec<Vec<Pos<i32>>>, box_map : &mut HashMap<Pos<i32>, usize>) {
    let in_map1 = box_map.get(&bx1).cloned();
    let in_map2 = box_map.get(&bx2).cloned();

    match (in_map1, in_map2) {
        (Some(index1), Some(index2)) => {
            if index1 != index2 {
                merge_circuits(index1, index2, circuits, box_map);
            }
        },
        (Some(index1), None) => {
            add_to_circuit(bx2, index1, circuits, box_map);
        },
        (None, Some(index2)) => {
            add_to_circuit(bx1, index2, circuits, box_map);
        },
        (None, None) => {
            create_circuit(bx1, bx2, circuits, box_map);
        }
    }
}

fn create_circuit(bx1 : Pos<i32>, bx2 : Pos<i32>, circuits : &mut Vec<Vec<Pos<i32>>>, box_map : &mut HashMap<Pos<i32>, usize>) {
    let mut new_circuit = vec![];
    new_circuit.push(bx1);
    new_circuit.push(bx2);
    let index = circuits.len();
    circuits.push(new_circuit);
    box_map.insert(bx1, index);
    box_map.insert(bx2, index);
}

fn add_to_circuit(bx : Pos<i32>, index : usize, circuits : &mut Vec<Vec<Pos<i32>>>, box_map : &mut HashMap<Pos<i32>, usize>) {
    circuits[index].push(bx);
    box_map.insert(bx, index);
}

fn merge_circuits(index1 : usize, index2 : usize, circuits : &mut Vec<Vec<Pos<i32>>>, box_map : &mut HashMap<Pos<i32>, usize>) {
    let circuit2 = circuits[index2].clone();

    // Merge circuit2 into circuit1
    circuits[index1].extend(circuit2.iter().cloned());

    // Update box_map to point to the merged circuit
    for b in &circuit2 {
        box_map.insert(*b, index1);
    }

    // Clear circuit2
    circuits[index2].clear();
}

fn get_circuits_p1(connections_count : usize, boxes : &Vec<Pos<i32>>) -> Vec<Vec<Pos<i32>>> {
    let mut box_map : HashMap<Pos<i32>, usize> = HashMap::new();
    let mut circuits : Vec<Vec<Pos<i32>>> = vec![];
    let mut connected_pairs : HashSet<(Pos<i32>, Pos<i32>)> = HashSet::new();

    let mut distances : Vec<(Pos<i32>, Pos<i32>, f64)> = boxes.iter()
        .flat_map(|bx1| {
            boxes.iter()
                .filter_map(move |bx2| {
                    if bx1 != bx2 {
                        Some(( *bx1, *bx2, bx1.distance(bx2)))
                    } else {
                        None
                    }
                })
        })
        .collect();

    distances.sort_by_key(|(_, _, d)| *d as u64);

    let mut count = 0;
    let mut d_index = 0;

    while count < connections_count {
        let (bx1, bx2, _) = distances[d_index];
        d_index += 1;

        if connected_pairs.contains(&(bx1, bx2)) || connected_pairs.contains(&(bx2, bx1)) {
            continue;
        }

        connect(bx1, bx2, &mut circuits, &mut box_map);
        connected_pairs.insert((bx1, bx2));
        count += 1;
    }

    for bx in boxes {
        if !box_map.contains_key(&bx) {
            // Create a new circuit for this box
            let index = circuits.len();
            circuits.push(vec![*bx]);
            box_map.insert(*bx, index);
        }
    }

    // Get rid of empty placeholder circuits
    circuits.into_iter().filter(|circuit| circuit.len() > 0).collect()
}

fn get_circuits_p2(boxes : &Vec<Pos<i32>>) -> i64 {
    let mut box_map : HashMap<Pos<i32>, usize> = HashMap::new();
    let mut circuits : Vec<Vec<Pos<i32>>> = vec![];
    let mut connected_pairs : HashSet<(Pos<i32>, Pos<i32>)> = HashSet::new();

    let mut distances : Vec<(Pos<i32>, Pos<i32>, f64)> = boxes.iter()
        .flat_map(|bx1| {
            boxes.iter()
                .filter_map(move |bx2| {
                    if bx1 != bx2 {
                        Some(( *bx1, *bx2, bx1.distance(bx2)))
                    } else {
                        None
                    }
                })
        })
        .collect();

    distances.sort_by_key(|(_, _, d)| *d as u64);

    let mut d_index = 0;
    let mut pos1;
    let mut pos2;

    loop {
        let (bx1, bx2, _) = distances[d_index];
        pos1 = bx1;
        pos2 = bx2;
        d_index += 1;

        if connected_pairs.contains(&(bx1, bx2)) || connected_pairs.contains(&(bx2, bx1)) {
            continue;
        }

        connect(bx1, bx2, &mut circuits, &mut box_map);
        connected_pairs.insert((bx1, bx2));

        if circuits[box_map[&bx1]].len() == boxes.len() {
            break;
        }
    }

    pos1.x as i64 * pos2.x as i64
}

fn main() {
    println!("Advent of Code 2025 - Day 8");
    // Your solution here

    let input = include_str!("../input.txt");
    let boxes = input
        .lines()
        .map(|line| get_box(line))
        .collect::<Vec<Pos<i32>>>();

    let mut circuits = get_circuits_p1(1000, &boxes);
    circuits.sort_by_key(|c| 10_000 - c.len()); // Reversed

    let p1 : usize = circuits.iter()
        .take(3)
        .map(|c| c.len())
        .product();

    println!("p1: {}", p1);

    let p2 = get_circuits_p2(&boxes);

    println!("p2: {}", p2);
}
