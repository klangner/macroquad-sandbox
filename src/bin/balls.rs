// Balls colliding inside the box
//

use macroquad::prelude::*;
use ::rand::Rng;


const WINDOW_WIDTH: i32 = 1024;
const WINDOW_HEIGHT: i32 = 800;
const BOX_WIDTH: f32 = WINDOW_WIDTH as f32;
const BOX_HEIGHT: f32 = WINDOW_HEIGHT as f32;
const BALL_RADUIS: f32 = 20.0;
const NUM_BALLS: usize = 10;

const COLORS: [Color; 25] = [LIGHTGRAY, GRAY, DARKGRAY, YELLOW, GOLD, ORANGE, PINK, RED, MAROON, GREEN, 
                            LIME, DARKGREEN, SKYBLUE, BLUE, DARKBLUE, PURPLE, VIOLET, DARKPURPLE, BEIGE,
                            BROWN, DARKBROWN, WHITE, BLACK, BLANK, MAGENTA];


#[derive(Copy, Clone)]
pub struct Vec2d {
    pub x: f32,
    pub y: f32,
}

pub struct Ball {
    pub pos: Vec2d,
    pub velocity: Vec2d,
    pub radius: f32,
}

pub struct Universe {
    width: f32,
    height: f32,
    balls: Vec<Ball>,
}


impl Ball {
    pub fn new(pos: Vec2d, velocity: Vec2d, radius: f32) -> Self {
        Self {pos, velocity, radius}
    }

    pub fn new_random(width: f32, height: f32, radius: f32) -> Self {
        let mut rng = ::rand::thread_rng();
        let x = rng.gen_range(radius..(width-radius));
        let y = rng.gen_range(radius..(height-radius));
        let vx = rng.gen_range(-5.0..5.0);
        let vy = rng.gen_range(-5.0..5.0);
        Self { 
            pos: Vec2d { x, y }, 
            velocity: Vec2d { x: vx, y: vy }, 
            radius }
    }
}

impl Universe {

    pub fn random(width: f32, height: f32, num_balls: usize, ball_radius: f32) -> Self {
        let balls = (0..num_balls).map(|_| 
            Ball::new_random(width, height, ball_radius)
        ).collect();
        Self {width, height, balls}
    }
    
    pub fn balls(&self) -> &[Ball] {
        &self.balls
    }

    pub fn tick(&mut self) {
        for ball in self.balls.as_mut_slice() {
            ball.pos.x += ball.velocity.x;
            if ball.pos.x <= ball.radius || ball.pos.x >= self.width - ball.radius {
                ball.velocity.x = -ball.velocity.x;
            }
            ball.pos.y += ball.velocity.y;
            if ball.pos.y <= ball.radius || ball.pos.y >= self.height - ball.radius {
                ball.velocity.y = -ball.velocity.y;
            }

        }
    }
}


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
    let mut universe = Universe::random(BOX_WIDTH, BOX_HEIGHT, NUM_BALLS, BALL_RADUIS);

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
            let v = f32::abs(ball.velocity.x) + f32::abs(ball.velocity.y);
            let idx = usize::min(v as usize, COLORS.len());
            draw_circle(ball.pos.x, ball.pos.y, ball.radius, COLORS[idx]);
        }
        
        next_frame().await
    }
}