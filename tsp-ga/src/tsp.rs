extern crate time;
extern crate getopts;
extern crate rand;

// TODO use terminal colors for nicer colored output
// extern crate term;

use getopts::{Options, Matches};
use std::env::args;
use rand::{SeedableRng, StdRng};
use time::precise_time_ns;
use std::str::FromStr;

use graph::Graph;
use population::Population;

pub mod edge;
pub mod graph;
pub mod nodept;
pub mod population;
pub mod tour;
// pub mod graphviz_conv;

static DEFAULT_ITERS: usize = 800;
static DEFAULT_MUT_RATE: f64 = 0.02;
static DEFAULT_POP_SIZE: usize = 200;
static DEFAULT_TOURNAMENT_SIZE: usize = 15;

fn usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief[..]));
}

fn parse_opt<T: FromStr>(matches: &Matches, opt: &str, default: T) -> T {
    match matches.opt_str(opt) {
        Some(o) => o.parse::<T>().unwrap_or(default),
        None => default,
    }
}

fn main() {
    let args: Vec<String> = args().skip(1).collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optopt("m",
                "mutation_rate",
                "change the mutation rate (default: 0.015)",
                "MUTRATE");
    opts.optopt("i",
                "iters",
                "change the number of GA iterations (default: 50)",
                "ITERS");
    opts.optopt("p",
                "pop_size",
                "change the population size (default: 5000)",
                "POPSIZE");
    opts.optflag("v",
                 "verbose",
                 "print a lot of information, including timing.");
    opts.optopt("r", "read", "read graph from a .tsp file", "READ");
    opts.optopt("t",
                "tournament_size",
                "change the number of specimens used for tournament selection",
                "TSIZE");

    let matches = match opts.parse(args) {
        Ok(m) => m,
        Err(_) => panic!("Failed matching options"),
    };

    if matches.opt_present("h") {
        usage(&program, opts);
        return;
    }
    let v_flag = matches.opt_present("v");

    let node_count = 15;
    let tournament_size = parse_opt::<usize>(&matches, "t", DEFAULT_TOURNAMENT_SIZE);
    let scale = 200.0;
    let mutation_rate = parse_opt::<f64>(&matches, "m", DEFAULT_MUT_RATE);
    let iter_count = parse_opt::<usize>(&matches, "i", DEFAULT_ITERS);
    let population_size = parse_opt::<usize>(&matches, "p", DEFAULT_POP_SIZE);

    let graph;

    if matches.opt_present("r") {
        let file_path = parse_opt::<String>(&matches, "r", String::new());
        if file_path.is_empty() {
            panic!("failed to parse file path")
        }
        graph = Graph::from_file(&file_path).unwrap();
    } else {
        // make a seeded RNG for the random graph generation for consistent testing
        let seed: &[_] = &[12, 13, 14, 15];
        let mut s_rng: StdRng = SeedableRng::from_seed(seed);
        graph = Graph::random_graph(&mut s_rng, node_count, scale, scale);
    }

    if v_flag {
        println!("Running TSP-GA on a graph with |N| = {}, |E| = {}",
                 graph.num_nodes,
                 graph.all_edges().len());
        println!("GA parameters:");
        println!("\tMutation rate = {}", mutation_rate);
        println!("\tPopulation size = {}", population_size);
        println!("\tNumber of iterations = {}", iter_count);
        println!("\tTournament size = {}", tournament_size);
    }

    // RNG for the GA
    let rng: StdRng = match StdRng::new() {
        Ok(r) => r,
        Err(_) => panic!("failed to acquire RNG"),
    };

    let mut pop = Population::new(population_size,
                                  Box::new(graph),
                                  mutation_rate,
                                  tournament_size,
                                  rng);
    let first_result = pop.fittest().total_weight;
    let mut best_result = pop.fittest();
    if v_flag {
        println!("Fittest at start: {}", first_result)
    }
    // Evolve the population
    let t0 = precise_time_ns();
    for _ in 0..iter_count {
        pop = pop.evolve();
        let r = pop.fittest();
        if r.total_weight < best_result.total_weight {
            best_result = r;
        }
    }
    let t1 = precise_time_ns();

    // Show the end result and the time it took.
    println!("Resulting tour: {:?}\nwith weight {}",
             best_result.nodes,
             best_result.total_weight);
    if v_flag {
        let dt = ((t1 - t0) as f64) / 1e6;
        println!("t_avg = {} ms, t_overall = {} s",
                 dt / iter_count as f64,
                 dt / 1000.0);
        println!("Improvement factor from first solution: {}",
                 (first_result / best_result.total_weight));
    }
}
