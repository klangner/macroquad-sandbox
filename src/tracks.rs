// Track network


#[derive(Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}


pub struct Track {
    pub from_node: usize,
    pub to_node: usize,
    length: f32,
}

pub struct TrackNetwork {
    pub nodes: Vec<Point>,
    pub tracks: Vec<Track>,
}

#[derive(Clone, Copy)]
pub struct TrackPos {
    edge: usize,
    distance: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Self {
        Self{ x, y }
    }
}


impl Track {
    fn new(from_node: usize, to_node: usize, nodes: &Vec<Point>) -> Self {
        let pos_a = nodes[from_node];
        let pos_b = nodes[to_node];
        let dx = pos_b.x - pos_a.x;
        let dy = pos_b.y - pos_a.y;
        let length = (dx.powi(2) + dy.powi(2)).sqrt();
        Self {
            from_node,
            to_node,
            length,
        }
    }
}

impl TrackPos {
    pub fn new(edge: usize, distance: f32) -> Self {
        Self { edge, distance }
    }
}

impl Default for TrackNetwork {
    /// Demo track
    fn default() -> Self {
        let nodes = vec![
            Point::new(100., 100.),
            Point::new(700., 100.),
            Point::new(700., 500.),
            Point::new(100., 500.),
        ];

        let tracks = vec![
            Track::new(0, 1, &nodes),
            Track::new(1, 2, &nodes),
            Track::new(2, 3, &nodes),
            Track::new(3, 0, &nodes),
        ];

        Self {
            nodes,
            tracks,
        }
    }
}

impl TrackNetwork {
    pub fn track_to_map(&self, track_pos: &TrackPos) -> Point {
        let track = &self.tracks[track_pos.edge];
        let pos_a = self.nodes[track.from_node];
        let pos_b = self.nodes[track.to_node];
        let dx = pos_b.x - pos_a.x;
        let dy = pos_b.y - pos_a.y;
        let frac = track_pos.distance / track.length;
        let pos_a = self.nodes[track.from_node];
        Point::new(pos_a.x + frac * dx, pos_a.y + frac * dy)
    }

    pub fn update_pos(&self, pos: &TrackPos, distance: f32) -> TrackPos {
        let track = &self.tracks[pos.edge];
        let new_distance = pos.distance + distance;

        if new_distance > track.length {
            let new_edge = self.tracks.iter().enumerate()
                .find(|(_, t)| t.from_node == track.to_node)
                .map(|(i, _)| i)
                .unwrap();
            TrackPos::new(new_edge, pos.distance + distance - track.length)
        } else if new_distance < 0. {
            let new_edge = self.tracks.iter().enumerate()
                .find(|(_, t)| t.to_node == track.from_node)
                .map(|(i, _)| i)
                .unwrap();
            TrackPos::new(new_edge, &self.tracks[new_edge].length + new_distance)
        } else {
            TrackPos::new(pos.edge, pos.distance + distance)
        }
    }
}
