use edge::Edge;
use nodept::{Node, NodePt};

use std::rand::Rng;
use std::collections::HashMap;
use std::io::BufferedReader;
use std::io::File;

// #[deriving(Show, Clone)]
// pub struct AdjMatrixGraph<'a> {
//     pub adj_matrix: &'a [&'a [f64]]
// }

// impl AdjMatrixGraph {
//     pub fn new<'a>(matrix: &'a [&'a f64]) {
//         AdjMatrixGraph {adj_matrix: matrix}
//     }
// }

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
    
    // Creates a complete graph from a list of NodePts (struct containing x-y coordinates and
    // a node ID.)
    pub fn from_nodes(nodes: Vec<NodePt>) -> Graph {
        let mut edges: Vec<Edge> = Vec::new();

        for a in nodes.iter() {
            for b in nodes.iter() {
                if a.id != b.id {
                    let edge = Edge::new(*a, *b);
                   // let rev = edge.reverse();
                    //if !edges.contains(&edge) && !edges.contains(&rev) {
                    edges.push(edge);
                    //}
                }
            }
        }

        let mut map: HashMap<Node, Vec<Edge>> = HashMap::new();

        for i in range(0, nodes.len()) {
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
    // Creates a random, euclidean, complete graph with a given number of nodes
    // and a scaling factor. The scaling factor affects the range of the coordinates being generated.
    pub fn random_graph<R: Rng>(rng: &mut R, num_nodes: uint, x_max: f64, y_max: f64) -> Graph {
        // Generates a list of 2-tuples of floats, adds an incrementing counter to each 
        // tuple and creates a NodePt (node with ID and 2D coordinates) from it.
        let points = rng.gen_iter::<(f64, f64)>()
            .enumerate()
            .map(|(idx, (x, y))| NodePt::new(idx, x * x_max, y * y_max))
            .take(num_nodes)
            .collect();

        Graph::from_nodes(points)
    } 
    // Returns the weight of the edge between node n and m.
    // If n == m, returns 0. (maybe return f64::INFINITY?)
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
    // Returns the edge between nodes n and m. An edge
    // has two node IDs (from, to) and a weight.
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
    // Returns a list of all edges in the graph, without duplicates
    // and sorted by their weight.
    pub fn all_edges(&self) -> Vec<Edge> {
        let mut all_edges: Vec<Edge> = Vec::new();

        for edge_list in self.adj_list.values() {
            all_edges.push_all(edge_list.as_slice());
        }
        all_edges.sort();
        all_edges.dedup();
        all_edges
    }

    pub fn node_count(&self) -> uint {
        self.adj_list.len()
    }

    pub fn get_nodes(&self) -> Vec<Node> {
        let nodes: Vec<Node> = range(0, self.node_count()).collect();
        nodes
    }

    fn read_point(string: &str) -> Option<NodePt> {
        let mut end = string.len() - 1;
        if !string.ends_with("\n") {
            end += 1;
        }

        let numbers: Vec<f64> = string
            .slice_to(end)
            .split(' ')
            .map(|x| from_str::<f64>(x))
            .filter(|f| f.is_some())
            .map(|o| o.unwrap())
            .collect();
        if numbers.len() >= 3 {
            let result = NodePt::new((*numbers.get(0) as uint) - 1, *numbers.get(1), *numbers.get(2));
            Some(result)
        }
        else {  
            None
        }
        
    }

    pub fn from_file(file_path: &str) -> Graph {
        let path = Path::new(file_path);
        let mut file = BufferedReader::new(File::open(&path));
        let node_opts: Vec<Option<NodePt>> = file.lines()
            .map(|r| match r {
                Ok(string) => Graph::read_point(string.as_slice()),
                Err(_) => fail!("failed to read")
            })
            .collect();
        let nodes = node_opts.iter()
            .filter(|x| x.is_some())
            .map(|y| y.unwrap())
            .collect();

        Graph::from_nodes(nodes)
    }
}