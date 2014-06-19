extern crate test;

use test::Bencher;

#[test]
fn test_random_graph() {
	let mut rng = task_rng();
	let node_count = 4u;
	let rand_graph = Graph::random_graph(&mut rng, node_count, 200.0, 200.0);

	assert!(rand_graph.nodes.len() == 4u)
}