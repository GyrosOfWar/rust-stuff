extern crate native;
extern crate time;
extern crate graphviz;
extern crate getopts;
extern crate rsfml;

use rsfml::system::Vector2f;
use rsfml::window::{ContextSettings, VideoMode, event, Close};
use rsfml::graphics::{RenderWindow, CircleShape, Color, VertexArray, Vertex, LinesStrip};

// TODO use terminal colors for nicer colored output
//extern crate term;

use dot = graphviz;
use getopts::{optopt, optflag, getopts, OptGroup, Matches};
use std::io::File;
use std::os::args;
use std::rand::{Rng, SeedableRng, StdRng, task_rng};
use time::precise_time_ns;
use std::from_str::FromStr;
use std::collections::HashMap;

use graph::Graph;
use population::Population;
use nodept::{NodePt, Node};

pub mod edge;
pub mod graph;
pub mod nodept;
pub mod population;
pub mod tour;
pub mod graphviz_conv;

static DEFAULT_ITERS: uint = 800;
static DEFAULT_MUT_RATE: f64 = 0.02;
static DEFAULT_POP_SIZE: uint = 200;
static DEFAULT_TOURNAMENT_SIZE: uint = 15;

fn write_to_file(graph: &Graph, file_name: &str) {
    let mut f = File::create(&Path::new(file_name));
    render_to(&mut f, graph);
}

pub fn render_to<W: Writer>(output: &mut W, graph: &Graph) {
    dot::render(graph, output).unwrap()
}

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

fn text_main() {
    let args: Vec<String> = args().iter().map(|x| x.to_string()).collect();
    let program = args.get(0).clone();

    let opts = [
        optflag("h", "help", "print this help menu"), 
        optopt("m", "mutation_rate", "change the mutation rate (default: 0.015)", "MUTRATE"),
        optopt("i", "iters", "change the number of GA iterations (default: 50)", "ITERS"),
        optopt("p", "pop_size", "change the population size (default: 5000)", "POPSIZE"),
        optflag("v", "verbose", "print a lot of information, including timing."),
        optopt("r", "read", "read graph from a .tsp file", "READ"),
        optopt("t", "tournament_size", "change the number of specimens used for tournament selection", "TSIZE")
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
    let tournament_size = parse_opt::<uint>(&matches, "t", DEFAULT_TOURNAMENT_SIZE);
    let scale = 200.0;
    let mutation_rate = parse_opt::<f64>(&matches, "m", DEFAULT_MUT_RATE);
    let iter_count = parse_opt::<uint>(&matches, "i", DEFAULT_ITERS);
    let population_size = parse_opt::<uint>(&matches, "p", DEFAULT_POP_SIZE);

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

    if v_flag {
        println!("Running TSP-GA on a graph with |N| = {}, |E| = {}", graph.num_nodes, graph.all_edges().len())
        println!("GA parameters:")
        println!("\tMutation rate = {}", mutation_rate)
        println!("\tPopulation size = {}", population_size)
        println!("\tNumber of iterations = {}", iter_count)
        println!("\tTournament size = {}", tournament_size)
    }

    // RNG for the GA
    let rng: StdRng = match StdRng::new() {
        Ok(r) => r,
        Err(_) => fail!("failed to acquire RNG")
    };

    let mut pop = Population::new(population_size, graph, mutation_rate, tournament_size, rng);
    let first_result = pop.fittest().total_weight;
    let mut best_result = pop.fittest();
    if v_flag {
        println!("Fittest at start: {}", first_result)
    }
    // Evolve the population 
    let t0 = precise_time_ns();
    for _ in range(0, iter_count) {
        pop = pop.evolve();
        let r = pop.fittest();
        if r.total_weight < best_result.total_weight {
            best_result = r;
        }
    }
    let t1 = precise_time_ns();

    // Show the end result and the time it took.
    println!("Resulting tour: {}\nwith weight {}", best_result.nodes, best_result.total_weight)
    if v_flag {
        let dt = ((t1-t0) as f64) / 1e6;
        println!("t_avg = {} ms, t_overall = {} s", dt / iter_count as f64, dt / 1000.0);
        println!("Improvement factor from first solution: {}", (first_result / best_result.total_weight))     
    }
}

fn draw_tour(nodes: &Vec<NodePt>, tour: &Vec<NodePt>) -> (VertexArray, Vec<CircleShape>) {
    let mut node_circles: Vec<CircleShape> = Vec::new();
    let mut vertex_array = VertexArray::new().expect("Failed to create VertexArray");
    vertex_array.set_primitive_type(LinesStrip);

    for node in nodes.iter() {
        let mut circle = CircleShape::new().expect("Could not create CircleShape");
        circle.set_radius(4.0);
        circle.set_fill_color(&Color::red());
        circle.set_position(&Vector2f::new((node.x as f32) - 4.0, (node.y as f32) - 4.0));
        node_circles.push(circle)
    }

    for tour_node in tour.iter() {
        let position = Vector2f::new(tour_node.x as f32, tour_node.y as f32);
        vertex_array.append(&Vertex::new_with_pos_color(&position, &Color::black()));
    }

    let first = tour.get(0);
    vertex_array.append(&Vertex::new_with_pos_color(&Vector2f::new(first.x as f32, first.y as f32), &Color::black()));
    (vertex_array, node_circles)
}

fn sfml_main() {
    let settings =         
        ContextSettings {
            depth_bits : 0,
            stencil_bits : 0,
            antialiasing_level : 4,
            major_version : 2,
            minor_version : 0
        };

    let mut window = RenderWindow::new
        (VideoMode::new_init(800, 800, 32), 
        "TSP-GA visualizer", 
        Close, 
        &settings).expect("Could not create a window!");

    let scale = 750.0;
    let graph = Graph::from_file("testdata/berlin52.tsp", scale);
    let rng: StdRng = StdRng::new().ok().expect("Failed to acquire RNG!");
    let mut pop = Population::new(250, graph.clone(), 0.03, 15, rng);

    for _ in range(0u, 500) {
        pop = pop.evolve();
    }

    let result = pop.fittest();
    println!("fittest = {} ", result);
    let result_positions = graph.tour_to_points(&result.nodes);
    let node_points: Vec<NodePt> = graph.get_node_points();
    let (tour_vertices, node_circles) = draw_tour(&node_points, &result_positions);
    while window.is_open() {
        // Handle events
        for event in window.events() {
            match event {
                event::Closed => window.close(),
                _             => {/* do nothing */}
            }
        }

        // Clear the window
        window.clear(&Color::new_RGB(255, 255, 255));

        for circle in node_circles.iter() {
            window.draw(circle);
        }
        window.draw(&tour_vertices);

        // Display things on screen
        window.display();
    }
}

fn main() {
    sfml_main();
}

#[start]
fn start(argc: int, argv: *const *const u8) -> int {
    native::start(argc, argv, main)
}