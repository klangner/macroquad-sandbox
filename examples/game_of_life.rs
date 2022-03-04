// “Escape! Code Your Way Out of a Paper Bag”
//

use macroquad::prelude::*;
use macroquad_sandbox::gol::*;


const WINDOW_WIDTH: usize = 1024;
const WINDOW_HEIGHT: usize = 800;
const UNIVERSE_WIDTH: usize = WINDOW_HEIGHT / 4;
const UNIVERSE_HEIGHT: usize = WINDOW_HEIGHT / 4;


fn draw_universe(universe: &Universe) {
    let cell_dx =  screen_width() / universe.width() as f32;
    let cell_dy = screen_height() / universe.height() as f32;

    clear_background(WHITE);

    // Densities
    for r in 0..universe.height() {
        for c in 0..universe.width() {
            if universe.cell_at(r, c) == Cell::Alive {
                let x = c as f32 * cell_dx;
                let y = r as f32 * cell_dy;
                draw_rectangle( x, y, cell_dx, cell_dy, BLACK);
            }
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
    let mut universe = Universe::random(UNIVERSE_WIDTH, UNIVERSE_HEIGHT, 0.5);

    loop {
        universe.tick();

        // Process input
        #[cfg(not(target_arch = "wasm32"))]
        if is_key_down(KeyCode::Q) | is_key_down(KeyCode::Escape) {
            break;
        }

        draw_universe(&universe);
        
        next_frame().await
    }
}