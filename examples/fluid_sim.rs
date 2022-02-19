// “Escape! Code Your Way Out of a Paper Bag”
//

use std::time::Instant;
use macroquad::prelude::*;


const WINDOW_WIDTH: usize = 1024;
const WINDOW_HEIGHT: usize = 800;


fn draw_grid(num_cols: usize, num_rows: usize) {
    let cell_dx = (WINDOW_WIDTH / num_cols) as f32;
    let cell_dy = (WINDOW_HEIGHT / num_rows) as f32;

    clear_background(LIGHTGRAY);
    for x in 0..num_cols {
        for y in 0..num_rows {
            let color = BLACK;
            draw_rectangle(
                x as f32 * cell_dx, 
                y as f32 * cell_dy, 
                cell_dx, 
                cell_dy, color);
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

        // Draw world
        draw_grid(10, 10);
        
        next_frame().await
    }
}