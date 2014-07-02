extern crate native;
extern crate getopts;
extern crate rsfml;

use std::collections::HashMap;
use std::from_str::FromStr;
use std::io::File;
use std::os::args;
use std::rand::{Rng, SeedableRng, StdRng, task_rng};
use rsfml::graphics::{RenderWindow, CircleShape, Color, VertexArray, Vertex, LinesStrip};
use rsfml::system::Vector2f;
use rsfml::window::{ContextSettings, VideoMode, event, Close};

use graph::Graph;
use population::Population;
use nodept::{NodePt, Node};
use simulated_annealing::simulated_annealing;
use tour::Tour;

pub mod edge;
pub mod graph;
pub mod nodept;
pub mod population;
pub mod tour;
pub mod simulated_annealing;


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

fn main() {
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
    let node_count = 70;
    let asdf = Graph::from_file("testdata/berlin52.tsp", scale);

    let mut rng: StdRng = StdRng::new().ok().expect("Failed to acquire RNG!");
    let graph = Graph::random_graph(&mut rng, node_count, scale, scale);
    let mut pop = Population::new(250, graph.clone(), 0.03, 15, rng);

    let mut k = 1;

    let result = pop.fittest();
    println!("fittest = {} ", result);
    let mut result_positions = graph.tour_to_points(&result.nodes);
    let node_points: Vec<NodePt> = graph.get_node_points();
    let b = draw_tour(&node_points, &result_positions);
    let mut tour_vertices = b.clone().val0();
    let mut node_circles = b.clone().val1();

    while window.is_open() {
        // Handle events
        for event in window.events() {
            match event {
                event::Closed => window.close(),
                _             => {/* do nothing */}
            }
        }

        if k % 100 == 0 {
            for _ in range(0u, 150) {
                pop = pop.evolve();
            }
            let result = pop.fittest();
            result_positions = graph.tour_to_points(&result.nodes);
            let b = draw_tour(&node_points, &result_positions);
            tour_vertices = b.clone().val0();
            node_circles = b.clone().val1();
            println!("fittest = {}", result);
        }

        // Clear the window
        window.clear(&Color::new_RGB(255, 255, 255));

        for circle in node_circles.iter() {
            window.draw(circle);
        }
        window.draw(&tour_vertices);

        // Display things on screen
        window.display();
        k += 1;
    }
}

#[start]
fn start(argc: int, argv: *const *const u8) -> int {
    native::start(argc, argv, main)
}