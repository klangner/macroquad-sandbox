// “Escape! Code Your Way Out of a Paper Bag”
//

use macroquad::prelude::*;
use macroquad_sandbox::fluid::Vec2d;


const SCREEN_WIDTH: u32 = 512;
const SCREEN_HEIGHT: u32 = 512;


struct Unit {
    pos: Vec2d,
    dest: Vec2d,
}

impl Unit {
    pub fn new(x: f32, y: f32) -> Unit {
        Unit {pos: Vec2d::new(x, y), dest: Vec2d::new(0., 0.)}
    }

    pub fn update(&mut self, dt: f32) {
        let amount = 1. * dt;
        self.pos.x += (self.dest.x - self.pos.x) * amount;
        self.pos.y += (self.dest.y - self.pos.y) * amount;
    }
    
    pub fn set_dest(&mut self, x: f32, y: f32) {
        self.dest.x = x;
        self.dest.y = y;
    }
}

fn draw( unit: &Unit) {
    clear_background(WHITE);
    draw_circle(unit.pos.x, unit.pos.y, 5.0, BLUE);
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
    let mut unit = Unit::new( (SCREEN_WIDTH / 2) as f32, (SCREEN_HEIGHT / 2) as f32); 

    loop {
        let dt = get_frame_time();
        if is_key_down(KeyCode::Q) | is_key_down(KeyCode::Escape) {
            break;
        }

         // Process mouse
        if is_mouse_button_down(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            unit.set_dest(mouse_x, mouse_y);
        }
        unit.update(dt);
        draw(&unit);

        next_frame().await
    }
}