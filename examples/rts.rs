// “Escape! Code Your Way Out of a Paper Bag”
//

use macroquad::prelude::*;
use macroquad_sandbox::rts::*;
use mapgen::{Map, MapBuilder, filter::*};


const SCREEN_WIDTH: usize = 1200;
const SCREEN_HEIGHT: usize = 900;


fn random_map(ncols: usize, nrows: usize) -> Map {
    MapBuilder::new(ncols, nrows)
        .with(NoiseGenerator::uniform())
        .with(CellularAutomata::new())
        .with(AreaStartingPosition::new(XStart::CENTER, YStart::CENTER))
        .with(CullUnreachable::new())
        .with(DistantExit::new())
        .build()
}

fn draw(universe: &Universe) {
    let cell_dx = (SCREEN_WIDTH / universe.map.width) as f32;
    let cell_dy = (SCREEN_HEIGHT / universe.map.height) as f32;

    clear_background(LIGHTGRAY);
    for x in 0..universe.map.width {
        for y in 0..universe.map.height {
            let color = if universe.map.at(x, y).is_blocked { DARKGRAY } else { WHITE };
            draw_rectangle(
                x as f32 * cell_dx, 
                y as f32 * cell_dy, 
                cell_dx, 
                cell_dy, color);
        }
    }

    if universe.path.len() > 1 {
        let mut last_x = universe.path[0].x * cell_dx;
        let mut last_y = universe.path[0].y * cell_dy;
        for i in 1..universe.path.len() {
            let x = universe.path[i].x * cell_dx;
            let y = universe.path[i].y * cell_dy;
            draw_line(last_x, last_y, x, y, 1., GREEN);
            last_x = x;
            last_y = y;
        }
    }

    for unit in &universe.units {
        draw_circle(unit.pos.x * cell_dx, unit.pos.y * cell_dy, 5.0, BLUE);
    }
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
    let map = random_map(80, 60);
    let data: Vec<bool> = map.tiles.iter().map(|&t| t.is_blocked()).collect();
    let world_map = WorldMap::from_data(map.width, map.height, &data);
    let mut universe = Universe::from_map(world_map);
    let sp = map.starting_point.unwrap();
    universe.add_unit(sp.x, sp.y);

    loop {
        if is_key_down(KeyCode::Q) | is_key_down(KeyCode::Escape) {
            break;
        }

        universe.tick();

        // Process mouse
        if is_mouse_button_pressed(MouseButton::Left) {
            let (x, y) = mouse_position();
            universe.move_to(x as usize * 80 / SCREEN_WIDTH , y as usize * 60 / SCREEN_HEIGHT);
        }

        draw(&universe);

        next_frame().await
    }
}