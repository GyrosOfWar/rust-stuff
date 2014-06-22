use std::rand::Rng;
use std::mem::swap;

use nodept::Node;
use graph::Graph;

#[deriving(Show, Clone)]
pub struct Tour {
    pub nodes: Vec<Node>,
    pub total_weight: f64
}

impl PartialEq for Tour {
    fn eq(&self, other: &Tour) -> bool {
        self.nodes == other.nodes
    }

    fn ne(&self, other: &Tour) -> bool {
        self.nodes != other.nodes
    }
}

impl PartialOrd for Tour {
    fn lt(&self, other: &Tour) -> bool {
        self.total_weight < other.total_weight
    }
}

impl Tour {
    pub fn new(nodes: Vec<Node>, weight: f64) -> Tour {
        Tour {
            nodes: nodes,
            total_weight: weight
        }
    }

    pub fn calc_tour_weight(tour: &Vec<Node>, graph: &Graph) -> f64 {
        let mut tour_weight = 0.0;
        let mut last_node = tour.get(0u);

        for node in tour.iter() {
            tour_weight += graph.get(*last_node, *node);
            last_node = node;
        }
        let last = match tour.last() {
            Some(l) => l,
            None => fail!("Empty tour!")
        };
        tour_weight += graph.get(*tour.get(0), *last);
        tour_weight
    }

    pub fn random_tour<R: Rng>(rng: &mut R, graph: &Graph) -> Tour {
        let mut tour_nodes = graph.get_nodes();
        rng.shuffle(tour_nodes.as_mut_slice());
        let tour_weight = Tour::calc_tour_weight(&tour_nodes, graph);
        Tour::new(tour_nodes, tour_weight)
    }

    pub fn mutate<R: Rng>(&self, rng: &mut R, graph: &Graph, mutation_rate: f64) -> Tour {
        let size = self.nodes.len() as f64;
        let mut mutated: Vec<Node> = self.nodes.clone();

        for i in range(0, self.nodes.len()) {
            let t = rng.gen::<f64>();
            if t < mutation_rate {
                let j = (rng.gen::<f64>() * size) as uint;
                mutated.as_mut_slice().swap(i, j);
            }
        }
        let weight = Tour::calc_tour_weight(&mutated, graph);
        Tour {
            nodes: mutated,
            total_weight: weight
        }
    }

    pub fn crossover<R: Rng>(&self, other: Tour, graph: &Graph, rng: &mut R) -> Tour {
        let size = self.nodes.len();
        let mut start = (rng.gen::<f64>() * (size as f64)) as uint;
        let mut end = (rng.gen::<f64>() * (size as f64)) as uint;
        if start == end {
            return self.clone()
        }

        if end > start {
            swap(&mut start, &mut end);
        }

        let new_tour = Vec::from_fn(size, |i| {
            if i >= start && i <= end {
                Some(*self.nodes.get(i))
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
                    let v = Some(*other.nodes.get(i));
                    if !new_tour.contains(&v) {
                        new_tour_2.push(v.unwrap());
                    }
                    i += 1;
                }
            }
        }

        let tour_weight = Tour::calc_tour_weight(&new_tour_2, graph);
        Tour::new(new_tour_2, tour_weight)
    }
}
