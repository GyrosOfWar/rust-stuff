use graph::Graph;
use tour::Tour;
use edge::Edge;
use std::f64::INFINITY;

struct UnionFind<'a> {
	id: &'a [uint],
	count: uint
}

impl<'a> UnionFind<'a> {
	fn new<'a>(length: uint) -> UnionFind<'a> {
		unimplemented!()
	}

	fn union(&mut self, p: uint, q: uint) {
		unimplemented!()
	}

	fn connected(&self, p: uint, q: uint) -> bool {
		unimplemented!()
	}
}

struct Problem<'a> {
	graph: &'a Graph,
	included_edges: Vec<Edge>,
	excluded_edges: Vec<Edge>,
	lower_bound: f64
}

impl<'a> Problem<'a> {
	fn new<'a>(graph: &'a Graph) -> Problem<'a> {
		unimplemented!()
	}
}

fn mst_tour(problem: &Problem) -> Tour {
	unimplemented!()
}

pub fn branch_and_bound(graph: &Graph) -> Tour {
	let mut upper_bound = INFINITY;
	let mut problems = vec!(Problem::new(graph));

	while problems.len() > 0 {
		let P = problems.pop().unwrap();
		let lower_bound = P.lower_bound;
		if lower_bound < upper_bound {
			let tour = mst_tour(&P);

		}
	}

	unimplemented!()
}