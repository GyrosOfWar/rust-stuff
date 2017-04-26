pub type Node = usize;

#[derive(Debug, Copy, Clone)]
pub struct NodePt {
    pub id: Node,
    pub x: f64,
    pub y: f64,
}

impl NodePt {
    pub fn new(node_id: Node, x: f64, y: f64) -> NodePt {
        NodePt {
            id: node_id,
            x: x,
            y: y,
        }
    }

    pub fn distance_to(self, other: NodePt) -> f64 {
        let xx = (self.x - other.x) * (self.x - other.x);
        let yy = (self.y - other.y) * (self.y - other.y);

        (xx + yy).sqrt().round()
    }
}

impl PartialEq for NodePt {
    fn eq(&self, other: &NodePt) -> bool {
        self.id == other.id
    }

    fn ne(&self, other: &NodePt) -> bool {
        self.id != other.id
    }
}
