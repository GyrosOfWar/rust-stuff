extern crate graphviz;

use graphviz::maybe_owned_vec::IntoMaybeOwnedVector;
use dot = graphviz;

use edge::Edge;
use graph::Graph;
use nodept::Node;

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