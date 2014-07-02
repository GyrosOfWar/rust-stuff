use graph::Graph;
use tour::Tour;
use simulated_annealing::TSPAlgorithm;
use nodept::Node;

use std::rand::{Rng, StdRng};
use std::fmt;
use std::mem::swap;

pub struct GeneticAlgorithm {
    population: Vec<Tour>,
    iterations: uint,
    mutation_rate: f64,
    population_size: uint,
    tournament_size: uint
}

#[deriving(Show)]
pub struct GAParameterSet {
    pub iterations: uint,
    pub mutation_rate: f64,
    pub population_size: uint,
    pub tournament_size: uint
}

impl TSPAlgorithm for GeneticAlgorithm {
    fn search<R: Rng>(&mut self, graph: &Graph, rng: &mut R) -> Tour {
        for i in range(0, self.iterations) {
            self.evolve(rng, graph);
        }

        find_min(&self.population)
    }
}

impl GeneticAlgorithm {
    pub fn new<R: Rng>(rng: &mut R, graph: &Graph, params: &GAParameterSet) -> GeneticAlgorithm {
        let population = Vec::from_fn(params.population_size, |_| Tour::random_tour(rng, graph));
        GeneticAlgorithm {
            population: population,
            population_size: params.population_size,
            iterations: params.iterations,
            mutation_rate: params.mutation_rate,
            tournament_size: params.tournament_size
        }
    }

    fn mutate_one<R: Rng>(&self, tour: &Tour, rng: &mut R, graph: &Graph) -> Tour {
        let size = tour.nodes.len();
        let mut mutated: Vec<Node> = tour.nodes.clone();

        for i in range(0, size) {
            let t = rng.gen::<f64>();
            if t < self.mutation_rate {
                let j = rng.gen_range::<uint>(0, size);
                mutated.as_mut_slice().swap(i, j);
            }
        }
        let weight = Tour::calc_tour_weight(&mutated, graph);
        Tour {
            nodes: mutated,
            total_weight: weight
        }
    }

    fn crossover<R: Rng>(&self, first: &Tour, second: &Tour, graph: &Graph, rng: &mut R) -> Tour {
        let size = first.nodes.len();

        let mut start = rng.gen_range::<uint>(0, size);
        let mut end = rng.gen_range::<uint>(0, size);
        if start == end {
            return first.clone();
        }

        if end > start {
            swap(&mut start, &mut end);
        }

        let new_tour = Vec::from_fn(size, |i| {
            if i >= start && i <= end {
                Some(*first.nodes.get(i))
            } else {
                None
            }
        });
        
        let mut i = 0;
        let mut new_tour_2: Vec<Node> = Vec::new();
        for node in new_tour.iter() {
            match *node {
                Some(n) => new_tour_2.push(n),
                None => { 
                    let v = Some(*second.nodes.get(i));
                    if !new_tour.contains(&v) {
                        new_tour_2.push(v.unwrap());
                    }
                    i += 1;
                }
            }
        }
        Tour::new(new_tour_2, graph)
    }

    fn tournament_selection<R: Rng>(&self, rng: &mut R) -> Tour {
        let size = self.population.len();
        let mut buffer: Vec<Tour> = Vec::new();
        for _ in range(0, self.tournament_size) {
            //let t = (self.rng.gen::<f64>() * (size as f64)) as uint;
            let t = rng.gen_range::<uint>(0, size);
            buffer.push(self.population.get(t).clone());
        }
        find_min(&buffer)
    }

    fn evolve<R: Rng>(&mut self, rng: &mut R, graph: &Graph) {
        for i in range(0, self.population.len()) {
            let first = self.tournament_selection(rng);
            let second = self.tournament_selection(rng);
            let child: Tour = self.crossover(&first, &second, graph, rng);

            *self.population.get_mut(i) = self.mutate_one(&child, rng, graph);
        }
    }
}


pub struct Population {
    rng: StdRng,
    graph: Graph,
    population: Vec<Tour>,
    mutation_rate: f64,
    tournament_size: uint
}

impl Population {
    // Creates a new population with a given size, on a given graph and with the given
    // GA parameters (mutation rate and tournament size).
    // The population is a list of tours, which are randomly generated.
    pub fn new(population_count: uint, graph: Graph, mutation_rate: f64, tournament_size: uint, mut rng: StdRng) -> Population {
        let population = Vec::from_fn(population_count, |_| Tour::random_tour(&mut rng, &graph));

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
        find_min(&self.population)
    }

    // Selects (self.tournament.size) tours randomly from the population
    // and returns the fittest one of those.
    pub fn tournament_selection(&mut self) -> Tour {
        let size = self.population.len();
        let mut buffer: Vec<Tour> = Vec::new();
        for _ in range(0, self.tournament_size) {
            let t = (self.rng.gen::<f64>() * (size as f64)) as uint;
            buffer.push(self.population.get(t).clone());
        }
        find_min(&buffer)
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
            let child: Tour = parent1.crossover(parent2, &self.graph, &mut self.rng);
            new_population.push(child);
        }

        let mutated = new_population.iter().map(|tour| tour.mutate(&mut self.rng, &self.graph, self.mutation_rate)).collect();
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

impl fmt::Show for Population {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Population: {}", self.population)
    }
}
// Utility function for finding the minimum in a list of 
// values that only implement PartialOrd and not Ord.
pub fn find_min<E: PartialOrd+Clone>(xs: &Vec<E>) -> E {
    let ref min = *xs.iter().fold(xs.get(0), |min, next| if next < min {next} else {min});
    min.clone()
}

pub fn find_max<E: PartialOrd+Clone>(xs: &Vec<E>) -> E {
    let ref max = *xs.iter().fold(xs.get(0), |max, next| if next > max {next} else {max});
    max.clone()
}