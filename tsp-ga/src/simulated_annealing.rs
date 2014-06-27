use graph::Graph;
use nodept::{NodePt, Node}


fn nearest_neighbor(tour: Vec<Node>, unvisited_nodes: Vec<Node>, adj_list: &HashMap<Node, Vec<Edge>>) -> Option<Vec<Node>> {
	if tour.len() == graph.num_nodes() {
		return Some(tour);
	}
	let last_node = match tour.last() {
		Some(node) => node,
		None => return None
	};

	let adjacent_edges = adj_list.get(last_node);
	let next_edge = adjacent_edges.iter()
		.filter(|edge| unvisited_nodes.contains(edge.to))
		.min();
	let next_node = next_edge.to;
	
}

fn simulated_annealing(graph: &Graph) -> Vec<Node> {

}