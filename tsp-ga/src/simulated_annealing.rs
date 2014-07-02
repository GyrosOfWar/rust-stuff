use graph::Graph;
use nodept::{NodePt, Node};
use std::rand::{Rng, StdRng};
use tour::Tour;

pub trait TSPAlgorithm {
    fn search<R: Rng>(&mut self, graph: &Graph, rng: &mut R) -> Tour;
}

pub struct SimulatedAnnealing {
	initial_solution: Tour,
	initial_temperature: f64,
	cooling_rate: f64
}

impl TSPAlgorithm for SimulatedAnnealing {
	fn search<R: Rng>(&mut self, graph: &Graph, rng: &mut R) -> Tour{
		let mut t: uint = 0;
		let mut solution = self.initial_solution.clone();
		let mut temperature = self.initial_temperature;

		while temperature > 0.001 {
			let neighbor = self.neighbor(&solution, graph, rng);
			let neighbor_weight = neighbor.total_weight;
			let solution_weight = solution.total_weight;

			let z = rng.gen::<f64>();
			let p = SimulatedAnnealing::acceptance_probability(solution_weight, neighbor_weight, temperature);
			if z < p {
				solution = neighbor;
			}

			temperature *= self.cooling_rate;
			t += 1;
		}

		solution
	}
}

impl SimulatedAnnealing {
	pub fn new(initial_solution: Tour, initial_temperature: f64, cooling_rate: f64) -> SimulatedAnnealing {
		SimulatedAnnealing {
	        initial_solution: initial_solution,
	        initial_temperature: initial_temperature,
	        cooling_rate: cooling_rate
	    }
	}

	fn neighbor<R: Rng>(&self, tour: &Tour, graph: &Graph, rng: &mut R) -> Tour {
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
}
