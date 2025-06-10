// Plot chart
//
use macroquad::prelude::*;
use macroquad_sandbox::mqx::plot::Plot;

const WINDOW_WIDTH: i32 = 1024;
const WINDOW_HEIGHT: i32 = 700;


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