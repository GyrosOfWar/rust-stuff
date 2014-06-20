extern crate time;

use std::rand::{Rng, task_rng, SeedableRng, StdRng};
use std::rand::distributions::{IndependentSample, Range};
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
		let node_count = graph.size();
		let mut tour_nodes = graph.nodes();
		rng.shuffle(tour_nodes.as_mut_slice());
		let last: Node = *tour_nodes.last().unwrap();
		tour_nodes.insert(0, last);
		let tour_weight = Tour::calc_tour_weight(&tour_nodes, graph);
		Tour::new(tour_nodes, tour_weight)
	}

	fn mutate<R: Rng>(&mut self, rng: &mut R, graph: &Graph, mutationRate: f64) {
		let size = self.nodes.len() as f64;

		for i in range(0, self.nodes.len()) {
			let t = rng.gen::<f64>();
			if t < mutationRate {
				let j = (rng.gen::<f64>() * size) as uint;
				self.nodes.as_mut_slice().swap(i, j);
			}
		}

		self.total_weight = Tour::calc_tour_weight(&self.nodes, graph);
		self.fitness = 1.0 / self.total_weight;
	}

	fn crossover<R: Rng>(&self, other: Tour, graph: &Graph, rng: &mut R) -> Tour {
		let size = self.nodes.len();
		let start = (rng.gen::<f64>() * (size as f64)) as uint;
		let end = (rng.gen::<f64>() * (size as f64)) as uint;

		let mut new_tour: Vec<Node> = Vec::new();
		new_tour.grow_set(size - 1, &(std::uint::MAX), std::uint::MAX);

		for i in range(0, size) {
			if start < end && i > start && i < end {
				//new_tour[i] = self.nodes.get(i);
				println!("start {}, end {}", start, end);
				new_tour.grow_set(i, &(std::uint::MAX), *self.nodes.get(i));
			}
			else if start > end {
				if !(i < start && i > end) {
					*new_tour.get_mut(i) = *self.nodes.get(i);
				}
			}
		}

		for i in range(0, size) {
			if !new_tour.contains(other.nodes.get(i)) {
				for j in range(0, new_tour.len()) {
					if *new_tour.get(j) == std::uint::MAX {
						*new_tour.get_mut(i) = *other.nodes.get(i);
						break;
					}
				}
			}
		}

		let tour_weight = Tour::calc_tour_weight(&new_tour, graph);

		Tour::new(new_tour, tour_weight)
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
		println!("num_nodes = {}", num_nodes);

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

	fn size(&self) -> uint {
		self.adj_list.len()
	}

	fn nodes(&self) -> Vec<Node> {
		range(0, self.size()).collect()
	}

	fn from_file(file_path: &str) -> Graph {
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

struct Population {
	rng: Box<Rng>,
	graph: Box<Graph>,
	population: Vec<Tour>
}

impl Population {
	fn new(population_count: uint, graph: &Graph) -> Population {
		unimplemented!();
	}
}

fn main() {
	let mut rng: StdRng = SeedableRng::from_seed(&[1, 2, 3, 4]);
	let graph = Graph::random_graph(&mut rng, 5, 200.0, 200.0);
	let nodes: Vec<Node> = graph.adj_list.keys().map(|x| x.clone()).collect();
	println!("graph.nodes = {}", nodes);

	let tour = Tour::random_tour(&mut rng, &graph);
	println!("tour: {}", tour);
	// let tour1 = Tour::random_tour(&mut rng, &graph);
	// let tour2 = Tour::random_tour(&mut rng, &graph);
	// println!("tour 1 = {}\ntour 2 = {}", tour1, tour2);
	// let child = tour1.crossover(tour2, &graph, &mut rng);
	// println!("child  = {}", child);
}