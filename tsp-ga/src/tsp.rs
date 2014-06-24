extern crate time;
extern crate graphviz;
extern crate getopts;

use dot = graphviz;
use getopts::{optopt, optflag, getopts, OptGroup};
use std::io::File;
use std::os::args;
use std::rand::{SeedableRng, StdRng};
use time::precise_time_ns;

use graph::Graph;
use population::Population;

pub mod edge;
pub mod graph;
pub mod nodept;
pub mod population;
pub mod tour;
pub mod graphviz_conv;

static DEFAULT_ITERS: uint = 50;
static DEFAULT_MUT_RATE: f64 = 0.015;
static DEFAULT_POP_SIZE: uint = 5000;

fn write_to_file(graph: &Graph, file_name: &str) {
    let mut f = File::create(&Path::new(file_name));
    render_to(&mut f, graph);
}

pub fn render_to<W: Writer>(output: &mut W, graph: &Graph) {
    dot::render(graph, output).unwrap()
}

fn usage(program: &str, opts: &[OptGroup]) {
    println!("Usage: {} [options]", program);
    for o in opts.iter() {
        println!("-{} --{}\t{}", o.short_name, o.long_name, o.desc);
    }
}

fn main() {
    // TODO use getopts to make the GA parameters commandline options

    let args: Vec<String> = args().iter().map(|x| x.to_string()).collect();
    let program = args.get(0).clone();

    let opts = [
        optflag("h", "help", "print this help menu"), 
        optopt("m", "mutrate", "change the mutation rate (default: 0.015)", "MUTRATE"),
        optopt("i", "iters", "change the number of GA iterations (default: 50)", "ITERS"),
        optopt("p", "popsize", "change the population size (default: 5000)", "POPSIZE"),
        optflag("v", "verbose", "print a lot of information, including timing.")
    ];

    let matches = match getopts(args.tail(), opts) {
        Ok(m) => m,
        Err(f) => fail!(f.to_str())
    };

    if matches.opt_present("h") {
        usage(program.as_slice(), opts);
        return;
    }
    let v_flag = matches.opt_present("v");

    let node_count = 15;
    let tournament_size = 5;
    let scale = 200.0;

    let mutation_rate = matches.opt_str("m")
        .map(|v| from_str::<f64>(v.as_slice()).unwrap_or(DEFAULT_MUT_RATE))
        .unwrap_or(DEFAULT_MUT_RATE);

    let iter_count = matches.opt_str("i")
        .map(|v| from_str::<uint>(v.as_slice()).unwrap_or(DEFAULT_ITERS))
        .unwrap_or(DEFAULT_ITERS);

    let population_size = matches.opt_str("p")
        .map(|v| from_str::<uint>(v.as_slice()).unwrap_or(DEFAULT_POP_SIZE))
        .unwrap_or(DEFAULT_POP_SIZE);

    // make a seeded RNG for the random graph generation for consistent
    // testing
    let mut s_rng: StdRng = SeedableRng::from_seed(&[12, 13, 14, 15]);
    // Generate a random graph and a population of tours, print out the best
    // of those tours to start with.
    let graph = Graph::random_graph(&mut s_rng, node_count, scale, scale);
    if v_flag {
        println!("Running TSP-GA on a graph with |N| = {}, |E| = {}", node_count, graph.all_edges().len())
        println!("GA parameters:")
        println!("\tMutation rate = {}", mutation_rate)
        println!("\tPopulation size = {}", population_size)
        println!("\tNumber of iterations = {}", iter_count)
    }

    // RNG for the GA
    let rng: StdRng = match StdRng::new() {
        Ok(r) => r,
        Err(_) => fail!("failed to acquire RNG")
    };

    let mut pop = Population::new(population_size, box graph, mutation_rate, tournament_size, rng);
    if v_flag {
        println!("Fittest at start: {}", pop.fittest().total_weight)
    }
    // Evolve the population 
    let t0 = precise_time_ns();
    for _ in range(0, iter_count) {
        pop = pop.evolve();
    }
    let t1 = precise_time_ns();

    // Show the end result and the time it took.
    let result = pop.fittest();
    println!("Resulting tour: {} with weight {}", result.nodes, result.total_weight)
    if v_flag {
        let dt = ((t1-t0) as f64) / 1e6;
        println!("t_avg = {} ms, t_overall = {} s", dt / iter_count as f64, dt / 1000.0);
    }
}