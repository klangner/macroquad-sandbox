//! Colliding balls
//! 

use rand::Rng;


#[derive(Copy, Clone)]
pub struct Vec2d {
    pub x: f32,
    pub y: f32,
}

pub struct Ball {
    pub pos: Vec2d,
    velocity: Vec2d,
    pub radius: f32,
    pub type_id: usize,
}

pub struct Universe {
    width: f32,
    height: f32,
    balls: Vec<Ball>,
}


impl Ball {
    pub fn new(pos: Vec2d, velocity: Vec2d, radius: f32, type_id: usize) -> Ball {
        Ball { pos, velocity, radius, type_id }
    }

    pub fn new_random(width: f32, height: f32, radius: f32) -> Ball {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(radius..(width-radius));
        let y = rng.gen_range(radius..(height-radius));
        let vx = rng.gen_range(-5.0..5.0);
        let vy = rng.gen_range(-5.0..5.0);
        let type_id = rng.gen_range(0..25);
        Ball { 
            pos: Vec2d { x, y }, 
            velocity: Vec2d { x: vx, y: vy }, 
            radius,
            type_id }
    }
}

impl Universe {

    pub fn random(width: f32, height: f32, num_balls: usize) -> Universe {
        let balls = (0..num_balls).map(|_| 
            Ball::new_random(width, height, 5.0)
        ).collect();
        Universe {width, height, balls}
    }
    
    pub fn balls(&self) -> &[Ball] {
        &self.balls
    }

    pub fn tick(&mut self) {
        for ball in self.balls.as_mut_slice() {
            ball.pos.x += ball.velocity.x;
            if ball.pos.x <= 0.0 {
                ball.pos.x = -ball.pos.x;
                ball.velocity.x = -ball.velocity.x;
            } else if ball.pos.x >= self.width {
                ball.pos.x = 2.0 * self.width - ball.pos.x;
                ball.velocity.x = -ball.velocity.x;
            }
            ball.pos.y += ball.velocity.y;
            if ball.pos.y <= 0.0 {
                ball.pos.y = -ball.pos.y;
                ball.velocity.y = -ball.velocity.y;
            } else if ball.pos.y >= self.height {
                ball.pos.y = 2.0 * self.height - ball.pos.y;
                ball.velocity.y = -ball.velocity.y;
            }
        }
    }
}
