// Balls colliding inside the box
//

use macroquad::prelude::*;
use macroquad_sandbox::collider::*;


const WINDOW_WIDTH: i32 = 1024;
const WINDOW_HEIGHT: i32 = 800;
const BOX_WIDTH: f32 = WINDOW_WIDTH as f32;
const BOX_HEIGHT: f32 = WINDOW_HEIGHT as f32;
// const BALL_RADUIS: f32 = 5.0;
const NUM_BALLS: usize = 100;

const COLORS: [Color; 25] = [LIGHTGRAY, GRAY, DARKGRAY, YELLOW, GOLD, ORANGE, PINK, RED, MAROON, GREEN, 
                            LIME, DARKGREEN, SKYBLUE, BLUE, DARKBLUE, PURPLE, VIOLET, DARKPURPLE, BEIGE,
                            BROWN, DARKBROWN, WHITE, BLACK, BLANK, MAGENTA];


fn window_conf() -> Conf {
    Conf {
        window_title: "Colliding balls".to_owned(),
        fullscreen: false,
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut universe = Universe::random(BOX_WIDTH, BOX_HEIGHT, NUM_BALLS);

    loop {
        universe.tick();

        // Process input
        #[cfg(not(target_arch = "wasm32"))]
        if is_key_down(KeyCode::Q) | is_key_down(KeyCode::Escape) {
            break;
        }

        // Draw universe
        clear_background(WHITE);
        for ball in universe.balls() {
            draw_circle(ball.pos.x, ball.pos.y, ball.radius, COLORS[ball.type_id]);
        }
        
        next_frame().await
    }
}