use std::{
    collections::{hash_map::DefaultHasher, HashMap, HashSet},
    hash::{Hash, Hasher},
    str::FromStr,
};

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
enum Node {
    START,
    SMALL(usize),
    BIG(usize),
    END,
}
pub struct Input {
    connections: HashMap<Node, Vec<Node>>,
}

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut id = 0;
        let mut node_names = HashMap::new();
        let mut connections = HashMap::new();

        for line in s.lines() {
            let (from, to) = line.split_once('-').unwrap();
            let from_node = *node_names.entry(from).or_insert_with(|| {
                id += 1;
                Input::parse_node(from, id)
            });
            let to_node = *node_names.entry(to).or_insert_with(|| {
                id += 1;
                Input::parse_node(to, id)
            });
            if from_node != Node::END && to_node != Node::START {
                connections
                    .entry(from_node)
                    .or_insert(Vec::new())
                    .push(to_node);
            }
            if to_node != Node::END && from_node != Node::START {
                connections
                    .entry(to_node)
                    .or_insert(Vec::new())
                    .push(from_node);
            }
        }

        Ok(Self { connections })
    }
}

impl Input {
    fn parse_node(s: &str, id: usize) -> Node {
        if s == "start" {
            Node::START
        } else if s == "end" {
            Node::END
        } else if s.chars().next().unwrap().is_uppercase() {
            Node::BIG(id)
        } else {
            Node::SMALL(id)
        }
    }

    fn solve_part1(self) -> usize {
        let mut paths = HashSet::new();
        let mut stack = Vec::new();

        stack.push((Node::START, Vec::new()));

        while !stack.is_empty() {
            let (node, mut path) = stack.pop().unwrap();
            path.push(node);
            if node == Node::END {
                let mut hasher = DefaultHasher::new();
                Hash::hash_slice(&path, &mut hasher);
                paths.insert(hasher.finish());
                continue;
            }
            for &connection in self.connections.get(&node).unwrap_or(&Vec::new()) {
                match connection {
                    Node::SMALL(_) => {
                        if path.contains(&connection) {
                            continue;
                        }
                    }
                    _ => {}
                }
                stack.push((connection, path.clone()));
            }
        }

        paths.len()
    }

    fn solve_part2(self) -> usize {
        let mut paths = HashSet::new();
        let mut stack = Vec::new();

        stack.push((Node::START, Vec::new(), false));

        while !stack.is_empty() {
            let (node, mut path, visited_twice) = stack.pop().unwrap();
            path.push(node);
            if node == Node::END {
                let mut hasher = DefaultHasher::new();
                Hash::hash_slice(&path, &mut hasher);
                paths.insert(hasher.finish());
                continue;
            }
            for &connection in self.connections.get(&node).unwrap_or(&Vec::new()) {
                let mut visited = visited_twice;
                match connection {
                    Node::SMALL(_) => {
                        if path.contains(&connection) {
                            if visited {
                                continue;
                            } else {
                                visited = true;
                            }
                        }
                    }
                    _ => {}
                }
                stack.push((connection, path.clone(), visited));
            }
        }

        paths.len()
    }
}

pub fn input_generator(input: &str) -> Input {
    input.parse().unwrap()
}

pub fn part1(input: Input) -> usize {
    input.solve_part1()
}

pub fn part2(input: Input) -> usize {
    input.solve_part2()
}
