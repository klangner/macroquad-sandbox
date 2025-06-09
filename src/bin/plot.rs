// Plot chart
//
use macroquad::prelude::*;
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

    fn plot(&self, xs: &Vec<f32>, color: Color) {
        draw_rounded_rectangle(self.pos, self.size, 10., WHITE);
        let chart_area = Rect::new(
            self.pos.x + 20., 
            self.pos.y + 30., self.size.x - 40., self.size.y - 40.);

        // Title
        let text_dims = measure_text(&self.title, Option::None, 20, 1.0);
        draw_text(
            &self.title, 
            self.pos.x + (self.size.x - text_dims.width) / 2., 
            self.pos.y +  text_dims.height +  5., 
            20.0, BLACK);
        // x line
        draw_line(
            chart_area.x, 
            chart_area.y + chart_area.h, 
            chart_area.x + chart_area.w,
            chart_area.y + chart_area.h, 
            1., 
            BLACK);
        // // y line
        draw_line(
            chart_area.x, 
            chart_area.y, 
            chart_area.x,
            chart_area.y + chart_area.h, 
            1., 
            BLACK);

        // Plot points
        if xs.len() > 1 {
            let dx = chart_area.w / xs.len() as f32;
            let dy_min = xs.iter()
                .map(|&x| x.into())
                .reduce(f32::min)
                .unwrap();
            let dy_max = xs.iter()
                .map(|&x| x.into())
                .reduce(f32::max)
                .unwrap();
            let scale_y = chart_area.h / (dy_max - dy_min);

            let ps: Vec<Vec2> = xs.iter().enumerate()
                .map(|(i, x)| 
                    Vec2::new(
                        chart_area.x + i as f32 * dx, 
                        chart_area.y + chart_area.h - x * scale_y))
                .collect();

            for i in 1..ps.len() {
                let p1 = ps[i-1];
                let p2 = ps[i];
                draw_line(p1.x, p1.y, p2.x,p2.y, 1., color);
            }
        }
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

    let xs: Vec<f32> = (0..100).map(|x| 10. * x as f32).collect();
    let ys: Vec<f32> = (0..100).map(|x|f32::sin((x as f32) / 10.) + 1.).collect();
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

        plot1.plot(&xs, BLUE);
        plot2.plot(&ys, GREEN);

        next_frame().await
    }
}