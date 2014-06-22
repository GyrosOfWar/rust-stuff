extern crate time;
extern crate graphviz;


use std::rand::{SeedableRng, StdRng};
use time::precise_time_ns;
use std::io::File;

use dot = graphviz;
use graphviz::maybe_owned_vec::IntoMaybeOwnedVector;

use population::Population;
use graph::Graph;
use nodept::Node;
use edge::Edge;

pub mod population;
pub mod tour;
pub mod graph;
pub mod nodept;
pub mod edge;

pub fn render_to<W: Writer>(output: &mut W, graph: &Graph) {
    dot::render(graph, output).unwrap()
}

impl<'a> dot::Labeller<'a, Node, Edge> for Graph {
    fn graph_id(&'a self) -> dot::Id<'a> {
        dot::Id::new("TSP")
    }

    fn node_id(&'a self, n: &Node) -> dot::Id<'a> {
        dot::Id::new(format!("N{}", *n))
    }
}

impl<'a> dot::GraphWalk<'a, Node, Edge> for Graph {
    fn nodes(&'a self) -> dot::Nodes<'a, Node> {
        let ref v: &'a Graph = self;
        let c: Vec<Node> = range(0, v.adj_list.len()).collect();
        c.into_maybe_owned()
    }
    fn edges(&'a self) -> dot::Edges<'a,Edge> {
        let ref v: &'a Graph = self;
        let mut all_edges: Vec<Edge> = Vec::new();

        for edge_list in v.adj_list.values() {
            all_edges.push_all(edge_list.as_slice());
        }
        all_edges.sort();
        all_edges.dedup();

        all_edges.into_maybe_owned()

    }
    fn source(&self, e: &Edge) -> Node { 
        e.from 
    }
    fn target(&self, e: &Edge) -> Node {
        e.to
    }
}

fn tour_to_edges(tour: Vec<Node>, graph: &Graph) -> Vec<Edge> {
    let mut edges: Vec<Edge> = Vec::new();
    let mut last_node = tour.get(0);
    for i in range(1, tour.len()) {
        let next_node = tour.get(i);
        let edge = graph.get_edge(*last_node, *next_node);
        edges.push(edge);
        last_node = next_node;
    }

    edges
}

fn write_to_file(graph: &Graph, file_name: &str) {
    let mut f = File::create(&Path::new(file_name));
    render_to(&mut f, graph);
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