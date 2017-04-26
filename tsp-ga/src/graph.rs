use edge::Edge;
use nodept::{Node, NodePt};

use rand::Rng;
use std::f64::INFINITY;
use std::fmt;
use std::iter::repeat;
use std::path::Path;
use std::io::{self, BufReader, BufRead};
use std::fs::File;

#[derive(Clone)]
pub struct Graph {
    pub num_nodes: usize,
    adj_matrix: Vec<f64>,
}

impl fmt::Debug for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();

        for i in 0..self.num_nodes {
            for j in 0..self.num_nodes {
                let w = self.get(i, j);
                if w == INFINITY {
                    s.push_str("∞\t");
                } else {
                    s.push_str(&format!("{}\t", w));
                }
            }
            s.push_str("\n");
        }

        write!(f, "{}", s)
    }
}

impl Graph {
    #[inline]
    fn offset(i: usize, j: usize, n_nodes: usize) -> usize {
        i + j * n_nodes
    }

    pub fn from_nodes(nodes: Vec<NodePt>) -> Graph {
        let num_nodes = nodes.len();
        let size = num_nodes * num_nodes;
        let mut matrix: Vec<f64> = repeat(INFINITY).take(size).collect();
        for a in nodes.iter() {
            for b in nodes.iter() {
                let a_id = a.id;
                let b_id = b.id;
                let offset = Graph::offset(a_id, b_id, num_nodes);
                if a_id == b_id {
                    matrix[offset] = INFINITY;
                } else {
                    matrix[offset] = a.distance_to(*b);
                }

            }
        }

        Graph {
            adj_matrix: matrix,
            num_nodes: num_nodes,
        }
    }

    pub fn random_graph<R: Rng>(rng: &mut R, num_nodes: usize, x_max: f64, y_max: f64) -> Graph {
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
        let weight = self.adj_matrix[offset];
        weight
    }

    #[inline]
    pub fn get_edge(&self, n: Node, m: Node) -> Edge {
        Edge {
            from: n,
            to: m,
            weight: self.get(n, m),
        }
    }

    pub fn all_edges(&self) -> Vec<Edge> {
        let mut edges: Vec<Edge> = Vec::new();
        let n = self.num_nodes;

        for i in 0..n {
            for j in i..n {
                let edge = self.get_edge(i, j);
                if edge.weight != INFINITY {
                    edges.push(edge);
                }
            }
        }

        edges
    }

    pub fn get_nodes(&self) -> Vec<Node> {
        (0..self.num_nodes).collect()
    }

    fn read_point(string: &str) -> Option<NodePt> {
        let mut end = string.len() - 1;
        if !string.ends_with("\n") {
            end += 1;
        }

        let numbers: Vec<f64> = string[..end]
            .split(' ')
            .map(|x| x.parse::<f64>())
            .filter(|f| f.is_ok())
            .map(|o| o.unwrap())
            .collect();
        if numbers.len() >= 3 {
            let result = NodePt::new(numbers[0] as usize - 1, numbers[1], numbers[2]);
            Some(result)
        } else {
            None
        }

    }

    pub fn from_file(file_path: &str) -> io::Result<Graph> {
        let path = Path::new(file_path);
        let file = BufReader::new(File::open(&path)?);
        let node_opts: Vec<Option<NodePt>> = file.lines()
            .map(|r| match r {
                Ok(string) => Graph::read_point(&string),
                Err(_) => panic!("failed to read"),
            })
            .collect();
        let nodes = node_opts.iter()
            .filter(|x| x.is_some())
            .map(|y| y.unwrap())
            .collect();

        Ok(Graph::from_nodes(nodes))
    }
}
