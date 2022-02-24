// Fluid sim implementation

#[derive(Copy, Clone)]
pub struct Vec2d {
    pub x: f32,
    pub y: f32,
}

pub struct Universe {
    width: i32,
    height: i32,
    velocities: Vec<Vec2d>,
    densities: Vec<f32>,
    diffusion_rate: f32,
}


impl Vec2d {
    pub fn new (x: f32, y: f32) -> Vec2d {
        Vec2d { x, y }
    }
}

impl Universe {
    pub fn new(width: i32, height: i32, velocity: Vec2d, density: f32) -> Universe {
        Universe { 
            width,
            height,
            velocities: vec![velocity; (width*height) as usize], 
            densities: vec![density; (width*height) as usize],
            diffusion_rate: 0.01,
        }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn density_at(&self, x: i32, y: i32) -> f32 {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return 0.0
        }
       self.densities[self.xy_idx(x, y)] 
    }

    pub fn increase_density(&mut self, x: i32, y: i32, amount: f32) {
        let idx = self.xy_idx(x, y);
        self.densities[idx] += amount;
        if self.densities[idx] > 1.0 {
            self.densities[idx] = 1.0;
        }

    }

    pub fn velocity_at(&self, x: i32, y: i32) -> Vec2d {
       self.velocities[self.xy_idx(x, y)] 
    }

    // Calculate diffusion using
    // Gauss-Seidel relaxation method for numerical stability
    pub fn diffuse(&mut self, dt: f32) {
        let a = dt * self.diffusion_rate * (self.width * self.height) as f32;
        let d0 = self.densities.clone();

        for _ in 0..20 {
            for x in 0..self.width as i32 {
                for y in 0..self.height as i32 {
                    let idx = self.xy_idx(x, y);
                    self.densities[idx] = (
                        d0[idx] + 
                        a*(self.density_at(x-1,y) + 
                        self.density_at(x+1,y) + 
                        self.density_at(x,y-1) +
                        self.density_at(x,y+1))
                    )/(1.0 + 4.0 * a)
               }
            }
        }
    }

    fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y * self.width + x) as usize
    }
}