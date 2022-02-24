// “Escape! Code Your Way Out of a Paper Bag”
//

use macroquad::prelude::*;
use macroquad_sandbox::fluid::*;


const WINDOW_WIDTH: i32 = 1024;
const WINDOW_HEIGHT: i32 = 800;
const UNIVERSE_WIDTH: i32 = 10;
const UNIVERSE_HEIGHT: i32 = 10;


fn plot_universe(universe: &Universe) {
    let cell_dx =  screen_width() / universe.width() as f32;
    let cell_dy = screen_height() / universe.height() as f32;

    clear_background(BLACK);

    // Densities
    for x in 0..universe.width() {
        for y in 0..universe.height() {
            let cx = x as f32 * cell_dx;
            let cy = y as f32 * cell_dy;
            let intensity = universe.density_at(x, y);
            let color = Color::new(intensity, intensity, intensity, 1.00);
            draw_rectangle(cx, cy, cell_dx, cell_dy, color);
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
    
    // Grid
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
    let mut universe = Universe::new(UNIVERSE_WIDTH, UNIVERSE_HEIGHT, Vec2d::new(1., 0.), 0.);
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
            let x = f32::trunc(mouse_x / cell_dx) as usize;
            let y = f32::trunc(mouse_y / cell_dy) as usize;
            universe.increase_density(x as i32, y as i32, dt / 1.0);
        }

        // Plot
        plot_universe(&universe);
        
        next_frame().await
    }
}