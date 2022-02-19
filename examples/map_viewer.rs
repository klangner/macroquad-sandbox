// “Escape! Code Your Way Out of a Paper Bag”
//

use std::time::Instant;
use macroquad::prelude::*;
use mapgen::{MapBuilder,Map};
use mapgen::filter::{BspRooms,NearestCorridors};


const WINDOW_WIDTH: usize = 800;
const WINDOW_HEIGHT: usize = 600;


#[derive(Debug)]
struct Viewport {
    scale: f32,
    pos_x: f32,
    pos_y: f32,
}

impl Viewport {
    pub fn new() -> Viewport {
        Viewport {scale: 1., pos_x: 0.0, pos_y: 0.0}
    }

    pub fn zoom_in(&mut self, dt: f32) {
        self.scale += dt;
        if self.scale > 10.0 {
            self.scale = 10.0;
        }
    }

    pub fn zoom_out(&mut self, dt: f32) {
        self.scale -= dt;
        if self.scale < 0.1 {
            self.scale = 0.1;
        }
    }

    pub fn move_right(&mut self, dt: f32) {
        self.pos_x -= 500.0 * dt;
    }

    pub fn move_left(&mut self, dt: f32) {
        self.pos_x += 500.0 * dt;
    }

    pub fn move_down(&mut self, dt: f32) {
        self.pos_y -= 500.0 * dt;
    }

    pub fn move_up(&mut self, dt: f32) {
        self.pos_y += 500.0 * dt;
    }
}

fn draw_map(viewport: &Viewport, map: &Map) {
    let cell_dx = viewport.scale * (WINDOW_WIDTH / map.width) as f32;
    let cell_dy = viewport.scale * (WINDOW_HEIGHT / map.height) as f32;

    clear_background(LIGHTGRAY);
    for x in 0..map.width {
        for y in 0..map.height {
            let color = if map.at(x, y).is_blocked() { DARKGRAY } else { WHITE };
            draw_rectangle(
                x as f32 * cell_dx + viewport.pos_x, 
                y as f32 * cell_dy + viewport.pos_y, 
                cell_dx, 
                cell_dy, color);
        }
    }
}


fn window_conf() -> Conf {
    Conf {
        window_title: "Map viewer".to_owned(),
        fullscreen: false,
        window_width: WINDOW_WIDTH as i32,
        window_height: WINDOW_HEIGHT as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut viewport = Viewport::new();
    let map = MapBuilder::new(80, 60)
        .with(BspRooms::new())
        .with(NearestCorridors::new())
        .build();  

    let mut prev_time = Instant::now(); 
    loop {
        // Delta time from previous frame
        let dt: f32 = prev_time.elapsed().as_millis() as f32 / 1000.;
        prev_time = Instant::now();

        // Process input
        #[cfg(not(target_arch = "wasm32"))]
        if is_key_down(KeyCode::Q) | is_key_down(KeyCode::Escape) {
            break;
        }
        if is_key_down(KeyCode::RightBracket) {
            viewport.zoom_in(dt);
        }
        if is_key_down(KeyCode::LeftBracket) {
            viewport.zoom_out(dt);
        }
        if is_key_down(KeyCode::Left) {
            viewport.move_left(dt);
        }
        if is_key_down(KeyCode::Right) {
            viewport.move_right(dt);
        }
        if is_key_down(KeyCode::Up) {
            viewport.move_up(dt);
        }
        if is_key_down(KeyCode::Down) {
            viewport.move_down(dt);
        }
        // println!("Viewport: {:?}", &viewport);

        // Draw world
        draw_map(&viewport, &map);
        
        next_frame().await
    }
}