// turn based strategy
//

use macroquad::prelude::*;
use macroquad_sandbox::mqx::drawx;
use macroquad_sandbox::ufo::{WorldMap};


const WINDOW_WIDTH: i32 = 1024;
const WINDOW_HEIGHT: i32 = 800;


fn window_conf() -> Conf {
    Conf {
        window_title: "turn based strategy game".to_owned(),
        fullscreen: false,
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    let map =  WorldMap::new(64, 64);

    loop {
        // let dt = get_frame_time();
        // Process input
        #[cfg(not(target_arch = "wasm32"))]
        if is_key_down(KeyCode::Q) | is_key_down(KeyCode::Escape) {
            break;
        }

        clear_background(GRAY);
        drawx::plot_grid(map.width, map.height);

        next_frame().await
    }
}