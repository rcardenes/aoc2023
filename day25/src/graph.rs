use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone)]
pub struct Edge {
    capacity: usize,
    flow: i64,
}

impl Edge {
    fn new() -> Self {
        Edge {
            capacity: 1,
            flow: 0,
        }
    }

    fn residual_capacity(&self) -> usize {
        ((self.capacity as i64) - self.flow) as usize
    }

    fn add_flow(&mut self, flow: i64) {
        self.flow += flow
    }
}

#[derive(Default, Debug, Clone)]
pub struct Graph {
    mapping: HashMap<String, HashMap<String, Edge>>,
}

impl Graph {
    fn add_directed_edge(&mut self, orig: &str, dest: &str) {
        self.mapping.entry(orig.to_string())
            .and_modify(|e| { e.insert(dest.to_string(), Edge::new()); })
            .or_insert(HashMap::from_iter(vec![(dest.to_string(), Edge::new())]));
    }

    pub fn add_edge(&mut self, orig: &str, dest: &str) {
        self.add_directed_edge(orig, dest);
        self.add_directed_edge(dest, orig);
    }

    pub fn ford_fulkerson(&self, source: &str, sink: &str) -> Graph {
        // First we create a copy of this graph, that will be our residual
        let mut residual = self.clone();

        while let Some(Path { vertices, max_flow }) = residual.find_path(source, sink) {
            let mut current = "sink".to_string();
            for v in vertices {
                residual.mapping.entry(current.clone())
                    .and_modify(|e| {
                        e.entry(v.clone()).and_modify(|f| f.add_flow(max_flow as i64));
                    });
                residual.mapping.entry(v.clone())
                    .and_modify(|e| {
                        e.entry(current).and_modify(|f| f.add_flow(-(max_flow as i64)));
                    });
                current = v;
            }
        }

        residual
    }

    fn dfs(&self, current: &str, visited: &mut Vec<String>) -> Vec<(String, String)> {
        let mut ret = vec![];

        visited.push(current.to_string());
        if let Some(edges) = self.mapping.get(current) {
            for (end, edge) in edges.iter() {
                if edge.residual_capacity() > 0 {
                    if !visited.contains(end) {
                        ret.extend(self.dfs(end.as_str(), visited));
                    }
                } else {
                    ret.push((current.to_string(), end.clone()));
                }
            }
        }

        ret
    }

    pub fn min_cut(&self, source: &str) -> Vec<(String, String)> {
        let mut visited = vec![];
        self.dfs(source, &mut visited)
            .into_iter()
            .filter(|(_, b)| !visited.contains(b) )
            .collect::<Vec<_>>()
    }

    pub fn partition(&self, source: &str) -> (Vec<String>, Vec<String>) {
        let mut visited_source = vec![];
        let _ = self.dfs(source, &mut visited_source);
        let sink_side = self.mapping
            .keys()
            .filter(|&s| !visited_source.contains(s))
            .cloned()
            .collect();

        (visited_source, sink_side)
    }

    pub fn to_dot(&self, cut: Vec<(String, String)>) {
        println!("graph {{");
        println!("\tedge [style=dotted];");
        let mut produced: HashSet<(String, String)> = HashSet::new();
        for (vertex, edges) in self.mapping.iter() {
            for (dest, _) in edges.iter() {
                let reverse = (dest.clone(), vertex.clone());
                if produced.contains(&reverse) {
                    continue
                }
                let direct = (vertex.clone(), dest.clone());
                let is_cut = cut.contains(&direct) || cut.contains(&reverse);
                if !is_cut {
                    println!("\t{vertex} -- {dest};");
                }

                produced.insert(direct);
            }
        }
        println!("}}");
    }

    fn find_path(&self, source: &str, sink: &str) -> Option<Path> {
        let mut queue: VecDeque<String> = VecDeque::new();
        let mut parents: HashMap<String, String> = HashMap::new();
        let mut visited: HashSet<String> = HashSet::new();
        queue.push_back(source.to_string());
        visited.insert(source.to_string());

        // Let's implement this first with a BFS, will do random if needed
        loop {
            if let Some(current) = queue.pop_front() {
                for (next, edge) in self.mapping.get(&current).unwrap().iter() {
                    if edge.residual_capacity() > 0 && !visited.contains(next) {
                        parents.insert(next.clone(), current.clone());
                        if next == sink {
                            break
                        }
                        visited.insert(next.clone());
                        queue.push_back(next.clone());
                    }
                }
                if parents.contains_key(sink) {
                    break;
                }
            } else {
                return None
            }
        }

        let mut max_flow = usize::max_value();
        let mut vertices: Vec<String> = vec![];
        let mut current = sink.to_string();

        loop {
            vertices.push(current.clone());
            if current == source {
                break;
            }
            let parent = parents.remove(&current).unwrap();
            max_flow = std::cmp::min(
                max_flow,
                self.mapping.get(&parent).unwrap().get(&current).unwrap().residual_capacity()
                );
            current = parent;
        }
        vertices.reverse();

        Some(Path {
            vertices,
            max_flow
        })

    }

}

#[derive(Debug)]
struct Path {
    vertices: Vec<String>,
    max_flow: usize,
}
