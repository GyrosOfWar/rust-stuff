use edge::Edge;
use nodept::{Node, NodePt};

use std::rand::Rng;
use std::collections::HashMap;
use std::io::BufferedReader;
use std::io::File;

#[deriving(Show, Clone)]
pub struct Graph {
    pub adj_list: HashMap<Node, Vec<Edge>>
}

impl Graph {
    pub fn new(adj_list: HashMap<Node, Vec<Edge>>) -> Graph {
        Graph {
            adj_list: adj_list
        }
    }

    pub fn from_edges(edges: Vec<Edge>, node_count: uint) -> Graph {
        let mut map: HashMap<Node, Vec<Edge>> = HashMap::new();

        for i in range(0, node_count) {
            //let adj_edges: Vec<Edge> = edges.iter().filter(|e| e.from == i).collect();
            let mut adj_edges: Vec<Edge> = Vec::new();
            for edge in edges.iter() {
                if edge.from == i {
                    adj_edges.push(*edge);
                }
            }
            map.insert(i, adj_edges);
        }

        Graph::new(map)
    }

    pub fn from_nodes(nodes: Vec<NodePt>) -> Graph {
        let mut map: HashMap<Node, Vec<Edge>> = HashMap::new();
        for i in range(0, nodes.len()) {
            let mut adj_edges: Vec<Edge> = Vec::new();

            for j in range(0, nodes.len()) {
                let a = nodes.get(i);
                let b = nodes.get(j);
                if i != j {
                    adj_edges.push(Edge::new(*a, *b));
                }
            }

            map.insert(i, adj_edges);
        }

        Graph::new(map)
    }

    pub fn random_graph<R: Rng>(rng: &mut R, num_nodes: uint, x_max: f64, y_max: f64) -> Graph {
        let points = rng.gen_iter::<(f64, f64)>()
            .enumerate()
            .map(|(idx, (x, y))| NodePt::new(idx, x * x_max, y * y_max))
            .take(num_nodes)
            .collect();

        Graph::from_nodes(points)
    } 

    pub fn get(&self, n: Node, m: Node) -> f64 {
        if n == m {
            0.0
        } else {
            let edges = self.adj_list.get(&n);
            let result = edges.iter().filter(|edge| edge.to == m).nth(0);
            match result {
                Some(edge) => edge.weight,
                None => fail!("No edge found!")
            }
        }
    }

    pub fn get_edge(&self, n: Node, m: Node) -> Edge {
        if n == m {
            fail!("No edge from {} to {}!", n, m);
        }
        let edges = self.adj_list.get(&n);
        let result = edges.iter().filter(|edge| edge.to == m).nth(0);
        match result {
            Some(edge) => *edge,
            None => fail!("No edge from {} to {}!", n, m)
        }
    }

    pub fn all_edges(&self) -> Vec<Edge> {
        let mut all_edges: Vec<Edge> = Vec::new();

        for edge_list in self.adj_list.values() {
            all_edges.push_all(edge_list.as_slice());
        }
        all_edges.sort();
        all_edges.dedup();

        all_edges
    }

    pub fn size(&self) -> uint {
        self.adj_list.len()
    }

    pub fn get_nodes(&self) -> Vec<Node> {
        let nodes: Vec<Node> = range(0, self.size()).collect();
        nodes
    }

    #[allow(dead_code)]
    pub fn from_file(file_path: &str) -> Graph {
        let path = Path::new(file_path);
        let mut file = BufferedReader::new(File::open(&path));
        let mut nodes: Vec<NodePt> = Vec::new();
        let mut i = 0u;

        for line in file.lines() {
            let mut numbers: Vec<f64> = Vec::new();
            // This could be a lot nicer
            match line {
                Ok(val) => {
                    for token in val.as_slice().split(' ') {
                        if token == "\n" {
                            continue
                        }

                        let n = match from_str::<f64>(token) {
                            Some(num) => num,
                            None => continue
                        };
                        numbers.push(n);
                    }
                },
                Err(_) => fail!("Failed reading file.")
            }

            if numbers.len() == 2 {
                let point = NodePt::new(i, *numbers.get(0), *numbers.get(1));
                nodes.push(point);
            }

            i += 1;
        }

        Graph::from_nodes(nodes)
    }
}