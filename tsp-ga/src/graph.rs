use edge::Edge;
use nodept::{Node, NodePt};

use std::rand::Rng;
use std::io::BufferedReader;
use std::io::File;
use std::f64::INFINITY;
use std::fmt;

#[deriving(Clone)]
pub struct Graph {
    pub num_nodes: uint,
    adj_matrix: Vec<f64>
}

impl fmt::Show for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();

        for i in range(0, self.num_nodes) {
            for j in range(0, self.num_nodes) {
                let w = self.get(i, j);
                if w == INFINITY {
                    s.push_str("∞\t");
                }
                else {
                    s.push_str(w.to_str().append("\t").as_slice());
                }
            }
            s.push_str("\n");
        }

        write!(f, "{}", s)
    }
}

impl Graph {
    #[inline]
    fn offset(i: uint, j: uint, n_nodes: uint) -> uint {
        i + j * n_nodes
    }

    pub fn from_nodes(nodes: Vec<NodePt>) -> Graph {
        let num_nodes = nodes.len();
        let size = num_nodes * num_nodes;
        let mut matrix: Vec<f64> = Vec::with_capacity(size);
        matrix.grow_set(size - 1, &INFINITY, INFINITY);

        for a in nodes.iter() {
            for b in nodes.iter() {
                let aId = a.id;
                let bId = b.id;
                let offset = Graph::offset(aId, bId, num_nodes);
                if aId == bId {
                    *matrix.get_mut(offset) = INFINITY;
                } else {
                    *matrix.get_mut(offset) = a.distance_to(*b);
                }

            }
        }

        Graph {
            adj_matrix: matrix, 
            num_nodes: num_nodes
        }
    }

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

    #[inline]
    pub fn get(&self, n: Node, m: Node) -> f64 {
        let offset = Graph::offset(n, m, self.num_nodes);
        let weight = *self.adj_matrix.get(offset);
        weight
    }

    #[inline]
    pub fn get_edge(&self, n: Node, m: Node) -> Edge {
        Edge {from: n, to: m, weight: self.get(n, m)}
    }

    pub fn all_edges(&self) -> Vec<Edge> {
        let mut edges: Vec<Edge> = Vec::new();
        let n = self.num_nodes;

        for i in range(0, n) {
            for j in range(i, n) {
                let edge = self.get_edge(i, j);
                if edge.weight != INFINITY {
                    edges.push(edge);
                }
            }
        }

        edges
    }

    pub fn get_nodes(&self) -> Vec<Node> {
        let nodes: Vec<Node> = range(0, self.num_nodes).collect();
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