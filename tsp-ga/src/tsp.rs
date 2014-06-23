extern crate time;
extern crate graphviz;
extern crate getopts;

use std::rand::{SeedableRng, StdRng};
use time::precise_time_ns;
use std::io::File;
use getopts::{optopt, optflag, getopts, OptGroup};
use dot = graphviz;
use graphviz::maybe_owned_vec::IntoMaybeOwnedVector;

use population::Population;
use graph::Graph;
use nodept::Node;
use edge::Edge;
use std::os::args;

pub mod population;
pub mod tour;
pub mod graph;
pub mod nodept;
pub mod edge;

// graphviz graph labeller implementation
impl<'a> dot::Labeller<'a, Node, Edge> for Graph {
    fn graph_id(&'a self) -> dot::Id<'a> {
        dot::Id::new("TSP")
    }

    fn node_id(&'a self, n: &Node) -> dot::Id<'a> {
        dot::Id::new(format!("N{}", *n))
    }
}

// graphviz graph walking implementation
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

fn write_to_file(graph: &Graph, file_name: &str) {
    let mut f = File::create(&Path::new(file_name));
    render_to(&mut f, graph);
}

pub fn render_to<W: Writer>(output: &mut W, graph: &Graph) {
    dot::render(graph, output).unwrap()
}

fn main() {
    // TODO use getopts to make the GA parameters commandline options

    // let args: Vec<String> = args().iter().map(|x| x.to_string()).collect();
    // let program = args.get(0).clone();

    // let opts = [
    //     optopt("")
    // ];

    let iter_count = 50;
    let node_count = 15;
    let mutation_rate = 0.03;
    let tournament_size = 5;
    let population_size = 5000;
    let scale = 200.0;

    // RNG for the GA
    let rng: StdRng = match StdRng::new() {
        Ok(r) => r,
        Err(_) => fail!("failed to acquire RNG")
    };
    // make a seeded RNG for the random graph generation for consistent
    // testing
    let mut s_rng: StdRng = SeedableRng::from_seed(&[12, 13, 14, 15]);
    // Generate a random graph and a population of tours, print out the best
    // of those tours to start with.
    let graph = Graph::random_graph(&mut s_rng, node_count, scale, scale);
    let mut pop = Population::new(population_size, box graph, mutation_rate, tournament_size, rng);
    println!("Fittest at start: {}", pop.fittest().total_weight)

    // Evolve the population 
    let t0 = precise_time_ns();
    for _ in range(0, iter_count) {
        pop = pop.evolve();
    }
    let t1 = precise_time_ns();

    // Show the end result and the time it took.
    println!("Fittest at end: {}", pop.fittest().total_weight)
    let dt = ((t1-t0) as f64) / 1e6;
    println!("t_avg = {} ms, t_overall = {} s", dt / iter_count as f64, dt / 1000.0);
}