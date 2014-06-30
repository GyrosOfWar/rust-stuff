use graph::Graph;
use nodept::{NodePt, Node};
use std::rand::{Rng, StdRng};
use tour::Tour;

struct SimulatedAnnealing {
	graph: Graph,
	initial_temperature: f64,
	cooling_rate: f64,
	rng: StdRng
}

// TODO impl TSPAlgorithm for SimulatedAnnealing

fn neighbor<R: Rng>(tour: &Tour, rng: &mut R, graph: &Graph) -> Tour {
	let m = rng.gen_range::<uint>(0, tour.nodes.len());
	let n = rng.gen_range::<uint>(0, tour.nodes.len());
	tour.swap_nodes(n, m, graph)
}

fn acceptance_probability(solution_weight: f64, neighbor_weight: f64, temperature: f64) -> f64 {
	if neighbor_weight < solution_weight {
		1.0
	} else {
		((solution_weight - neighbor_weight) / temperature).exp()
	}
}

pub fn simulated_annealing<R: Rng>(graph: &Graph, initial_temperature: f64, initial_solution: Tour, cooling_rate: f64, rng: &mut R) -> Tour {
	let mut t = 0;
	let mut solution = initial_solution;
	let mut temperature = initial_temperature;

	while temperature > 0.001 {
		let neighbor = neighbor(&solution, rng, graph);
		let neighbor_weight = neighbor.total_weight;
		let solution_weight = solution.total_weight;

		let z = rng.gen::<f64>();
		let p = acceptance_probability(solution_weight, neighbor_weight, temperature);
		if z < p {
			solution = neighbor;
		}

		temperature *= cooling_rate;
		t += 1;
	}

	solution
}