use std::rand::{Rng, task_rng};
use std::collections::HashMap;
use std::iter;

type Node = uint;

#[deriving(Show)]
struct NodePt {
    id: Node, 
    x: f64,
    y: f64
}

#[deriving(Show, Clone)]
struct Edge {
	from: Node,
	to: Node,
	weight: f64
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
struct Tour {
    nodes: Vec<Node>,
    total_weight: f64
}

impl Tour {
	fn new(nodes: Vec<Node>, weight: f64) -> Tour {
		Tour {
			nodes: nodes,
			total_weight: weight
		}
	}
}

fn make_complete_graph(nodes: Vec<NodePt>) -> Vec<Edge> {
	let mut edges: Vec<Edge> = Vec::new();

	for x in nodes.iter() {
		for y in nodes.iter() {
			if x.id != y.id {
				let e = Edge::new(*x, *y);
				edges.push(e);
			}
		}
	}

	edges
}

fn random_nodes(n: uint, x_max: f64, y_max: f64) -> Vec<NodePt> {
	let mut rng1 = task_rng();
	let mut rng2 = task_rng();
	let rngIter1 = rng1.gen_iter::<f64>();
	let rngIter2 = rng2.gen_iter::<f64>();

	rngIter1.zip(rngIter2)
		.enumerate()
		.map(|t| 
			match t { (idx, (x, y)) => NodePt::new(idx, x * x_max, y * y_max) })
		.take(n)
		.collect()
}

fn make_adjacency_map(edges: Vec<Edge>, node_count: uint) -> HashMap<Node, Vec<Edge>> {
	let mut map: HashMap<Node, Vec<Edge>> = HashMap::new();

	for i in range(0, node_count) {
		let adj_edges = edges.iter()
			.map(|x| x.clone())
			.filter(|edge| edge.from == i)
			.collect();
		map.insert(i, adj_edges);
	}

	map
}



fn calc_tour_weight(tour_pts: &Vec<NodePt>) -> f64 {
	let mut sum = 0.0;
	let mut last = tour_pts.get(0);

	for point in tour_pts.iter() {
		sum += last.distance_to(*point);
		last = point;
	}
	if (tour_pts.get(0) != last) {
		sum += last.distance_to(*tour_pts.get(0));
	}

	sum
}

fn random_tour(map: &HashMap<Node, NodePt>, node_count: uint) -> Tour {
	let mut rng = task_rng();
	let mut tour_nodes: Vec<uint> = range(0, node_count).collect();
	rng.shuffle(tour_nodes.as_mut_slice());

	let mut tour_pts: Vec<NodePt> = Vec::new();
	for n in tour_nodes.iter() {
		tour_pts.push(*map.get(n));
	}

	let tour_weight = calc_tour_weight(&tour_pts);
	Tour::new(tour_nodes, tour_weight)
}

fn main() {
	let N = 12;
	let nodes = random_nodes(N, 300.0, 300.0);
	let mut map: HashMap<Node, NodePt> = HashMap::new();

	for n in nodes.iter() {
		map.insert(n.id, *n);
	}

	for i in range(0, 100000) {
		let tour = random_tour(&map, N);
	}
	// let graph = make_complete_graph(nodes);
	// let map = make_adjacency_map(graph, N);
	// let adj_edges = map.get(&2);

	//println!("{}", tour);
}