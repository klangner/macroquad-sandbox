// Fluid sim implementation

#[derive(Clone)]
pub struct Vec2d {
    x: f32,
    y: f32,
}

pub struct Universe {
    width: usize,
    height: usize,
    velocities: Vec<Vec2d>,
    densities: Vec<f32>
}


impl Vec2d {
    pub fn new (x: f32, y: f32) -> Vec2d {
        Vec2d { x, y }
    }
}

impl Universe {
    pub fn new(width: usize, height: usize, velocity: Vec2d, density: f32) -> Universe {
        Universe { 
            width,
            height,
            velocities: vec![velocity; width*height], 
            densities: vec![density; width*height],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}