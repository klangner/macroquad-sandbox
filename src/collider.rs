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
    pub velocity: Vec2d,
    pub radius: f32,
}

pub struct Universe {
    width: f32,
    height: f32,
    balls: Vec<Ball>,
}


impl Ball {
    pub fn new(pos: Vec2d, velocity: Vec2d, radius: f32) -> Ball {
        Ball {pos, velocity, radius}
    }

    pub fn new_random(width: f32, height: f32, radius: f32) -> Ball {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(radius..(width-radius));
        let y = rng.gen_range(radius..(height-radius));
        let vx = rng.gen_range(-5.0..5.0);
        let vy = rng.gen_range(-5.0..5.0);
        Ball { 
            pos: Vec2d { x, y }, 
            velocity: Vec2d { x: vx, y: vy }, 
            radius }
    }
}

impl Universe {

    pub fn random(width: f32, height: f32, num_balls: usize, ball_radius: f32) -> Universe {
        let balls = (0..num_balls).map(|_| 
            Ball::new_random(width, height, ball_radius)
        ).collect();
        Universe {width, height, balls}
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
