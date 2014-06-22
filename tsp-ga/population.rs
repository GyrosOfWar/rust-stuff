use graph::Graph;
use tour::Tour;

use std::rand::{Rng, StdRng};
use std::fmt;

pub struct Population {
    rng: StdRng,
    graph: Box<Graph>,
    population: Vec<Tour>,
    mutation_rate: f64,
    tournament_size: uint
}

impl Population {
    pub fn new(population_count: uint, graph: Box<Graph>, mutation_rate: f64, tournament_size: uint, mut rng: StdRng) -> Population {
        let population = Vec::from_fn(population_count, |_| Tour::random_tour(&mut rng, graph));

        Population {
            rng: rng,
            graph: graph,
            population: population,
            mutation_rate: mutation_rate,
            tournament_size: tournament_size
        }
    }

    pub fn fittest(&self) -> Tour {
        find_min(&self.population)
    }

    pub fn tournament_selection(&mut self) -> Tour {
        let size = self.population.len();
        let mut buffer: Vec<Tour> = Vec::new();
        for _ in range(0, self.tournament_size) {
            let t = (self.rng.gen::<f64>() * (size as f64)) as uint;
            buffer.push(self.population.get(t).clone());
        }
        find_min(&buffer)
    }

    pub fn evolve(&mut self) -> Population {
        let mut new_population: Vec<Tour> = Vec::new();

        for _ in range(0, self.population.len()) {
            let parent1 = self.tournament_selection();
            let parent2 = self.tournament_selection();
            let child: Tour = parent1.crossover(parent2, self.graph, &mut self.rng);
            new_population.push(child);
        }

        let mutated = new_population.iter().map(|tour| tour.mutate(&mut self.rng, self.graph, self.mutation_rate)).collect();

        Population {
            rng: self.rng,
            graph: self.graph.clone(),
            population: mutated,
            mutation_rate: self.mutation_rate,
            tournament_size: self.tournament_size
        }
    }
}

impl fmt::Show for Population {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Population: {}", self.population)
    }
}

fn find_min<E: PartialOrd+Clone>(xs: &Vec<E>) -> E {
    let ref min = *xs.iter().fold(xs.get(0), |min, next| if next < min {next} else {min});
    min.clone()
}