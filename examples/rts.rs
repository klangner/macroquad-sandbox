// “Escape! Code Your Way Out of a Paper Bag”
//

use macroquad::prelude::*;
use macroquad_sandbox::rts::*;
use mapgen::{MapBuilder, filter::*};


const SCREEN_WIDTH: usize = 1200;
const SCREEN_HEIGHT: usize = 900;


fn draw(universe: &Universe) {
    let cell_dx = (SCREEN_WIDTH / universe.map.width) as f32;
    let cell_dy = (SCREEN_HEIGHT / universe.map.height) as f32;

    clear_background(LIGHTGRAY);
    for x in 0..universe.map.width {
        for y in 0..universe.map.height {
            let color = if universe.map.at(x, y).is_blocked() { DARKGRAY } else { WHITE };
            draw_rectangle(
                x as f32 * cell_dx, 
                y as f32 * cell_dy, 
                cell_dx, 
                cell_dy, color);
        }
    }

    draw_circle(universe.unit.pos.x * cell_dx, universe.unit.pos.y * cell_dy, 5.0, BLUE);
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
        let map = MapBuilder::new(80, 60)
            .with(NoiseGenerator::uniform())
            .with(CellularAutomata::new())
            .with(AreaStartingPosition::new(XStart::CENTER, YStart::CENTER))
            .with(CullUnreachable::new())
            .with(DistantExit::new())
            .build();

    let mut universe = Universe::from_map(map);

    loop {
        if is_key_down(KeyCode::Q) | is_key_down(KeyCode::Escape) {
            break;
        }

        universe.tick();

        // Process mouse
        if is_mouse_button_down(MouseButton::Left) {
            let (x, y) = mouse_position();
            universe.move_to(x as usize * 80 / SCREEN_WIDTH , y as usize * 60 / SCREEN_HEIGHT);
        }

        draw(&universe);

        next_frame().await
    }
}