// GPS Triangle racing
//

use macroquad::{prelude::*, ui};
use macroquad_sandbox::mqx::drawx;


const WINDOW_WIDTH: i32 = 1024;
const WINDOW_HEIGHT: i32 = 700;
const LABEL_FONT_SIZE: u16 = 20;

const BACKGROUND_COLOR: Color = color_u8!(0xcd, 0xe2, 0xf9, 255);
const SIDEBAR_COLOR: Color = color_u8!(0x15, 0x83, 0xd7, 255);
const PANEL_COLOR: Color = color_u8!(0x62, 0xaa, 0xea, 255);


struct Glider {
    pos: Vec3,  // meters

}

impl Glider {
    pub fn new() -> Self {
        Self {
            pos: vec3(0., 0. ,100.),
        }
    }

    pub fn update(&mut self, _dt: f32) {

    }
}


struct PositionPanel {
    pos: Vec2,
    size: Vec2,
}

impl PositionPanel {
    fn new(pos: Vec2, size: Vec2) -> Self {
        Self { pos, size }
    }

    fn draw(&self, glider: &Glider) {
        let hoffset = LABEL_FONT_SIZE as f32;

        drawx::draw_rounded_rectangle(self.pos, self.size, 10., PANEL_COLOR); 
        
        ui::root_ui().label(
            vec2(self.pos.x + 10., self.pos.y + 10.), 
            &format!("Position: {}m, {}m", glider.pos.x, glider.pos.y));
        ui::root_ui().label(
            vec2(self.pos.x + 10., self.pos.y + 10. + hoffset), 
            &format!("Altitude: {}m", glider.pos.z));
    }
}

struct OrientationPanel {
    pos: Vec2,
    size: Vec2,
}

impl OrientationPanel {
    fn new(pos: Vec2, size: Vec2) -> Self {
        Self { pos, size }
    }

    fn draw(&self, _glider: &Glider) {
        let hoffset = LABEL_FONT_SIZE as f32;

        drawx::draw_rounded_rectangle(self.pos, self.size, 10., PANEL_COLOR); 
        
        ui::root_ui().label(
            vec2(self.pos.x + 10., self.pos.y + 10.), 
            &format!("Yaw: {} deg", 0));
        ui::root_ui().label(
            vec2(self.pos.x + 10., self.pos.y + 10. + hoffset), 
            &format!("Roll: {} deg", 0));
        ui::root_ui().label(
            vec2(self.pos.x + 10., self.pos.y + 10. + 2.*hoffset), 
            &format!("Pitch: {} deg", 0));
    }
}

struct VelocityPanel {
    pos: Vec2,
    size: Vec2,
}

impl VelocityPanel {
    fn new(pos: Vec2, size: Vec2) -> Self {
        Self { pos, size }
    }

    fn draw(&self, _glider: &Glider) {
        let hoffset = LABEL_FONT_SIZE as f32;

        drawx::draw_rounded_rectangle(self.pos, self.size, 10., PANEL_COLOR); 
        
        ui::root_ui().label(
            vec2(self.pos.x + 10., self.pos.y + 10.), 
            &format!("vspeed: {} m/s", 0));
        ui::root_ui().label(
            vec2(self.pos.x + 10., self.pos.y + 10. + hoffset), 
            &format!("hspeed: {} m/s", 0));
    }
}

struct DebugPanel {
    pos: Vec2,
    size: Vec2,
}

impl DebugPanel {
    fn new(pos: Vec2, size: Vec2) -> Self {
        Self { pos, size }
    }

    fn draw(&self, _glider: &Glider) {
        drawx::draw_rounded_rectangle(self.pos, self.size, 10., PANEL_COLOR); 
        
        ui::root_ui().label(
            vec2(self.pos.x + 10., self.pos.y + 10.), 
            &format!("Info: "));
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "GPS Triangle Racing".to_owned(),
        fullscreen: false,
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // Sim
    let mut glider = Glider::new();
    // UI
    let label_style = ui::root_ui()
        .style_builder()
        .font_size(LABEL_FONT_SIZE)
        .build();
    let ui_skin = ui::Skin {
        label_style,
        ..ui::root_ui().default_skin()
    };
    ui::root_ui().push_skin(&ui_skin);

    let position_panel = PositionPanel::new(
        vec2(5., WINDOW_HEIGHT as f32 - 100.),
        vec2(200., 95.));
    let orientation_panel = OrientationPanel::new(
        vec2(210., WINDOW_HEIGHT as f32 - 100.),
        vec2(200., 95.));
    let velocity_panel = VelocityPanel::new(
        vec2(415., WINDOW_HEIGHT as f32 - 100.),
        vec2(200., 95.));
    let debug_panel = DebugPanel::new(
        vec2(620., WINDOW_HEIGHT as f32 - 100.),
        vec2(WINDOW_WIDTH as f32 - 625., 95.));

    loop {
        let dt = get_frame_time();
        glider.update(dt);

        // Process input
        #[cfg(not(target_arch = "wasm32"))]
        if is_key_down(KeyCode::Q) | is_key_down(KeyCode::Escape) {
            break;
        }

        // Draw universe
        clear_background(BACKGROUND_COLOR);
        draw_rectangle(0., WINDOW_HEIGHT as f32 - 105., WINDOW_WIDTH as f32, 120., SIDEBAR_COLOR);
        orientation_panel.draw(&glider);
        position_panel.draw(&glider);
        velocity_panel.draw(&glider);
        debug_panel.draw(&glider);
        
        next_frame().await
    }
}