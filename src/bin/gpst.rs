// GPS Triangle racing
//
// TODO:
// * Move to separate project
// * Show core metrics only: Speed, variometer, altitude
// * Show flight trace
// * Add thermals (visualize them with background color)
// * Apply thermals
// * Working elevator
// * Aplying ailerons (bank will increase sink)
// * Stop when altidute = 0

use std::f32::consts::PI;

use macroquad::{prelude::*, ui};
use macroquad_sandbox::mqx::drawx;


const WINDOW_WIDTH: i32 = 1024;
const WINDOW_HEIGHT: i32 = 700;
const LABEL_FONT_SIZE: u16 = 20;

const BACKGROUND_COLOR: Color = color_u8!(0xcd, 0xe2, 0xf9, 255);
const SIDEBAR_COLOR: Color = color_u8!(0x15, 0x83, 0xd7, 255);
const PANEL_COLOR: Color = color_u8!(0x62, 0xaa, 0xea, 255);


struct Glider {
    // body: GliderBody,
    pos: Vec2,  // meters
    velocity: f32, // m/s
    orientation: Vec2, // radians
    altitude: f32,
    hvelocity: f32, // m/s
}

impl Glider {
    pub fn new() -> Self {
        Self {
            pos: vec2(150., 100.),
            velocity: 10.,
            orientation: vec2(0., 1.),
            altitude: 100.,
            hvelocity: -1.,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.pos +=  dt * self.velocity * self.orientation;
        self.altitude += self.hvelocity * dt;
    }

    pub fn turn_left(&mut self, dt: f32) {
        let rhs = Vec2::from_angle(-0.5 * dt);
        self.orientation = self.orientation.rotate(rhs);
    }

    pub fn turn_right(&mut self, dt: f32) {
        let rhs = Vec2::from_angle(0.5 * dt);
        self.orientation = self.orientation.rotate(rhs);
    }
}


struct FlyingField {
    area_size: Vec2,
    tp1: Vec2,
    tp2: Vec2,
    tp3: Vec2,
}

impl FlyingField {
    pub fn new() -> Self {
        let area_size = vec2(700., 400.);
        Self { 
            area_size,
            tp1: vec2(550., 300.),
            tp2: vec2(350., 100.),
            tp3: vec2(150., 300.),
         }
    }
}

struct MapView {
    pos: Vec2,
    size: Vec2,
    flying_field: FlyingField,
    glider_image: Texture2D,
}

impl MapView {
    pub fn new(pos: Vec2, size: Vec2, flying_field: FlyingField, glider_image: Texture2D) -> Self {
        Self { pos, size, flying_field, glider_image }
    }

    pub fn draw(&self, glider: &Glider) {
        let area =  &self.flying_field.area_size;
        self.draw_tp(&self.flying_field.tp1, area);
        self.draw_tp(&self.flying_field.tp2, area);
        self.draw_tp(&self.flying_field.tp3, area);

        let params = DrawTextureParams {
            rotation: glider.orientation.to_angle(),
            ..Default::default()
        };

        let scale_x = self.size.x / area.x;
        let scale_y = self.size.y / area.y;
        let glider_x = glider.pos.x * scale_x;
        let glider_y = glider.pos.y * scale_y;
        draw_texture_ex(&self.glider_image, glider_x, glider_y, WHITE, params);
    }

    pub fn draw_tp(&self, tp: &Vec2, area: &Vec2) {
        let scale_x = self.size.x / area.x;
        let scale_y = self.size.y / area.y;
        let x = self.pos.x + tp.x * scale_x;
        let y = self.pos.y + tp.y * scale_y;
        draw_circle(x, y, 10., ORANGE);
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
            &format!("Position: {:.1} m, {:.1} m", glider.pos.x, glider.pos.y));
        ui::root_ui().label(
            vec2(self.pos.x + 10., self.pos.y + 10. + hoffset), 
            &format!("Altitude: {:.1} m", glider.altitude));
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

    fn draw(&self, glider: &Glider) {
        // let hoffset = LABEL_FONT_SIZE as f32;

        drawx::draw_rounded_rectangle(self.pos, self.size, 10., PANEL_COLOR); 
        
        ui::root_ui().label(
            vec2(self.pos.x + 10., self.pos.y + 10.), 
            &format!("Yaw: {:.2} deg", glider.orientation.to_angle() * 180. / PI));
        // ui::root_ui().label(
        //     vec2(self.pos.x + 10., self.pos.y + 10. + hoffset), 
        //     &format!("Roll: {} deg", glider.orientation.y));
        // ui::root_ui().label(
        //     vec2(self.pos.x + 10., self.pos.y + 10. + 2.*hoffset), 
        //     &format!("Pitch: {} deg", glider.orientation.z));
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

    fn draw(&self, glider: &Glider) {
        let hoffset = LABEL_FONT_SIZE as f32;

        drawx::draw_rounded_rectangle(self.pos, self.size, 10., PANEL_COLOR); 
        
        ui::root_ui().label(
            vec2(self.pos.x + 10., self.pos.y + 10.), 
            &format!("speed: {} m/s ({} km/h)", glider.velocity, glider.velocity * 3.6));
        ui::root_ui().label(
            vec2(self.pos.x + 10., self.pos.y + 10. + hoffset), 
            &format!("variometer: {} m/s", glider.hvelocity));
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
    let flying_field = FlyingField::new();
    // UI
    let glider_image = load_texture("assets/glider.png").await.unwrap();
    let map = MapView::new(
        vec2(0., 0.), 
        vec2(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32 - 100.),
        flying_field,
        glider_image);
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
        vec2(300., 95.));
    let orientation_panel = OrientationPanel::new(
        vec2(310., WINDOW_HEIGHT as f32 - 100.),
        vec2(300., 95.));
    let velocity_panel = VelocityPanel::new(
        vec2(615., WINDOW_HEIGHT as f32 - 100.),
        vec2(WINDOW_WIDTH as f32 - 620., 95.));

    loop {
        let dt = get_frame_time();
        glider.update(dt);

        // Process input
        #[cfg(not(target_arch = "wasm32"))]
        if is_key_down(KeyCode::Q) | is_key_down(KeyCode::Escape) {
            break;
        }

        if is_key_down(KeyCode::Left) {
            glider.turn_left(dt);
        }
        if is_key_down(KeyCode::Right) {
            glider.turn_right(dt);
        }
        // Draw universe
        clear_background(BACKGROUND_COLOR);
        draw_rectangle(0., WINDOW_HEIGHT as f32 - 105., WINDOW_WIDTH as f32, 120., SIDEBAR_COLOR);

        map.draw(&glider);
        orientation_panel.draw(&glider);
        position_panel.draw(&glider);
        velocity_panel.draw(&glider);
        
        next_frame().await
    }
}