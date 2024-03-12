// Balls colliding inside the box
//

use macroquad::prelude::*;


const WINDOW_WIDTH: i32 = 1024;
const WINDOW_HEIGHT: i32 = 800;


struct FrameSeq {
    pub start: i32,
    pub end: i32,
}

struct Frames {
    texture: Texture2D,
    num_cols: i32,
    sequences: Vec<FrameSeq>,
}

struct Sprite<'a> {
    frames: &'a Frames,
    fps: f32,
    time: f32,
}

impl FrameSeq {
    fn new(start: i32, end: i32) -> FrameSeq {
        FrameSeq { start, end }
    }
}

impl Frames {
    pub fn new(texture: Texture2D, num_cols: i32, sequences: Vec<FrameSeq>) -> Self {
        Self {
            texture,
            num_cols,
            sequences
        }
    }

    fn draw(&self, pos: Vec2, seq_id: usize, frame: i32) {
        let seq = self.sequences.get(seq_id).unwrap();
        let frame_id = frame % (seq.end - seq.start) + seq.start;
        let r = frame_id / self.num_cols;
        let c = frame_id % self.num_cols;
        let params = DrawTextureParams {
                dest_size: None,
                source: Some(Rect::new(c as f32 * 184., r as f32 * 184., 184., 184.)),
                rotation: 0.,
                pivot: None,
                flip_x: false,
                flip_y: false,
            };
        draw_texture_ex(&self.texture, pos.x, pos.y, WHITE, params);
    }
}

impl<'a> Sprite<'a> {
    fn new(frames: &'a Frames, fps: f32) -> Self {
        Self { frames, fps, time: 0. }
    }

    fn draw(&mut self, pos: Vec2, dt: f32) {
        self.time += dt;
        let frame_id = (self.time * self.fps) as i32;
        self.frames.draw(pos, 0, frame_id); 
    }
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
    let frames = Frames::new(texture, 5, 
        vec![FrameSeq::new(11, 20), FrameSeq::new(21, 23)]);
    let mut sprite = Sprite::new(&frames, 20.);
    let pos = vec2(((WINDOW_WIDTH - 184) / 2) as f32, ((WINDOW_HEIGHT - 184) / 2) as f32);

    loop {
        let dt = get_frame_time();
        // Process input
        #[cfg(not(target_arch = "wasm32"))]
        if is_key_down(KeyCode::Q) | is_key_down(KeyCode::Escape) {
            break;
        }

        // Draw universe
        clear_background(WHITE);

        sprite.draw(pos, dt);
        next_frame().await
    }
}