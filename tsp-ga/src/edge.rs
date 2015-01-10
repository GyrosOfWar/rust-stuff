use nodept::{Node, NodePt};
use std::cmp::Ordering;
use std::cmp::Ordering::{Less, Equal, Greater};

#[derive(Show, Clone, Copy)]
pub struct Edge {
    pub from: Node,
    pub to: Node,
    pub weight: f64
}

impl Edge {
    pub fn new(a: NodePt, b: NodePt) -> Edge {
        Edge {
            from: a.id,
            to: b.id,
            weight: a.distance_to(b)
        }
    }

    pub fn reverse(&self) -> Edge {
        Edge {
            from: self.to,
            to: self.from,
            weight: self.weight
        }
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Edge) -> bool {
        self.from == other.from && self.to == other.to 
    }

    fn ne(&self, other: &Edge) -> bool {
        self.from != other.from && self.to != other.to
    }
}

impl Eq for Edge {}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Edge) -> Option<Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Edge) -> Ordering {
        if self.weight < other.weight {
            Less
        }
        else if self.weight == other.weight {
            Equal
        }
        else {
            Greater
        }
    }
}
