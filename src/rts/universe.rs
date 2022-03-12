
use glam::Vec2;
use mapgen::Map;


pub struct Universe {
    pub map: Map,
    pub unit: Unit,
}

pub struct Unit {
    pub pos: Vec2,
    dest: Vec2,
}

impl Universe {
    pub fn from_map(map: Map) -> Universe {
        let start_pos = map.starting_point.unwrap();
        let unit = Unit::new( start_pos.x as f32 + 0.5, start_pos.y as f32 + 0.5); 
        Universe { map, unit }
    }

    // Move unit to a given cell on the map
    pub fn move_to(&mut self, x: usize, y: usize) {
        self.unit.set_dest(x as f32 + 0.5, y as f32 + 0.5)
    }

    // Clock tick. update sim state
    pub fn tick(&mut self) {
        self.unit.update_pos(0.1);
    }
}

impl Unit {
    pub fn new(x: f32, y: f32) -> Unit {
        Unit {pos: Vec2::new(x, y), dest: Vec2::new(x, y)}
    }

    pub fn update_pos(&mut self, amount: f32) {
        let dir = (self.dest - self.pos).normalize_or_zero();
        self.pos.x += dir.x * amount;
        self.pos.y += dir.y * amount;
    }
    
    pub fn set_dest(&mut self, x: f32, y: f32) {
        self.dest.x = x;
        self.dest.y = y;
    }
}