use edge::Edge;
use nodept::{Node, NodePt};
use population::find_max;

use std::rand::Rng;
use std::io::BufferedReader;
use std::io::File;
use std::f64::INFINITY;
use std::fmt;
use std::collections::HashMap;

#[deriving(Clone)]
pub struct Graph {
    pub num_nodes: uint,
    scaled_node_map: HashMap<Node, NodePt>,
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

    pub fn from_nodes(nodes: Vec<NodePt>, max: f64) -> Graph {
        let num_nodes = nodes.len();
        let size = num_nodes * num_nodes;
        let mut matrix: Vec<f64> = Vec::with_capacity(size);
        let mut node_map: HashMap<Node, NodePt> = HashMap::new();
        matrix.grow_set(size - 1, &INFINITY, INFINITY);

        let mut x_max = -1.0;
        let mut y_max = -1.0;

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

            x_max = if a.x > x_max { a.x } else { x_max };
            y_max = if a.y > y_max { a.y } else { y_max };
            node_map.insert(a.id, *a);
        }

        for (id, node) in node_map.mut_iter() {
            node.x = Graph::scale_to_range(node.x, 0.0, max, 0.0, x_max);
            node.y = Graph::scale_to_range(node.y, 0.0, max, 0.0, y_max);
        }

        Graph {
            adj_matrix: matrix, 
            num_nodes: num_nodes,
            scaled_node_map: node_map
        }
    }

    fn scale_to_range(x: f64, a: f64, b: f64, min: f64, max: f64) -> f64 {
        (((b - a) * (x - min)) / (max - min)) + a
    }

    pub fn random_graph<R: Rng>(rng: &mut R, num_nodes: uint, x_max: f64, y_max: f64) -> Graph {
        // Generates a list of 2-tuples of floats, adds an incrementing counter to each 
        // tuple and creates a NodePt (node with ID and 2D coordinates) from it.
        let points = rng.gen_iter::<(f64, f64)>()
            .enumerate()
            .map(|(idx, (x, y))| NodePt::new(idx, x * x_max, y * y_max))
            .take(num_nodes)
            .collect();

        Graph::from_nodes(points, x_max)
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

    pub fn get_node_points(&self) -> Vec<NodePt> {
        self.scaled_node_map.values().map(|x| *x).collect()
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

    pub fn from_file(file_path: &str, scale: f64) -> Graph {
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

        Graph::from_nodes(nodes, scale)
    }

    pub fn tour_to_points(&self, tour: &Vec<Node>) -> Vec<NodePt> {
        let mut points: Vec<NodePt> = Vec::new();

        for id in tour.iter() {
            let node = self.scaled_node_map.get(id);
            points.push(*node);
        }
        points
    }
}