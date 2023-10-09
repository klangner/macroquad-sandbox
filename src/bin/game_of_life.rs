use macroquad::prelude::*;
use std::{fmt, fs};
use ::rand::Rng;
use serde_derive::Deserialize;
use anyhow::Result;


const WINDOW_WIDTH: usize = 1024;
const WINDOW_HEIGHT: usize = 800;

 #[derive(Deserialize, Debug)]
struct SimParams {
    width: usize,
    height: usize,
    prob: f32,
}


#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

pub struct Universe {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Universe {

    pub fn new(width: usize, height: usize) -> Universe {
        let cells = vec![Cell::Dead; width*height];
        Universe {width, height, cells}
    }
    
    pub fn random(width: usize, height: usize, prob: f32) -> Universe {
        let mut rng = ::rand::thread_rng();
        let cells = (0..(width*height)).map(|_| 
            if rng.gen_range(0.0..1.0)  > prob {Cell::Alive}
            else {Cell::Dead}
        ).collect();
        Universe {width, height, cells}
    }
    
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn cell_at(&self, row: usize, col: usize) -> Cell {
        let idx = self.xy_idx(row, col);
        self.cells[idx]
    }

    pub fn tick(&mut self) {
        let mut next_cells = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.xy_idx(row, col);
                let old_cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let cell = match (old_cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (Cell::Dead, 3) => Cell::Alive,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };

                next_cells[idx] = cell;
            }
        }

        self.cells = next_cells;
    }

    fn xy_idx(&self, row: usize, column: usize) -> usize {
        (row * self.width + column) as usize
    }

    // Count number of neighbors
    fn live_neighbor_count(&self, row: usize, column: usize) -> usize {
        let north = if row == 0 { self.height - 1 } else { row - 1 };
        let south = if row == self.height - 1 { 0 } else { row + 1 };
        let west = if column == 0 { self.width - 1 } else { column - 1 };
        let east = if column == self.width - 1 { 0 } else { column + 1 };

        let neighbors = vec![
            self.cell_at(north, west),
            self.cell_at(north, column),
            self.cell_at(north, east),
            self.cell_at(row, west),
            self.cell_at(row, east),
            self.cell_at(south, west),
            self.cell_at(south, column),
            self.cell_at(south, east)];
        neighbors.iter().map(|&c| c as usize).sum()
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}


fn draw_universe(universe: &Universe) {
    let cell_dx =  screen_width() / universe.width() as f32;
    let cell_dy = screen_height() / universe.height() as f32;

    clear_background(WHITE);

    // Densities
    for r in 0..universe.height() {
        for c in 0..universe.width() {
            if universe.cell_at(r, c) == Cell::Alive {
                let x = c as f32 * cell_dx;
                let y = r as f32 * cell_dy;
                draw_rectangle( x, y, cell_dx, cell_dy, BLACK);
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
    let mut universe = Universe::random(params.width, params.height, params.prob);

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