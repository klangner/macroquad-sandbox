// “Escape! Code Your Way Out of a Paper Bag”
//

use macroquad::prelude::*;
use glam::Vec2;
use mapgen::{Map, MapBuilder, filter::*};


const SCREEN_WIDTH: usize = 1200;
const SCREEN_HEIGHT: usize = 900;


struct Universe {
    map: Map,
    unit: Unit,
}

struct Unit {
    pos: Vec2,
    dest: Vec2,
}

impl Universe {
    pub fn new(width: usize, height: usize) -> Universe {
        let map = MapBuilder::new(width, height)
            .with(NoiseGenerator::uniform())
            .with(CellularAutomata::new())
            .with(AreaStartingPosition::new(XStart::CENTER, YStart::CENTER))
            .with(CullUnreachable::new())
            .with(DistantExit::new())
            .build();
        let start_pos = map.starting_point.unwrap();
        let unit = Unit::new( start_pos.x as f32 + 0.5, start_pos.y as f32 + 0.5); 
        Universe { map, unit }
    }

    pub fn set_dest(&mut self, x: usize, y: usize) {
        self.unit.set_dest(x as f32 + 0.5, y as f32 + 0.5)
    }

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

fn draw(universe: &Universe) {
    let cell_dx = (SCREEN_WIDTH / universe.map.width) as f32;
    let cell_dy = (SCREEN_HEIGHT / universe.map.height) as f32;

    clear_background(LIGHTGRAY);
    for x in 0..universe.map.width {
        for y in 0..universe.map.height {
            let color = if universe.map.at(x, y).is_blocked() { DARKGRAY } else { WHITE };
            draw_rectangle(
                x as f32 * cell_dx, 
                y as f32 * cell_dy, 
                cell_dx, 
                cell_dy, color);
        }
    }

    draw_circle(universe.unit.pos.x * cell_dx, universe.unit.pos.y * cell_dy, 5.0, BLUE);
}


fn window_conf() -> Conf {
    Conf {
        window_title: "Movable object".to_owned(),
        fullscreen: false,
        window_width: SCREEN_WIDTH as i32,
        window_height: SCREEN_HEIGHT as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut universe = Universe::new(80, 60);

    loop {
        if is_key_down(KeyCode::Q) | is_key_down(KeyCode::Escape) {
            break;
        }

        universe.tick();

        // Process mouse
        if is_mouse_button_down(MouseButton::Left) {
            let (x, y) = mouse_position();
            universe.set_dest(x as usize * 80 / SCREEN_WIDTH , y as usize * 60 / SCREEN_HEIGHT);
        }

        draw(&universe);

        next_frame().await
    }
}