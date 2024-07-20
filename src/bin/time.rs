// Observe passing time ;-)
//

use macroquad::{prelude::*, ui};


const WINDOW_WIDTH: i32 = 1024;
const WINDOW_HEIGHT: i32 = 800;


#[derive(Copy, Clone)]
enum TimeSpeed {
    Pause,
    Slow,
    Normal,
    Fast,
    VeryFast,
}


#[derive(Copy, Clone)]
pub struct Time {
    current_time_ms: i64,
    speed: TimeSpeed,
}

impl Time {
    pub fn new() -> Self {
        Self {current_time_ms: 0, speed: TimeSpeed::Normal}
    }
    
    pub fn tick(&mut self, dt: f32) {
        let speed = match self.speed {
            TimeSpeed::Pause => 0.,
            TimeSpeed::Slow => 0.5,
            TimeSpeed::Normal => 1.,
            TimeSpeed::Fast => 2.,
            TimeSpeed::VeryFast => 5.,
        };

        self.current_time_ms += (1000.0 * dt * speed) as i64 ;
    }

    pub fn current_time_sec(&self) -> i32 {
        self.current_time_ms as i32 / 1000
    }

    pub fn hour(&self) -> i32 {
        self.current_time_sec() / (60 * 60)
    }

    pub fn minute(&self) -> i32 {
        (self.current_time_sec() / 60) % 60
    }

    pub fn second(&self) -> i32 {
        self.current_time_sec() % 60
    }
}


struct TimeWidget {}

impl TimeWidget {
    fn new() -> Self {
        Self {}
    }

    fn draw(&self, time: &Time, font: &Font) {
        draw_text_ex(
            &format!("{}:{:02}:{:02}", time.hour(), time.minute(), time.second()),
            400.0,
            70.0,
            TextParams {
                font_size: 50,
                font: Some(font),
                ..Default::default()
            },
        );

        if ui::root_ui().button(Vec2::new(400.0, 100.0), "||") {
            println!("Paused");
        }
        if ui::root_ui().button(Vec2::new(450.0, 100.0), "|>") {
            println!("Slow");
        }
        if ui::root_ui().button(Vec2::new(500.0, 100.0), ">") {
            println!("Normal");
        }
        if ui::root_ui().button(Vec2::new(550.0, 100.0), ">>") {
            println!("Fast");
        }
        if ui::root_ui().button(Vec2::new(600.0, 100.0), ">>>") {
            println!("Very fast");
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "The Time".to_owned(),
        fullscreen: false,
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut time = Time::new();

    let font = load_ttf_font("assets/FiraMono-subset.ttf").await.unwrap();
    let time_widget = TimeWidget::new();

    loop {
        let dt = get_frame_time();
        time.tick(dt);

        // Process input
        #[cfg(not(target_arch = "wasm32"))]
        if is_key_down(KeyCode::Q) | is_key_down(KeyCode::Escape) {
            break;
        }

        // Draw universe
        clear_background(DARKGRAY);
        time_widget.draw(&time, &font);
        
        next_frame().await
    }
}