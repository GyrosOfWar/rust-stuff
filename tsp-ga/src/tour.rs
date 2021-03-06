use rand::Rng;
use std::mem::swap;
use std::cmp::Ordering;

use nodept::Node;
use graph::Graph;
use edge::Edge;

#[derive(Debug, Clone)]
pub struct Tour {
    pub nodes: Vec<Node>,
    pub total_weight: f64,
}

impl PartialEq for Tour {
    fn eq(&self, other: &Tour) -> bool {
        self.nodes == other.nodes
    }

    fn ne(&self, other: &Tour) -> bool {
        self.nodes != other.nodes
    }
}

impl Eq for Tour {}

impl PartialOrd for Tour {
    fn partial_cmp(&self, other: &Tour) -> Option<Ordering> {
        self.total_weight.partial_cmp(&other.total_weight)
    }
}

impl Ord for Tour {
    fn cmp(&self, other: &Tour) -> Ordering {
        match self.total_weight.partial_cmp(&other.total_weight) {
            Some(ord) => ord,
            None => Ordering::Greater,
        }
    }
}

impl Tour {
    pub fn new(nodes: Vec<Node>, weight: f64) -> Tour {
        Tour {
            nodes: nodes,
            total_weight: weight,
        }
    }
    // Calculates a tour's weight by summing up all the edge weights
    // between the nodes.
    pub fn calc_tour_weight(tour: &Vec<Node>, graph: &Graph) -> f64 {
        let mut tour_weight = 0.0;
        let mut last_node = tour[0];

        for idx in 1..tour.len() {
            let node = tour[idx];
            let x = graph.get(last_node, node);
            tour_weight += x;
            last_node = node;
        }
        let last = match tour.last() {
            Some(l) => l,
            None => panic!("Empty tour!"),
        };
        tour_weight += graph.get(tour[0], *last);
        tour_weight
    }
    // Creates a random tour by taking a range of nodes [0..tour_len[
    // and shuffling it.
    pub fn random_tour<R: Rng>(rng: &mut R, graph: &Graph) -> Tour {
        let mut tour_nodes = graph.get_nodes();
        rng.shuffle(tour_nodes.as_mut_slice());
        let tour_weight = Tour::calc_tour_weight(&tour_nodes, graph);
        Tour::new(tour_nodes, tour_weight)
    }

    // Mutation works by looping over the tour and exchanging two
    // random values in the tour.
    pub fn mutate<R: Rng>(&self, rng: &mut R, graph: &Graph, mutation_rate: f64) -> Tour {
        let size = self.nodes.len() as f64;
        let mut mutated: Vec<Node> = self.nodes.clone();

        for i in 0..self.nodes.len() {
            let t = rng.gen::<f64>();
            if t < mutation_rate {
                let j = (rng.gen::<f64>() * size) as usize;
                mutated.as_mut_slice().swap(i, j);
            }
        }
        let weight = Tour::calc_tour_weight(&mutated, graph);
        Tour {
            nodes: mutated,
            total_weight: weight,
        }
    }
    // Crossover takes two tours (parents) and returns their child.
    // This takes a random start and end value, copies that range of
    // values over to the new tour from the first parent
    // and then copies missing values to the child in the second
    // parent's order.
    pub fn crossover<R: Rng>(&self, other: Tour, graph: &Graph, rng: &mut R) -> Tour {
        let size = self.nodes.len();

        let mut start = rng.gen_range::<usize>(0, size);
        let mut end = rng.gen_range::<usize>(0, size);
        if start == end {
            return self.clone();
        }

        if end > start {
            swap(&mut start, &mut end);
        }

        let new_tour = (0..size)
            .map(|i| {
                if i >= start && i <= end {
                    Some(self.nodes[0])
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        let mut i = 0;
        let mut new_tour_2: Vec<Node> = Vec::new();
        for node in new_tour.iter() {
            match *node {
                Some(n) => new_tour_2.push(n),
                None => {
                    let v = Some(other.nodes[i]);
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

    pub fn to_edges(&self, graph: &Graph) -> Vec<Edge> {
        let mut edges: Vec<Edge> = Vec::new();
        let mut last_node = self.nodes[0];
        for i in 1..self.nodes.len() {
            let next_node = self.nodes[i];
            let edge = graph.get_edge(last_node, next_node);
            edges.push(edge);
            last_node = next_node;
        }

        edges
    }
}
