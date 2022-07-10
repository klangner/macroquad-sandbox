// “Escape! Code Your Way Out of a Paper Bag”
//

use macroquad::prelude::*;
use macroquad_sandbox::{fluid::*, quad::draw_arrow};


const WINDOW_WIDTH: i32 = 1024;
const WINDOW_HEIGHT: i32 = 800;
const UNIVERSE_WIDTH: i32 = 20;
const UNIVERSE_HEIGHT: i32 = 20;
const SOURCE_SIZE: i32 = 1;


fn add_particles(dt: f32, x: i32, y: i32, universe: &mut Universe){
    let source_power = dt  / 1.0;
    for i in i32::max(x-SOURCE_SIZE+1, 0)..i32::min(x+SOURCE_SIZE, universe.width()) {
        for j in i32::max(y-SOURCE_SIZE+1, 0)..i32::min(y+SOURCE_SIZE, universe.height()) {
            universe.increase_density(i as i32, j as i32, source_power);
        }
    }
}

fn draw_universe(universe: &Universe) {
    clear_background(BLACK);
    draw_densities(universe);
    draw_velocities(universe);
    draw_grid_lines(universe);
}
    
fn draw_densities(universe: &Universe) {
    let cell_dx =  screen_width() / universe.width() as f32;
    let cell_dy = screen_height() / universe.height() as f32;

    for x in 0..universe.width() {
        for y in 0..universe.height() {
            let cx = x as f32 * cell_dx;
            let cy = y as f32 * cell_dy;
            let intensity = universe.density_at(x, y);
            let color = Color::new(intensity, intensity, intensity, 1.00);
            draw_rectangle(cx, cy, cell_dx, cell_dy, color);
        }
    }
}

fn draw_velocities(universe: &Universe) {
    let cell_dx =  screen_width() / universe.width() as f32;
    let cell_dy = screen_height() / universe.height() as f32;

    for x in 0..universe.width() {
        for y in 0..universe.height() {
            let cx = cell_dx / 2. + x as f32 * cell_dx;
            let cy = cell_dy / 2. + y as f32 * cell_dy;
            let v = universe.velocity_at(x, y);
            draw_arrow(cx, cy, v.x*cell_dx, v.y*cell_dy, RED);
        }
    }
}

pub fn draw_grid_lines(universe: &Universe) {
    let cell_dx =  screen_width() / universe.width() as f32;
    let cell_dy = screen_height() / universe.height() as f32;
    
    for i in 1..universe.width() {
        let x = i as f32 * cell_dx;
        draw_line(x, 0.0, x, screen_height(), 1.0, DARKGRAY)
    }
    for i in 1..universe.height() {
        let y = i as f32 * cell_dy;
        draw_line(0.0, y, screen_width(), y, 1.0, DARKGRAY)
    }
}



fn window_conf() -> Conf {
    Conf {
        window_title: "Fluid sim".to_owned(),
        fullscreen: false,
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut universe = UniverseBuilder::new(UNIVERSE_WIDTH, UNIVERSE_HEIGHT)
        .with_velocity(Vec2d::new(0.8, 0.0))
        .with_density(0.)
        .with_diffusion_rate(0.001)
        .build();
    let cell_dx = screen_width() / universe.width() as f32;
    let cell_dy = screen_height() / universe.height() as f32;

    loop {
        let dt = get_frame_time();
        
        universe.diffuse(dt);

        // Process input
        #[cfg(not(target_arch = "wasm32"))]
        if is_key_down(KeyCode::Q) | is_key_down(KeyCode::Escape) {
            break;
        }

        // Process mouse
        if is_mouse_button_down(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            let x = f32::trunc(mouse_x / cell_dx) as i32;
            let y = f32::trunc(mouse_y / cell_dy) as i32;

            add_particles(dt, x, y, &mut universe);
    
        }

        // Draw universe
        draw_universe(&universe);
        
        next_frame().await
    }
}