extern crate time;

use std::rand::{Rng, SeedableRng, StdRng};
use std::collections::HashMap;
use time::precise_time_ns;
use std::io::{File, BufferedReader};
use std::fmt;

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

#[deriving(Show, Clone)]
pub struct Tour {
    nodes: Vec<Node>,
    total_weight: f64
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
    fn new(nodes: Vec<Node>, weight: f64) -> Tour {
        Tour {
            nodes: nodes,
            total_weight: weight
        }
    }

    fn calc_tour_weight(tour: &Vec<Node>, graph: &Graph) -> f64 {
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

    fn random_tour<R: Rng>(rng: &mut R, graph: &Graph) -> Tour {
        let mut tour_nodes = graph.nodes();
        rng.shuffle(tour_nodes.as_mut_slice());
        let tour_weight = Tour::calc_tour_weight(&tour_nodes, graph);
        Tour::new(tour_nodes, tour_weight)
    }

    fn mutate<R: Rng>(&self, rng: &mut R, graph: &Graph, mutation_rate: f64) -> Tour {
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

    fn crossover<R: Rng>(&self, other: Tour, graph: &Graph, rng: &mut R) -> Tour {
        let size = self.nodes.len();
        let mut start = (rng.gen::<f64>() * (size as f64)) as uint;
        let mut end = (rng.gen::<f64>() * (size as f64)) as uint;
        if start == end {
            return self.clone()
        }

        if end > start {
            std::mem::swap(&mut start, &mut end);
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

#[deriving(Show, Clone)]
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

        Graph::from_nodes(points)
    } 

    fn get(&self, n: Node, m: Node) -> f64 {
        if n == m {
            0.0
        } else {
            let edges = self.adj_list.get(&n);
            let result = edges.iter().filter(|edge| edge.to == m).nth(0);
            match result {
                Some(edge) => edge.weight,
                None => fail!("No edge found!")
            }
        }
    }

    fn size(&self) -> uint {
        self.adj_list.len()
    }

    fn nodes(&self) -> Vec<Node> {
        range(0, self.size()).collect()
    }

    #[allow(dead_code)]
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
    rng: StdRng,
    graph: Box<Graph>,
    population: Vec<Tour>,
    mutation_rate: f64,
    tournament_size: uint
}

impl Population {
    fn new(population_count: uint, graph: Box<Graph>, mutation_rate: f64, tournament_size: uint, mut rng: StdRng) -> Population {
        let population = Vec::from_fn(population_count, |_| Tour::random_tour(&mut rng, graph));

        Population {
            rng: rng,
            graph: graph,
            population: population,
            mutation_rate: mutation_rate,
            tournament_size: tournament_size
        }
    }

    fn fittest(&self) -> Tour {
        find_min(&self.population)
    }

    fn tournament_selection(&mut self) -> Tour {
        let size = self.population.len();
        let mut buffer: Vec<Tour> = Vec::new();
        for _ in range(0, self.tournament_size) {
            let t = (self.rng.gen::<f64>() * (size as f64)) as uint;
            buffer.push(self.population.get(t).clone());
        }
        find_min(&buffer)
    }

    fn evolve(&mut self) -> Population {
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

fn main() {
    let iter_count = 100;
    let node_count = 15;
    let mutation_rate = 0.03;
    let tournament_size = 5;
    let population_size = 10000;
    let scale = 200.0;

    let rng: StdRng = match StdRng::new() {
        Ok(r) => r,
        Err(_) => fail!("failed to acquire RNG")
    };
    let mut s_rng: StdRng = SeedableRng::from_seed(&[12, 13, 14, 15]);

    let graph = Graph::random_graph(&mut s_rng, node_count, scale, scale);
    let mut pop = Population::new(population_size, box graph, mutation_rate, tournament_size, rng);
    println!("Fittest at start: {}", pop.fittest().total_weight)

    let t0 = precise_time_ns();
    for _ in range(0, iter_count) {
        pop = pop.evolve();
    }
    let t1 = precise_time_ns();

    println!("Fittest at end: {}", pop.fittest().total_weight)
    let dt = ((t1-t0) as f64) / 1e6;
    println!("t_avg = {} ms", dt / iter_count as f64);
}