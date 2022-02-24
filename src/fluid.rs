// Fluid sim implementation

#[derive(Copy, Clone)]
pub struct Vec2d {
    pub x: f32,
    pub y: f32,
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

    pub fn density_at(&self, x: usize, y: usize) -> f32 {
       self.densities[self.xy_idx(x, y)] 
    }

    pub fn velocity_at(&self, x: usize, y: usize) -> Vec2d {
       self.velocities[self.xy_idx(x, y)] 
    }

    fn xy_idx(&self, x: usize, y: usize) -> usize {
        y * self.width + x        
    }
}