extern crate time;

use std::rand::{Rng, task_rng};
use std::collections::HashMap;
use time::precise_time_ns;
use std::io::{File, BufferedReader, IoResult};

pub type Node = uint;

#[deriving(Show)]
pub struct NodePt {
    id: Node, 
    x: f64,
    y: f64
}

impl NodePt {
	fn new(nodeId: Node, x: f64, y: f64) -> NodePt {
		NodePt {
			id: nodeId,
			x: x,
			y: y
		}
	}

	fn distance_to(self, other: NodePt) -> f64 {
		let xx = (self.x - other.x) * (self.x - other.x);
		let yy = (self.y - other.y) * (self.y - other.y);

		(xx+yy).sqrt()
	}
}

impl PartialEq for NodePt {
	fn eq(&self, other: &NodePt) -> bool {
		self.id == other.id
	}

	fn ne(&self, other: &NodePt) -> bool {
		self.id != other.id
	}
}

#[deriving(Show, Clone)]
pub struct Edge {
	from: Node,
	to: Node,
	weight: f64
}

impl Edge {
	fn new(a: NodePt, b: NodePt) -> Edge {
		Edge {	
			from: a.id,
			to: b.id,
			weight: a.distance_to(b)
		}
	}
}

#[deriving(Show)]
pub struct Tour {
    nodes: Vec<Node>,
    total_weight: f64,
    fitness: f64
}

impl Tour {
	fn new(nodes: Vec<Node>, weight: f64) -> Tour {
		Tour {
			nodes: nodes,
			total_weight: weight,
			fitness: 1.0 / weight
		}
	}

	fn calc_tour_weight(tour: &Vec<Node>, graph: &Graph) -> f64 {
		let mut tour_weight = 0.0;
		let mut last_node = tour.get(0u);
		for node in tour.iter() {
			tour_weight += graph.get(*last_node, *node);
			last_node = node;
		}

		tour_weight
	}

	fn random_tour<R: Rng>(rng: &mut R, graph: &Graph) -> Tour {
		let node_count = graph.adj_list.len();
		let mut tour_nodes: Vec<Node> = range(0, node_count).collect();
		rng.shuffle(tour_nodes.as_mut_slice());
		let last: Node = *tour_nodes.last().unwrap();
		tour_nodes.insert(0, last);
		let tour_weight = Tour::calc_tour_weight(&tour_nodes, graph);
		Tour::new(tour_nodes, tour_weight)
	}

}

#[deriving(Show)]
pub struct Graph {
	adj_list: HashMap<Node, Vec<Edge>>
}

impl Graph {
	fn new(adj_list: HashMap<Node, Vec<Edge>>) -> Graph {
		Graph {
			adj_list: adj_list
		}
	}

	fn from_nodes(nodes: Vec<NodePt>) -> Graph {
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

	fn random_graph<R: Rng>(rng: &mut R, num_nodes: uint, x_max: f64, y_max: f64) -> Graph {
		let points = rng.gen_iter::<(f64, f64)>()
			.enumerate()
			.map(|(idx, (x, y))| NodePt::new(idx, x * x_max, y * y_max))
			.take(num_nodes)
			.collect();

		Graph::from_nodes(points)
	} 

	fn get(&self, n: Node, m: Node) -> f64 {
		if n == m {
			0.0
		}
		else {
			let edges = self.adj_list.get(&n);
			let result = edges.iter().filter(|edge| edge.to == m).nth(0);
			match result {
				Some(edge) => edge.weight,
				None => 0.0
			}
		}
	}

	fn from_file(file_path: &str) -> Graph {
		let path = Path::new(file_path);
		let mut file = BufferedReader::new(File::open(&path));
		let mut nodes: Vec<NodePt> = Vec::new();
		let mut i = 0u;

		for line in file.lines() {
			// let val: Vec<f64> = line.map(|l| {
			// 	l.as_slice().split(' ').map(|i| from_str::<f64>(i)).collect()
			// }).unwrap();

			// let nodePt = NodePt::new(i, *val.get(0), *val.get(1));
			line.map(|s| s.as_slice());
			i += 1;
		}

		Graph::from_nodes(nodes)
	}
}

fn main() {
	let test = "20 14";
	let split: Vec<f64> = test.split(' ').map(|i| from_str::<f64>(i).unwrap_or(0.0)).collect();
	println!("{}", NodePt::new(0, *split.get(0), *split.get(1)))

	let mut rng = task_rng();
	let graph = Graph::random_graph(&mut rng, 10, 200.0, 200.0);
}