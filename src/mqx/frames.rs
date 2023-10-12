// 2d frames from source image.
//

use macroquad::prelude::*;


pub struct Frames {
    texture: Texture2D,
    num_rows: i32,
    num_cols: i32,
}

impl Frames {
    pub fn new(texture: Texture2D, num_rows: i32, num_cols: i32) -> Frames {
        Frames {
            texture,
            num_rows,
            num_cols,
        }
    }

    pub fn draw(&self, pos: Vec2, row: i32, col: i32) {
        let frame_width = self.texture.width() / self.num_rows as f32;
        let frame_height = self.texture.height() / self.num_cols as f32;
        let params = DrawTextureParams {
                dest_size: None,
                source: Some(Rect::new(row as f32 * frame_width, col as f32 * frame_height, frame_width, frame_height)),
                rotation: 0.,
                pivot: None,
                flip_x: false,
                flip_y: false,
            };
        draw_texture_ex(&self.texture, pos.x, pos.y, WHITE, params);
    }
}