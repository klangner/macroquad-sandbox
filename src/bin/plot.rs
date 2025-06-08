// Plot chart
//
use macroquad::{prelude::*, ui};
use macroquad_sandbox::mqx::drawx::draw_rounded_rectangle;

const WINDOW_WIDTH: i32 = 1024;
const WINDOW_HEIGHT: i32 = 700;


struct Plot {
    title: String,
    pos: Vec2,
    size: Vec2,
}

impl Plot {
    fn new(title: &str, pos: Vec2, size: Vec2) -> Self {
        Self { title: title.to_owned(), pos, size }
    }

    fn plot(&self, _xs: &Vec<f32>) {
        draw_rounded_rectangle(self.pos, self.size, 10., WHITE);
        let min_x = 20.;
        let max_x = self.size.x - 20.;
        let min_y = 50.;
        let max_y = self.size.y - 20.;

        // Title
        ui::root_ui().label(
            vec2(self.pos.x + max_x / 2., self.pos.y +  10.), 
            &self.title);
        // x line
        draw_line(
            self.pos.x + min_x, 
            self.pos.y + max_y, 
            self.pos.x + max_x,
            self.pos.y + max_y, 
            1., 
            BLACK);
        // y line
        draw_line(
            self.pos.x + min_x, 
            self.pos.y + min_y, 
            self.pos.x + min_x,
            self.pos.y + max_y, 
            1., 
            BLACK);
    }
}


fn window_conf() -> Conf {
    Conf {
        window_title: "Plots".to_owned(),
        fullscreen: false,
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    let xs: Vec<f32> = (0..100).map(|x|10. * x as f32).collect();
    let height = WINDOW_HEIGHT as f32 / 2. - 15.;
    let plot1 = Plot::new(
        "Plot 1",
        Vec2::new(10., 10.), 
        Vec2::new(WINDOW_WIDTH as f32 - 20.,height));

    let plot2 = Plot::new(
        "Plot 2",
        Vec2::new(10., height + 20.), 
        Vec2::new(WINDOW_WIDTH as f32 - 20., height));

    loop {
        // Process input
        #[cfg(not(target_arch = "wasm32"))]
        if is_key_down(KeyCode::Q) | is_key_down(KeyCode::Escape) {
            break;
        }

        clear_background(SKYBLUE);

        plot1.plot(&xs);
        plot2.plot(&xs);

        next_frame().await
    }
}