// Track network


#[derive(Clone, Copy)]
pub struct Node {
    pub x: f32,
    pub y: f32,
}

pub struct Edge {
    pub from_node_id: usize,
    pub to_node_id: usize,
    length: f32,
}

pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

#[derive(Clone, Copy)]
pub struct GraphPos {
    edge_id: usize,
    distance: f32,
}


impl Node {
    pub fn new(x: f32, y: f32) -> Self {
        Self{ x, y }
    }
}


impl Edge {
    pub fn new(from_node_id: usize, to_node_id: usize, locations: &Vec<Node>) -> Self {
        let pos_a = locations[from_node_id];
        let pos_b = locations[to_node_id];
        let dx = pos_b.x - pos_a.x;
        let dy = pos_b.y - pos_a.y;
        let length = (dx.powi(2) + dy.powi(2)).sqrt();
        Self {
            from_node_id,
            to_node_id,
            length,
        }
    }
}

impl GraphPos {
    pub fn init(edge_id: usize) -> Self {
        Self::new(edge_id, 0.)
    }

    fn new(edge_id: usize, distance: f32) -> Self {
        Self { edge_id, distance }
    }
}


impl Graph {
    pub fn new(nodes: Vec<Node>, edges: Vec<Edge>) -> Self {
        Self { nodes, edges }
    }

    pub fn pos_to_location(&self, graph_pos: &GraphPos) -> Node {
        let track = &self.edges[graph_pos.edge_id];
        let pos_a = self.nodes[track.from_node_id];
        let pos_b = self.nodes[track.to_node_id];
        let dx = pos_b.x - pos_a.x;
        let dy = pos_b.y - pos_a.y;
        let frac = graph_pos.distance / track.length;
        let pos_a = self.nodes[track.from_node_id];
        Node::new(pos_a.x + frac * dx, pos_a.y + frac * dy)
    }

    // route is a list of edges
    pub fn update_pos(&self, pos: &GraphPos, route: &Vec<usize>, distance: f32) -> GraphPos {
        let track = &self.edges[pos.edge_id];
        let new_distance = pos.distance + distance;

        if new_distance > track.length {
            let new_edge = route.iter()
                .find(|&edge_id| self.edges[*edge_id].from_node_id == track.to_node_id)
                .unwrap();
            GraphPos::new(*new_edge, pos.distance + distance - track.length)
        } else if new_distance < 0. {
            let new_edge = route.iter()
                .find(|&edge_id| self.edges[*edge_id].to_node_id == track.from_node_id)
                .unwrap();
            GraphPos::new(*new_edge, &self.edges[*new_edge].length + new_distance)
        } else {
            GraphPos::new(pos.edge_id, pos.distance + distance)
        }
    }
}
