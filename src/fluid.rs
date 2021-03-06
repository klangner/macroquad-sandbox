// Fluid sim implementation

#[derive(Copy, Clone)]
pub struct Vec2d {
    pub x: f32,
    pub y: f32,
}

pub struct UniverseBuilder {
    width: i32,
    height: i32,
    velocities: Vec<Vec2d>,
    densities: Vec<f32>,
    diffusion_rate: f32,
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

    pub fn zero() -> Vec2d {
        Vec2d { x: 0.0, y: 0.0 }
    }

    pub fn scale(self, s: f32) -> Self {
        Vec2d::new(self.x*s, self.y*s)        
    }
}

impl UniverseBuilder {
    pub fn new(width: i32, height: i32) -> UniverseBuilder {
        UniverseBuilder { 
            width,
            height,
            velocities: vec![Vec2d::zero(); (width*height) as usize], 
            densities: vec![0.0; (width*height) as usize],
            diffusion_rate: 0.0,
        }
    }

    /// Set same velocity for all cells
    pub fn with_velocity(mut self, velocity: Vec2d) -> UniverseBuilder {
        self.velocities = vec![velocity; (self.width*self.height) as usize];
        self
    }

    /// Set same density for all cells
    pub fn with_density(mut self, density: f32) -> UniverseBuilder {
        self.densities = vec![density; (self.width*self.height) as usize];
        self
    }

    /// Set diffusion rate
    pub fn with_diffusion_rate(mut self, rate: f32) -> UniverseBuilder {
        self.diffusion_rate = rate;
        self
    }

    pub fn build(&self) -> Universe {
        Universe {
            width: self.width,
            height: self.height,
            velocities: self.velocities.clone(),
            densities: self.densities.clone(),
            diffusion_rate: self.diffusion_rate,
        }
    }
}

impl Universe {
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

    // Move fluid along velocity field
    #[allow(unused_variables)]
    pub fn advect(&mut self, dt: f32) {
        let d0 = self.densities.clone();
        let ds = dt * self.width as f32;

        for i in 0..self.width as i32 {
            for j in 0..self.height as i32 {
                let idx = self.xy_idx(i, j);
                let x = i as f32 - ds * self.velocities[idx].x; 
                let y = j as f32 - ds * self.velocities[idx].y;
                // if (x<0.5) x=0.5; 
                // if (x>N+0.5) x=N+0.5; 
                // i0=(int)x; 
                // i1=i0+1;
                // if (y<0.5) y=0.5; 
                // if (y>N+0.5) y=N+ 0.5; 
                // j0=(int)y; 
                // j1=j0+1;
                // s1 = x-i0; 
                // s0 = 1-s1; 
                // t1 = y-j0; 
                // t0 = 1-t1;
                // d[IX(i,j)] = s0*(t0*d0[IX(i0,j0)] + t1*d0[IX(i0,j1)]) + s1*(t0*d0[IX(i1,j0)] + t1*d0[IX(i1,j1)]);
            }
        }
    }

    fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y * self.width + x) as usize
    }
}