// “Escape! Code Your Way Out of a Paper Bag”
//

use macroquad::prelude::*;
use macroquad_sandbox::quad::*;
use macroquad_sandbox::fluid::*;


const WINDOW_WIDTH: usize = 1024;
const WINDOW_HEIGHT: usize = 800;
const UNIVERSE_WIDTH: usize = 10;
const UNIVERSE_HEIGHT: usize = 10;


fn plot_velocities(universe: &Universe) {
    let margin: f32 = 5.;
    let max_width = screen_width() - 2. * margin;
    let max_height = screen_height() - 2. * margin;
    let cell_dx =  max_width / universe.width() as f32;
    let cell_dy = max_height / universe.height() as f32;

    for x in 0..universe.width() {
        for y in 0..universe.height() {
            let cx = margin + cell_dx / 2. + x as f32 * cell_dx;
            let cy = margin + cell_dy / 2. + y as f32 * cell_dy;
            draw_circle(cx, cy, 5., BLUE);
        }
    }
}

fn plot_densities(universe: &Universe) {

}

fn plot_universe(universe: &Universe) {
    plot_grid(universe.width(), universe.height());
    plot_densities(&universe);
    plot_velocities(&universe);
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