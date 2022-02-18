// “Escape! Code Your Way Out of a Paper Bag”
//

use macroquad::prelude::*;
use mapgen::{
    MapBuilder,
    filter::{
        NoiseGenerator, 
        CellularAutomata,
        CullUnreachable,
        AreaStartingPosition,
        XStart, 
        YStart,
    },
    Map,
};


const WINDOW_WIDTH: usize = 800;
const WINDOW_HEIGHT: usize = 600;


fn draw_map(map: &Map) {
    let cell_dx = (WINDOW_WIDTH / map.width) as f32;
    let cell_dy = (WINDOW_HEIGHT / map.height) as f32;

    clear_background(LIGHTGRAY);
    // draw frames
    //draw cells
    for x in 0..map.width {
        for y in 0..map.height {
            let color = if map.at(x, y).is_blocked() { BLACK } else { WHITE };
            draw_rectangle(x as f32 * cell_dx, y as f32 * cell_dy, cell_dx, cell_dy, color);
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
    let map = MapBuilder::new(80, 60)
        .with(NoiseGenerator::uniform())
        .with(CellularAutomata::new())
        .with(AreaStartingPosition::new(XStart::CENTER, YStart::CENTER))
        .with(CullUnreachable::new())
        .build();  

    loop {
        // Process input
        #[cfg(not(target_arch = "wasm32"))]
        if is_key_down(KeyCode::Q) | is_key_down(KeyCode::Escape) {
            break;
        }

        // Draw world
        draw_map(&map);
        
        next_frame().await
    }
}