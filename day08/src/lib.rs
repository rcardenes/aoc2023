use std::io::BufRead;
use std::collections::HashMap;

#[derive(Debug)]
enum Instruction {
    Left,
    Right,
}

#[derive(Debug)]
pub struct Node {
    left: String,
    right: String,
}

#[derive(Debug)]
pub struct Problem {
    instructions: Vec<Instruction>,
    nodes: HashMap<String, Node>
}

impl Problem {
    pub fn iterate(&self, start: &str, end_p: impl Fn(&str) -> bool) -> u64 {
        let mut steps = 0;
        let mut current_node = self.nodes.get(start).unwrap();

        loop {
            for inst in self.instructions.iter() {
                steps += 1;

                let next_node = match inst {
                    Instruction::Left => current_node.left.as_str(),
                    Instruction::Right => current_node.right.as_str(),
                };

                if end_p(next_node) {
                    return steps
                }

                current_node = self.nodes.get(next_node).unwrap();
            }
        }
    }

    pub fn parallel_iterate(&self, start: char, end: char) -> u64 {
        let starting_keys = self.nodes.keys()
            .filter(|&s| s.ends_with(start))
            .map(|s| s.as_str());

        // Now, find the path length for each ghost
        let lengths = starting_keys.map(|key| self.iterate(key, |s| s.ends_with(end)));

        // And find the LCM for the lenghts, that's our answer
        lengths.reduce(|a, b| num::integer::lcm(a, b)).unwrap()
    }
}

pub fn parse_instructions<R: BufRead>(stream: R) -> Problem {
    let mut lines = stream.lines();

    let instructions: Vec<Instruction> = lines.next().unwrap().expect("Something horrible happened")
        .chars()
        .map(|c| if c == 'L' { Instruction::Left } else { Instruction::Right })
        .collect();

    let _ = lines.next(); // Skip

    let nodes: HashMap<_, _> = HashMap::from_iter(lines.into_iter().map(|line| {
            let line = line.expect("Something horrible happened");
            let (node, edges) = line.split_once(" = ").unwrap();
            let (left, right) = edges[1..edges.len() - 1].split_once(", ").unwrap();

            (node.to_string(), Node { left: left.to_string(), right: right.to_string() })
        }));

    Problem {
        instructions,
        nodes,
    }
}
