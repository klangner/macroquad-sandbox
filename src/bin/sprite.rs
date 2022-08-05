// Balls colliding inside the box
//

use macroquad::prelude::*;


const WINDOW_WIDTH: i32 = 1024;
const WINDOW_HEIGHT: i32 = 800;


fn draw_frame(texture: Texture2D, pos: Vec2, frame_id: i32) {
    let r = frame_id / 5;
    let c = frame_id % 5;
    let params = DrawTextureParams {
            dest_size: None,
            source: Some(Rect::new(c as f32 * 184., r as f32 * 184., 184., 184.)),
            rotation: 0.,
            pivot: None,
            flip_x: false,
            flip_y: false,
        };
    draw_texture_ex(texture, pos.x, pos.y, WHITE, params);
}


fn window_conf() -> Conf {
    Conf {
        window_title: "Animated Sprite".to_owned(),
        fullscreen: false,
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    let texture = load_texture("assets/flapping_bird.png").await.unwrap();
    let pos = vec2(((WINDOW_WIDTH - 184) / 2) as f32, ((WINDOW_HEIGHT - 184) / 2) as f32);
    let mut frame = 11.;

    loop {

        // Process input
        #[cfg(not(target_arch = "wasm32"))]
        if is_key_down(KeyCode::Q) | is_key_down(KeyCode::Escape) {
            break;
        }

        // Draw universe
        clear_background(WHITE);

        draw_frame(texture, pos, frame as i32);
        frame += 0.07;
        if frame > 20.0 {
            frame = 11.;
        }
        
        next_frame().await
    }
}