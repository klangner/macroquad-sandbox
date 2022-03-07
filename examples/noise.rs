// “Escape! Code Your Way Out of a Paper Bag”
//

use macroquad::prelude::*;


const MAP_WIDTH: usize = 512;
const MAP_HEIGHT: usize = 512;


fn draw_map() {
    clear_background(WHITE);
}


fn window_conf() -> Conf {
    Conf {
        window_title: "Fluid sim".to_owned(),
        fullscreen: false,
        window_width: MAP_WIDTH as i32,
        window_height: MAP_HEIGHT as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let image = Image::gen_image_color(MAP_WIDTH as u16, MAP_HEIGHT as u16, RED);
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