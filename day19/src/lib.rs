use std::{io::BufRead, collections::HashMap};

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
enum ByteCode {
    Comparison { op: char, attr: char, value: usize, act: Action },
    Do(Action),
}

#[derive(Debug)]
struct Workflow {
    bytecode: Vec<ByteCode>
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
pub struct Evaluator {
    workflows: HashMap<String, Workflow>,
}

impl Evaluator {
    pub fn is_accepted(&self, part: &Part) -> bool {
        let mut current_workflow = self.workflows.get("in").unwrap();

        loop {
            for bc in current_workflow.bytecode.iter() {
                let action = match bc {
                    ByteCode::Comparison { op, attr, value, act } => {
                        let part_val = part.get_rating(*attr);
                        let matches = if *op == '<' { part_val < *value } else { part_val > *value };
                        if matches { Some(act) } else { None }
                    }
                    ByteCode::Do(act) => Some(act),
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
                ByteCode::Comparison {
                    op: ch.next().unwrap(),
                    attr: var,
                    value: value.parse::<usize>().unwrap(),
                    act: act.into()
                }
            } else {
                ByteCode::Do(inst.into())
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
