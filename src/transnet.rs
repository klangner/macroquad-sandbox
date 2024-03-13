// Track network


#[derive(Clone, Copy)]
pub struct Location {
    pub x: f32,
    pub y: f32,
}

pub struct Connection {
    pub from_location: usize,
    pub to_location: usize,
    length: f32,
}

pub struct TransNet {
    pub locations: Vec<Location>,
    pub connections: Vec<Connection>,
}

#[derive(Clone, Copy)]
pub struct RoutePos {
    connection_id: usize,
    distance: f32,
}


impl Location {
    pub fn new(x: f32, y: f32) -> Self {
        Self{ x, y }
    }
}


impl Connection {
    pub fn new(from_location: usize, to_location: usize, locations: &Vec<Location>) -> Self {
        let pos_a = locations[from_location];
        let pos_b = locations[to_location];
        let dx = pos_b.x - pos_a.x;
        let dy = pos_b.y - pos_a.y;
        let length = (dx.powi(2) + dy.powi(2)).sqrt();
        Self {
            from_location,
            to_location,
            length,
        }
    }
}

impl RoutePos {
    pub fn init(connection_id: usize) -> Self {
        Self::new(connection_id, 0.)
    }

    fn new(connection_id: usize, distance: f32) -> Self {
        Self { connection_id, distance }
    }
}


impl TransNet {
    pub fn new(locations: Vec<Location>, connections: Vec<Connection>) -> Self {
        Self { locations, connections }
    }

    pub fn track_to_map(&self, track_pos: &RoutePos) -> Location {
        let track = &self.connections[track_pos.connection_id];
        let pos_a = self.locations[track.from_location];
        let pos_b = self.locations[track.to_location];
        let dx = pos_b.x - pos_a.x;
        let dy = pos_b.y - pos_a.y;
        let frac = track_pos.distance / track.length;
        let pos_a = self.locations[track.from_location];
        Location::new(pos_a.x + frac * dx, pos_a.y + frac * dy)
    }

    pub fn update_pos(&self, pos: &RoutePos, distance: f32) -> RoutePos {
        let track = &self.connections[pos.connection_id];
        let new_distance = pos.distance + distance;

        if new_distance > track.length {
            let new_edge = self.connections.iter().enumerate()
                .find(|(_, t)| t.from_location == track.to_location)
                .map(|(i, _)| i)
                .unwrap();
            RoutePos::new(new_edge, pos.distance + distance - track.length)
        } else if new_distance < 0. {
            let new_edge = self.connections.iter().enumerate()
                .find(|(_, t)| t.to_location == track.from_location)
                .map(|(i, _)| i)
                .unwrap();
            RoutePos::new(new_edge, &self.connections[new_edge].length + new_distance)
        } else {
            RoutePos::new(pos.connection_id, pos.distance + distance)
        }
    }
}
