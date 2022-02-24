// “Escape! Code Your Way Out of a Paper Bag”
//

use macroquad::prelude::*;
use macroquad_sandbox::quad::*;
use macroquad_sandbox::fluid::*;


const WINDOW_WIDTH: usize = 1024;
const WINDOW_HEIGHT: usize = 800;
const UNIVERSE_WIDTH: usize = 10;
const UNIVERSE_HEIGHT: usize = 10;


fn plot_universe(universe: &Universe) {
    let cell_dx =  screen_width() / universe.width() as f32;
    let cell_dy = screen_height() / universe.height() as f32;

    clear_background(BLACK);

    // Grid
    for i in 1..universe.width() {
        let x = i as f32 * cell_dx + 5.;
        draw_line(x, 0.0, x, screen_height(), 1.0, DARKGRAY)
    }
    for i in 1..universe.height() {
        let y = i as f32 * cell_dy + 5.;
        draw_line(0.0, y, screen_width(), y, 1.0, DARKGRAY)
    }

    // Densities
    for x in 0..universe.width() {
        for y in 0..universe.height() {
            let cx = x as f32 * cell_dx;
            let cy = y as f32 * cell_dy;
            let intensity = universe.density_at(x, y);
            let color = Color::new(intensity, intensity, intensity, 1.00);
            draw_rectangle(cx+1.0, cy+1.0, cell_dx-2.0, cell_dy-2.0, color);
        }
    }

    // Velocities
    for x in 0..universe.width() {
        for y in 0..universe.height() {
            let cx = cell_dx / 2. + x as f32 * cell_dx;
            let cy = cell_dy / 2. + y as f32 * cell_dy;
            let v = universe.velocity_at(x, y);
            draw_circle(cx, cy, 2., BLUE);
            draw_line(cx, cy, cx+v.x*cell_dx/2.0, cy+v.y*cell_dy/2.0, 1.0, RED);
        }
    }
}


fn window_conf() -> Conf {
    Conf {
        window_title: "Fluid sim".to_owned(),
        fullscreen: false,
        window_width: WINDOW_WIDTH as i32,
        window_height: WINDOW_HEIGHT as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let field = Universe::new(UNIVERSE_WIDTH, UNIVERSE_HEIGHT, Vec2d::new(1., 0.), 0.);

    loop {
        // Process input
        #[cfg(not(target_arch = "wasm32"))]
        if is_key_down(KeyCode::Q) | is_key_down(KeyCode::Escape) {
            break;
        }

        plot_universe(&field);
        
        next_frame().await
    }
}