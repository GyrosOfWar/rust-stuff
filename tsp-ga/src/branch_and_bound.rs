use graph::Graph;
use tour::Tour;
use edge::Edge;
use std::f64::INFINITY;

struct Problem<'a> {
	graph: &'a Graph,
	included_edges: Vec<Edge>,
	excluded_edges: Vec<Edge>,
	lower_bound: f64
}

impl<'a> Problem<'a> {
	fn new<'a>(graph: &'a Graph) -> Problem<'a> {
		Problem {
			graph: graph,
			included_edges: Vec::new(),
			excluded_edges: Vec::new(),
			lower_bound: -1.0
		}
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