// “Escape! Code Your Way Out of a Paper Bag”
//

use macroquad::prelude::*;
use macroquad::prelude::Color;
use noise::{utils::*, Perlin, Seedable};


const MAP_WIDTH: u32 = 512;
const MAP_HEIGHT: u32 = 512;


fn draw_map() {
    clear_background(WHITE);
}


fn window_conf() -> Conf {
    Conf {
        window_title: "Noise map".to_owned(),
        fullscreen: false,
        window_width: MAP_WIDTH as i32,
        window_height: MAP_HEIGHT as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let perlin = Perlin::default();
    let perlin = perlin.set_seed(10);

    let noise = PlaneMapBuilder::new(&perlin)
        .set_size(MAP_WIDTH as usize, MAP_HEIGHT as usize)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build();
    let mut image = Image::gen_image_color(MAP_WIDTH as u16, MAP_HEIGHT as u16, BLACK);
    for i in 0..MAP_WIDTH {
        for j in 0..MAP_HEIGHT {
            let c = noise.get_value(i as usize, j as usize) as f32;
            image.set_pixel(i, j, Color::new(c, c, c, 1.));
        }
    }
    let texture = Texture2D::from_image(&image);

    loop {
        // Process input
        #[cfg(not(target_arch = "wasm32"))]
        if is_key_down(KeyCode::Q) | is_key_down(KeyCode::Escape) {
            break;
        }

        draw_map();
        draw_texture(texture, 0., 0., WHITE);
        
        next_frame().await
    }
}