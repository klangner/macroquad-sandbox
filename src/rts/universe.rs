
use glam::Vec2;
use crate::rts::WorldMap;
use crate::rts::pathfinding as pf;


pub struct Unit {
    pub pos: Vec2,
    dest: Vec2,
    pub is_moving: bool,
}

pub struct Universe {
    pub map: WorldMap,
    pub units: Vec<Unit>,
    pub path: Vec<Vec2>,
}

impl Universe {
    pub fn from_map(map: WorldMap) -> Universe {
        Universe { map, units: vec![], path: vec![] }
    }

    pub fn add_unit(&mut self, pos_x: usize, pos_y: usize) {
        let unit =  Unit::new(pos_x as f32 + 0.5, pos_y as f32 + 0.5);
        self.units.push(unit);
    }

    // Move unit to a given cell on the map
    pub fn move_to(&mut self, x: usize, y: usize) {
        if self.units.len() > 0 {
            let unit = &self.units[0];
            let pos = (unit.dest.x as i32, unit.dest.y as i32);
            self.path = pf::find_path(&self.map, pos, (x as i32, y as i32))
                .into_iter().map(|(i, j)| Vec2::new(i as f32 + 0.5, j as f32 + 0.5))
                .collect();
        }
    }

    // Clock tick. update sim state
    pub fn tick(&mut self) {
        for unit in &mut self.units {

            if let Some(&dest) = self.path.get(0) {
                if unit.pos != dest {
                    unit.dest = dest;
                } else {
                    self.path.remove(0);
                }
                unit.update_pos(0.1);
            }
        }
    }
}

impl Unit {
    pub fn new(x: f32, y: f32) -> Unit {
        Unit {pos: Vec2::new(x, y), dest: Vec2::new(x, y), is_moving: false}
    }

    pub fn update_pos(&mut self, amount: f32) {
        let dir = (self.dest - self.pos).normalize_or_zero();
        self.pos.x += dir.x * amount;
        self.pos.y += dir.y * amount;
        self.is_moving = dir.length() > 0.
    }
}