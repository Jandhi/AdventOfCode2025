mod common;

use std::collections::{HashMap, HashSet};

use crate::common::{Pos, PosIndexable};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Start,
    Splitter
}

struct Grid {
    tiles: Vec<Vec<Tile>>,
    start : Pos<usize>,
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let cleaned = value.replace("\r", "");
        let lines: Vec<&str> = cleaned.lines().collect();
        let height = lines.len();
        let width = lines[0].len();

        let mut tiles = vec![vec![Tile::Empty; width]; height];
        let mut start = Pos::default();

        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                tiles[y][x] = match ch {
                    'S' => {
                        start = Pos::new(x, y);
                        Tile::Start
                    },
                    '^' => Tile::Splitter,
                    _ => Tile::Empty,
                };
            }
        }

        Self {
            tiles,
            start,
        }
    }
}

impl Grid {
    fn count_splits(&self) -> usize {
        let mut count = 0;

        let mut beams = vec![self.start];
        let mut visited = HashSet::new();

        while beams.len() > 0 {
            let pos = beams.pop().unwrap();

            if pos.y + 1 >= self.tiles.len() || pos.x >= self.tiles[0].len() {
                continue;
            }

            if visited.contains(&pos) {
                continue;
            }
            visited.insert(pos);

            if *self.tiles.at(pos + Pos::y(1)) == Tile::Splitter {
                count += 1;

                if pos.x > 0 {
                    beams.push(Pos::new(pos.x - 1, pos.y + 1));
                }
                beams.push(Pos::new(pos.x + 1, pos.y + 1));
            } else {
                beams.push(Pos::new(pos.x, pos.y + 1));
            }
        }
        count
    }
}

#[derive(Debug)]
enum NextNode {
    None,
    Single(usize),
    Double(usize, usize),
}

#[derive(Debug)]
struct TimelineNode {
    pos : Pos<usize>,
    next : NextNode,
    calculated_timelines : Option<usize>
}

impl TimelineNode {
    fn from_grid(grid : &Grid) -> Vec<TimelineNode> {
        let mut nodes = vec![
        ];
        let mut pos_to_index : HashMap<Pos<usize>, usize> = HashMap::new();

        let mut node_queue : Vec<usize> = vec![];

        let mut pos = grid.start;
        while *grid.tiles.at(pos) != Tile::Splitter {
            pos.y += 1;
        }

        let start_node = TimelineNode {
            pos,
            next: NextNode::None,
            calculated_timelines: None
        };
        nodes.push(start_node);
        pos_to_index.insert(pos, 0);
        node_queue.push(0);

        while node_queue.len() > 0 {
            let node_index = node_queue.pop().unwrap();

            let mut left = nodes[node_index].pos - Pos::x(1);
            while grid.tiles.at(left) != &Tile::Splitter && left.y + 1 < grid.tiles.len() {
                left.y += 1;
            }

            if grid.tiles.at(left) == &Tile::Splitter {
                if let Some(&left_index) = pos_to_index.get(&left) {
                    nodes[node_index].next = match nodes[node_index].next {
                        NextNode::None => NextNode::Single(left_index),
                        NextNode::Single(first) => NextNode::Double(first, left_index),
                        NextNode::Double(_, _) => panic!("A node cannot have more than two next nodes"),
                    };
                } else {
                    let left_node = TimelineNode {
                        pos: left,
                        next: NextNode::None,
                        calculated_timelines: None
                    };
                    nodes.push(left_node);
                    let left_index = nodes.len() - 1;
                    pos_to_index.insert(left, left_index);
                    node_queue.push(left_index);
                    
                    nodes[node_index].next = match nodes[node_index].next {
                        NextNode::None => NextNode::Single(left_index),
                        NextNode::Single(first) => NextNode::Double(first, left_index),
                        NextNode::Double(_, _) => panic!("A node cannot have more than two next nodes"),
                    };
                }
            }
            
            let mut right = nodes[node_index].pos + Pos::x(1);
            while grid.tiles.at(right) != &Tile::Splitter && right.y + 1 < grid.tiles.len() {
                right.y += 1;
            }

            if grid.tiles.at(right) == &Tile::Splitter {
                if let Some(&right_index) = pos_to_index.get(&right) {
                    nodes[node_index].next = match nodes[node_index].next {
                        NextNode::None => NextNode::Single(right_index),
                        NextNode::Single(first) => NextNode::Double(first, right_index),
                        NextNode::Double(_, _) => panic!("A node cannot have more than two next nodes"),
                    };
                } else {
                    let right_node = TimelineNode {
                        pos: right,
                        next: NextNode::None,
                        calculated_timelines: None
                    };
                    nodes.push(right_node);
                    let right_index = nodes.len() - 1;
                    pos_to_index.insert(right, right_index);
                    node_queue.push(right_index);
                    
                    nodes[node_index].next = match nodes[node_index].next {
                        NextNode::None => NextNode::Single(right_index),
                        NextNode::Single(first) => NextNode::Double(first, right_index),
                        NextNode::Double(_, _) => panic!("A node cannot have more than two next nodes"),
                    };
                }
            }
        }

        nodes
    }

    fn calculate_timelines(nodes : &mut Vec<TimelineNode>, index : usize) -> usize {
        if let Some(count) = nodes[index].calculated_timelines {
            return count;
        }
        let count = match nodes[index].next {
            NextNode::None => 2,
            NextNode::Single(next) => 1 + Self::calculate_timelines(nodes, next),
            NextNode::Double(first, second) => {
                Self::calculate_timelines(nodes, first) + Self::calculate_timelines(nodes, second)
            }
        };

        nodes[index].calculated_timelines = Some(count);
        count
    }
}

fn main() {
    println!("Advent of Code 2025 - Day 7");
    // Your solution here

    let input = include_str!("../input.txt");
    let grid : Grid = input.into();
    let splits = grid.count_splits();
    println!("p1: {}", splits);

    let mut nodes = TimelineNode::from_grid(&grid);

    println!("p2: {}", TimelineNode::calculate_timelines(&mut nodes, 0));
}
