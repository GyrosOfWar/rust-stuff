use graph::Graph;
use tour::Tour;

use std::rand::{Rng, StdRng};
use std::fmt;

pub struct Population {
    rng: StdRng,
    graph: Box<Graph>,
    population: Vec<Tour>,
    mutation_rate: f64,
    tournament_size: usize
}

impl Population {
    // Creates a new population with a given size, on a given graph and with the given
    // GA parameters (mutation rate and tournament size).
    // The population is a list of tours, which are randomly generated.
    pub fn new(population_count: usize, graph: Box<Graph>, mutation_rate: f64, tournament_size: usize, mut rng: StdRng) -> Population {
        let population = (0..population_count).map(|_| Tour::random_tour(&mut rng, &*graph)).collect::<Vec<_>>();
        Population {
            rng: rng,
            graph: graph,
            population: population,
            mutation_rate: mutation_rate,
            tournament_size: tournament_size
        }
    }

    // Returns the fittest (lowest weighted) tour in the population. 
    pub fn fittest(&self) -> Tour {
        self.population.iter().min().unwrap().clone()
    }

    // Selects (self.tournament.size) tours randomly from the population
    // and returns the fittest one of those.
    pub fn tournament_selection(&mut self) -> Tour {
        let size = self.population.len();
        let mut buffer: Vec<Tour> = Vec::new();
        for _ in range(0, self.tournament_size) {
            let t = (self.rng.gen::<f64>() * (size as f64)) as usize;
            buffer.push(self.population[t].clone());
        }
        buffer.iter().min().unwrap().clone()

    }
    // Creates a new population based on the current one
    // by taking two parents with a tournament selection,
    // doing a crossover between them and then 
    // potentially mutating the child.
    pub fn evolve(&mut self) -> Population {
        let mut new_population: Vec<Tour> = Vec::new();

        for _ in range(0, self.population.len()) {
            let parent1 = self.tournament_selection();
            let parent2 = self.tournament_selection();
            let child: Tour = parent1.crossover(parent2, &*self.graph, &mut self.rng);
            new_population.push(child);
        }

        let mutated = new_population.iter().map(|tour| tour.mutate(&mut self.rng, &*self.graph, self.mutation_rate)).collect();
        //println!("{}\n", mutated)
        Population {
            rng: self.rng,
            graph: self.graph.clone(),
            population: mutated,
            mutation_rate: self.mutation_rate,
            tournament_size: self.tournament_size
        }
    }
}

impl fmt::Debug for Population {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Population: {:?}", self.population)
    }
}
