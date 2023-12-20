use std::{io::BufRead, collections::HashMap, ops::Range};

#[derive(Debug)]
enum Action {
    Accept,
    Reject,
    JumpTo(String),
}

impl From<&str> for Action {
    fn from(value: &str) -> Self {
        match value {
            "A" => Action::Accept,
            "R" => Action::Reject,
            other => Action::JumpTo(other.to_string()),
        }
    }
}

#[derive(Debug)]
enum Bytecode {
    Comparison { op: char, attr: char, value: usize, act: Action },
    Do(Action),
}

#[derive(Debug)]
struct Workflow {
    bytecode: Vec<Bytecode>
}

#[derive(Debug)]
pub struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    pub fn combined_rating(&self) -> usize {
        self.x + self.m + self.a + self.s
    }

    pub fn get_rating(&self, attr: char) -> usize {
        match attr {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _ => unimplemented!() // Won't happen. Really
        }
    }
}

#[derive(Debug)]
pub struct AcceptablePart {
    x: Range<usize>,
    m: Range<usize>,
    a: Range<usize>,
    s: Range<usize>,
}

fn split_range(current: Range<usize>, op: char, split_point: usize) -> (Range<usize>, Range<usize>) {
    if op == '<' {
        (current.start..split_point, split_point..current.end)
    } else {
        (split_point+1..current.end, current.start..split_point+1)
    }
}

impl AcceptablePart {
    fn new() -> Self {
        AcceptablePart { x: 1..4001, m: 1..4001, a: 1..4001, s: 1..4001 }
    }

    fn split_using(self, op: char, attr: char, split_point: usize) -> (AcceptablePart, AcceptablePart) {
        match attr {
            'x' => {
                let (r1, r2) = split_range(self.x, op, split_point);
                (
                    AcceptablePart { x: r1, m: self.m.clone(), a: self.a.clone(), s: self.s.clone() },
                    AcceptablePart { x: r2, m: self.m, a: self.a, s: self.s },
                )
            }
            'm' => {
                let (r1, r2) = split_range(self.m, op, split_point);
                (
                    AcceptablePart { m: r1, x: self.x.clone(), a: self.a.clone(), s: self.s.clone() },
                    AcceptablePart { m: r2, x: self.x, a: self.a, s: self.s },
                )
            }
            'a' => {
                let (r1, r2) = split_range(self.a, op, split_point);
                (
                    AcceptablePart { a: r1, x: self.x.clone(), m: self.m.clone(), s: self.s.clone() },
                    AcceptablePart { a: r2, x: self.x, m: self.m, s: self.s },
                )
            }
            's' => {
                let (r1, r2) = split_range(self.s, op, split_point);
                (
                    AcceptablePart { s: r1, x: self.x.clone(), m: self.m.clone(), a: self.a.clone() },
                    AcceptablePart { s: r2, x: self.x, m: self.m, a: self.a },
                )
            }
            _ => unimplemented!() // No more cases, really
        }
    }

    pub fn combinations(&self) -> u64 {
        (self.x.len() * self.m.len() * self.a.len() * self.s.len()) as u64
    }
}

#[derive(Debug)]
pub struct InteriorNode {
    cond: (char, char, usize),
    yes: Node,
    no: Node,
}

#[derive(Debug)]
pub enum Node {
    Accept,
    Reject,
    Interior(Box<InteriorNode>),
}

impl Node {
    pub fn traverse(&self) -> Vec<AcceptablePart> {
        self.rec_traverse(AcceptablePart::new())
    }

    fn rec_traverse(&self, part: AcceptablePart) -> Vec<AcceptablePart> {
        let mut result = vec![];

        if let Node::Interior(node) = self {
            let (op, attr, val) = node.cond;
            let (yes_part, no_part) = part.split_using(op, attr, val);
            for (new_part, further_node) in [(yes_part, &node.yes), (no_part, &node.no)] {
                match further_node {
                    Node::Reject => { },
                    Node::Accept => { result.push(new_part) },
                    other => result.extend(other.rec_traverse(new_part)),
                }
            }
        } else {
            panic!("Shouldn't be here!")
        }

        result
    }
}

#[derive(Debug)]
pub struct Evaluator {
    workflows: HashMap<String, Workflow>,
}

impl Evaluator {
    pub fn is_accepted(&self, part: &Part) -> bool {
        let mut current_workflow = self.workflows.get("in").unwrap();

        loop {
            for bc in current_workflow.bytecode.iter() {
                let action = match bc {
                    Bytecode::Comparison { op, attr, value, act } => {
                        let part_val = part.get_rating(*attr);
                        let matches = if *op == '<' { part_val < *value } else { part_val > *value };
                        if matches { Some(act) } else { None }
                    }
                    Bytecode::Do(act) => Some(act),
                };
                match action {
                    Some(Action::Accept) => return true,
                    Some(Action::Reject) => return false,
                    Some(Action::JumpTo(foo)) => {
                        current_workflow = self.workflows.get(foo).unwrap();
                        break
                    }
                    None => {}
                }
            }
        }
    }

    fn build_action_node(&self, act: &Action) -> Node {
        match act {
            Action::Accept => Node::Accept,
            Action::Reject => Node::Reject,
            Action::JumpTo(other) => {
                let next_workflow = self.workflows.get(other.as_str()).unwrap();
                self.build_tree(next_workflow.bytecode.iter())
            }
        }
    }

    fn build_tree(&self, mut bc: std::slice::Iter<Bytecode>) -> Node {
        match bc.next().unwrap() {
            Bytecode::Comparison { op, attr, value, act } => {
                Node::Interior(
                    Box::new(
                        InteriorNode {
                            cond: (*op, *attr, *value),
                            yes: self.build_action_node(act),
                            no: self.build_tree(bc),
                        }))
            }
            Bytecode::Do(act) => self.build_action_node(act),
        }
    }

    pub fn as_tree(&self) -> Node {
        let root_workflow = self.workflows.get("in").unwrap();
        self.build_tree(root_workflow.bytecode.iter())
    }
}

fn compile_workflow(source: &str) -> (String, Workflow) {
    let (name, right) = source.split_once("{").unwrap();
    let rest = &right[..right.len()-1];

    let instructions = rest.split(",")
        .map(|inst| {
            if let Some((cond, act)) = inst.split_once(":") {
                let mut ch = cond.chars();
                let var = ch.next().unwrap();
                let value = &cond[2..];
                Bytecode::Comparison {
                    op: ch.next().unwrap(),
                    attr: var,
                    value: value.parse::<usize>().unwrap(),
                    act: act.into()
                }
            } else {
                Bytecode::Do(inst.into())
            }
        })
        .collect::<Vec<_>>();

    (name.to_string(), Workflow { bytecode: instructions })
}

pub fn read_problem<R: BufRead>(stream: R) -> (Evaluator, Vec<Part>) {
    let mut lines = stream.lines().map(|l| l.unwrap()).into_iter();
    // First read the workflows
    let workflows: Vec<(String, Workflow)> = lines
        .by_ref()
        .take_while(|l| !l.is_empty())
        .map(|l| compile_workflow(l.as_str()))
        .collect();

    let parts: Vec<Part> = lines
        .map(|l| {
            let no_terminators = &l[1..l.len() - 1];
            let mut sp = no_terminators.split(",");
            Part {
                x: sp.next().unwrap()[2..].parse::<usize>().unwrap(),
                m: sp.next().unwrap()[2..].parse::<usize>().unwrap(),
                a: sp.next().unwrap()[2..].parse::<usize>().unwrap(),
                s: sp.next().unwrap()[2..].parse::<usize>().unwrap(),
            }
        })
        .collect();

    (
        Evaluator { workflows: HashMap::from_iter(workflows) },
        parts
    )
}
