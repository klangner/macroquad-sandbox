use macroquad::prelude::*;
use std::fs;
use ::rand::Rng;
use serde_derive::Deserialize;
use anyhow::Result;


const WINDOW_WIDTH: usize = 1024;
const WINDOW_HEIGHT: usize = 800;

 #[derive(Deserialize, Debug)]
pub struct SimParams {
    width: usize,
    height: usize,
    population: usize,
    steps_per_generation: usize,
}

 #[derive(Clone)]
pub struct Ant {
    pub genes: u32,
    pub move_left: bool,
    pub move_right: bool,
    pub move_top: bool,
    pub move_bottom: bool,
}

 #[derive(Clone)]
pub enum Cell {
    Empty,
    Occupied(Ant)
}

pub struct Universe {
    params: SimParams,
    cells: Vec<Cell>,
    step: usize,
    generation: usize,
}

impl Ant {
    fn new(genes: u32) -> Self {
        let move_left = genes & 0b0001 > 0;
        let move_right = genes & 0b0010 > 0;
        let move_top = genes & 0b0100 > 0;
        let move_bottom = genes & 0b1000 > 0;
        Self { 
            genes,
            move_left,
            move_right,
            move_top,
            move_bottom,
        }
    }
}

impl Universe {
    pub fn random(params: SimParams) -> Universe {
        let mut rng = ::rand::thread_rng();

        let mut cells = vec![Cell::Empty; params.width*params.height];
        let distance = params.width*params.height/ params.population - 1;
        for i in 0..params.population {
            let a = i*distance;
            let b = (i+1)*distance;
            let idx = rng.gen_range(a..b);
            cells[idx] = Cell::Occupied(Ant::new(rng.gen()))
        }
        Universe {
            params,
            cells,
            step: 0,
            generation: 0,
        }
    }
    
    pub fn cell_at(&self, row: usize, col: usize) -> &Cell {
        let idx = self.xy_idx(row, col);
        &self.cells[idx]
    }

    pub fn tick(&mut self) {
        if self.step < self.params.steps_per_generation {
            self.update_ants();
            self.step += 1;
        } else {
            self.new_generation();
            self.step = 0;
            self.generation += 1;
        }
    }

    fn update_ants(&mut self) {
        let mut next_cells = vec![Cell::Empty; self.params.width*self.params.height];
        
        for row in 0..self.params.height {
            for col in 0..self.params.width {
                let idx = self.xy_idx(row, col);
                if let Cell::Occupied(a) = &self.cells[idx] {
                    let mut r = row;
                    let mut c = col;
                    if r > 0 && a.move_top { r -= 1}
                    if r < self.params.height-1 && a.move_bottom { r += 1}
                    if c > 0 && a.move_left { c -= 1}
                    if c < self.params.width-1 && a.move_right { c += 1}

                    let new_idx = self.xy_idx(r, c);
                    next_cells[new_idx] = Cell::Occupied(a.clone());
                }
            }
        }
 
        self.cells = next_cells;
    }

    fn new_generation(&mut self) {
        let mut rng = ::rand::thread_rng();

        let mut cells = vec![Cell::Empty; self.params.width*self.params.height];
        let distance = self.params.width*self.params.height/ self.params.population - 1;
        for i in 0..self.params.population {
            let a = i*distance;
            let b = (i+1)*distance;
            let idx = rng.gen_range(a..b);
            cells[idx] = Cell::Occupied(Ant::new(rng.gen()))
        } 
 
        self.cells = cells;
    }

    fn xy_idx(&self, row: usize, column: usize) -> usize {
        (row * self.params.width + column) as usize
    }
}


fn draw_universe(universe: &Universe) {
    let cell_dx =  screen_width() / universe.params.width as f32;
    let cell_dy = screen_height() / universe.params.height as f32;

    clear_background(WHITE);

    for r in 0..universe.params.height {
        for c in 0..universe.params.width {
            if let Cell::Occupied(a) = universe.cell_at(r, c) {
                let x = c as f32 * cell_dx;
                let y = r as f32 * cell_dy;
                let r =  a.genes as u8;
                let g =  (a.genes >> 8) as u8;
                let b =  (a.genes >> 16) as u8;
                draw_rectangle( x, y, cell_dx, cell_dy, color_u8!(r, g, b, 255));
            }
        }
    }
}


fn window_conf() -> Conf {
    Conf {
        window_title: "Game of life".to_owned(),
        fullscreen: false,
        window_width: WINDOW_WIDTH as i32,
        window_height: WINDOW_HEIGHT as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() -> Result<()> {
    let param_file = std::env::args().last().expect("No param file provided");
    let contents = fs::read_to_string(&param_file)?;
    let params: SimParams = toml::from_str(&contents)?;
    let mut universe = Universe::random(params);

    loop {
        universe.tick();

        // Process input
        #[cfg(not(target_arch = "wasm32"))]
        if is_key_down(KeyCode::Q) | is_key_down(KeyCode::Escape) {
            break;
        }

        draw_universe(&universe);
        
        next_frame().await
    }

    Ok(())
}