extern crate native;
extern crate time;
extern crate graphviz;
extern crate getopts;
extern crate rsfml;

use getopts::{optopt, optflag, getopts, OptGroup, Matches};

use std::collections::HashMap;
use std::from_str::FromStr;
use std::io::File;
use std::os::args;
use std::rand::{Rng, SeedableRng, StdRng, task_rng};
use time::precise_time_ns;

use graph::Graph;
use population::{Population, GeneticAlgorithm, GAParameterSet};
use nodept::{NodePt, Node};
use simulated_annealing::{SimulatedAnnealing, TSPAlgorithm};
use tour::Tour;

pub mod edge;
pub mod graph;
pub mod nodept;
pub mod population;
pub mod tour;
pub mod graphviz_conv;
pub mod simulated_annealing;
pub mod branch_and_bound;

static DEFAULT_GA_PARAMS: GAParameterSet = GAParameterSet {
    iterations: 800,
    mutation_rate: 0.02,
    population_size: 400,
    tournament_size: 15
};

fn usage(program: &str, opts: &[OptGroup]) {
    println!("Usage: {} [options]\n", program);
    for o in opts.iter() {
        println!("-{}\t--{}: {}", o.short_name, o.long_name, o.desc);
    }
}

fn parse_opt<T: FromStr>(matches: &Matches, opt: &str, default: T) -> T {
    match matches.opt_str(opt) {
        Some(o) => from_str::<T>(o.as_slice()).unwrap_or(default),
        None => default
    }
}

fn real_main() {
    let args: Vec<String> = args().iter().map(|x| x.to_string()).collect();
    let program = args.get(0).clone();

    let opts = [
        optflag("h", "help", "print this help menu"), 
        optopt("m", "mutation_rate", "change the mutation rate (default: 0.015)", "MUTRATE"),
        optopt("i", "iters", "change the number of GA iterations (default: 50)", "ITERS"),
        optopt("p", "pop_size", "change the population size (default: 5000)", "POPSIZE"),
        optflag("v", "verbose", "print a lot of information, including timing."),
        optopt("r", "read", "read graph from a .tsp file", "READ"),
        optopt("t", "tournament_size", "change the number of specimens used for tournament selection", "TSIZE"),
        optflag("g", "gen_alg", "use the genetic algorithm (either -g or -s must be used)"),
        optflag("s", "simulated_annealing", "use the simulated annealing algorith")
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
    // TODO either remove random graph default or introduce it
    // as a parameter
    let node_count = 15;
    let scale = 200.0;
    // RNG for the GA/SA algorithm
    let rng: StdRng = StdRng::new().ok().expect("failed to acquire RNG");

    // let tsp_algorithm = if matches.opt_present("s") {
    //     SimulatedAnnealing 
    // } else if matches.opt_present("g") {
    //     GeneticAlgorithm
    // } else {
    //     fail!("Need to specify an algorithm!")
    // };

    let mut graph_opt: Option<Graph> = None;

    if matches.opt_present("r") {
        let file_path = parse_opt::<String>(&matches, "r", String::new());
        if file_path.is_empty() {
            fail!("failed to parse file path")
        }
        graph_opt = Some(Graph::from_file(file_path.as_slice(), 800.0));
    }
    else {    
        // make a seeded RNG for the random graph generation for consistent testing
        let mut s_rng: StdRng = SeedableRng::from_seed(&[12, 13, 14, 15]);
        graph_opt = Some(Graph::random_graph(&mut s_rng, node_count, scale, scale))
    }

    let graph = graph_opt.unwrap();

    let mut algorithm: Option<Box<TSPAlgorithm>> = None;
    if matches.opt_present("g") {
        // TODO urgh
        let tournament_size = parse_opt::<uint>(&matches, "t", DEFAULT_GA_PARAMS.tournament_size);
        let mutation_rate = parse_opt::<f64>(&matches, "m", DEFAULT_GA_PARAMS.mutation_rate);
        let iterations = parse_opt::<uint>(&matches, "i", DEFAULT_GA_PARAMS.iterations);
        let population_size = parse_opt::<uint>(&matches, "p", DEFAULT_GA_PARAMS.population_size);
        let params = GAParameterSet {
            tournament_size: tournament_size,
            mutation_rate: mutation_rate,
            iterations: iterations,
            population_size: population_size
        };

        algorithm = Some(box GeneticAlgorithm::new(&mut rng, &graph, &params));
    }

    // if v_flag {
    //     println!("Running TSP-GA on a graph with |N| = {}, |E| = {}", graph.num_nodes, graph.all_edges().len())
    //     println!("GA parameters:")
    //     println!("\tMutation rate = {}", mutation_rate)
    //     println!("\tPopulation size = {}", population_size)
    //     println!("\tNumber of iterations = {}", iter_count)
    //     println!("\tTournament size = {}", tournament_size)
    // }

    // Evolve the population 
    let t0 = precise_time_ns();
    // for _ in range(0, iter_count) {
    //     pop = pop.evolve();
    //     let r = pop.fittest();
    //     if r.total_weight < best_result.total_weight {
    //         best_result = r;
    //     }
    // }
    let result = algorithm.unwrap().search(&graph, &mut rng);
    let t1 = precise_time_ns();

    // Show the end result and the time it took.
    println!("Resulting tour: {}\nwith weight {}", result.nodes, result.total_weight)
    if v_flag {
        let dt = ((t1-t0) as f64) / 1e6;
        //println!("t_avg = {} ms, t_overall = {} s", dt / iter_count as f64, dt / 1000.0);
        //println!("Improvement factor from first solution: {}", (first_result / result.total_weight))     
    }
}

fn main() {
    let mut rng = StdRng::new().ok().expect("Could not create RNG");
    let graph = Graph::from_file("testdata/berlin52.tsp", 1705.0);
    // let initial_solution = Tour::random_tour(&mut rng, &graph);
    // let mut sa = SimulatedAnnealing::new(initial_solution.clone(), 5000.0, 0.999);
    let ga_params = GAParameterSet {
        iterations: 1200,
        mutation_rate: 0.03,
        population_size: 600,
        tournament_size: 15
    };
    let mut ga = GeneticAlgorithm::new(&mut rng, &graph, &ga_params);
    //let result = ga.search(&graph, &mut rng);
    let solution = ga.search(&graph, &mut rng);
    //println!("initial_solution = {}", initial_solution.total_weight)
    println!("solution = {}", solution.total_weight)
}