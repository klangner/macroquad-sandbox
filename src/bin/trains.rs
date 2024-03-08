// Draw map on the screen

use macroquad::prelude::*;

const WINDOW_WIDTH: usize = 800;
const WINDOW_HEIGHT: usize = 600;


#[derive(Clone)]
struct Tile {
    color: Color,
}

struct Point {
    x: f32,
    y: f32,
}


struct Map {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

struct World {
    map: Map,
    track: Vec<Point>,
    train: Point,
}


impl Point {
    fn new(x: f32, y: f32) -> Self {
        Self{ x, y }
    }
}


impl Tile {
    fn new(color: Color) -> Self {
        Self { color }
    }
}


impl Default for Map {
    fn default() -> Self {
        let width = 80;
        let height = 60;
        let tiles = (0..width*height).map(|_|Tile::new(DARKGREEN)).collect();

        Self {
            width,
            height,
            tiles,
        }
    }
}

impl Map {
    fn tile_at(&self, x: usize, y: usize) -> Option<&Tile> {
        let idx = y * self.width + x;
        self.tiles.get(idx)
    }
}

impl Default for World {
    fn default() -> Self {
        let track = vec![
            Point::new(100., 100.),
            Point::new(700., 100.),
            Point::new(700., 500.),
            Point::new(100., 500.),
        ];
        let train = Point::new(100., 100.);

        Self { 
            map: Default::default(), 
            track, 
            train,
        }
    }
}


#[derive(Debug)]
struct MapView {
    scale: f32,
    pos_x: f32,
    pos_y: f32,
}

impl MapView {
    pub fn new() -> MapView {
        MapView {scale: 1., pos_x: 0.0, pos_y: 0.0}
    }

    pub fn zoom_in(&mut self, dt: f32) {
        self.scale += dt;
        if self.scale > 10.0 {
            self.scale = 10.0
        }
    }

    pub fn zoom_out(&mut self, dt: f32) {
        self.scale -= dt;
        if self.scale < 0.1 {
            self.scale = 0.1
        }
    }

    pub fn move_right(&mut self, dt: f32) {
        self.pos_x -= 500.0 * dt;
    }

    pub fn move_left(&mut self, dt: f32) {
        self.pos_x += 500.0 * dt;
        if self.pos_x > 0.0 {
            self.pos_x = 0.0
        }
    }

    pub fn move_down(&mut self, dt: f32) {
        self.pos_y -= 500.0 * dt;
    }

    pub fn move_up(&mut self, dt: f32) {
        self.pos_y += 500.0 * dt;
        if self.pos_y > 0.0 {
            self.pos_y = 0.0
        }
    }

    fn draw(&self, world: &World) {
        let map = &world.map;
        let cell_dx = self.scale * (WINDOW_WIDTH / map.width) as f32;
        let cell_dy = self.scale * (WINDOW_HEIGHT / map.height) as f32;

        clear_background(LIGHTGRAY);
        for x in 0..map.width {
            for y in 0..map.height {
                let color = map.tile_at(x, y).unwrap().color;
                let rect_x = x as f32 * cell_dx + self.pos_x;
                let rect_y = y as f32 * cell_dy + self.pos_y;
                draw_rectangle(
                    rect_x, 
                    rect_y, 
                    cell_dx, 
                    cell_dy, color);
            }
        }
        self.draw_track(&world.track);
        self.draw_train(&world.train);
    }
    
    fn draw_track(&self, track: &Vec<Point>) {
        let thickness = 10.;
        for i in 0..track.len() {
            let j = if i+1 < track.len() {i+1} else {0};
            let p1 = &track[i];
            let p2 = &track[j];
            draw_line(p1.x, p1.y, p2.x, p2.y, thickness, BROWN);
        }
    }
    
    fn draw_train(&self, train: &Point) {
        draw_circle(train.x, train.y, 10., YELLOW);
    }

}


fn window_conf() -> Conf {
    Conf {
        window_title: "Map viewer".to_owned(),
        fullscreen: false,
        window_width: WINDOW_WIDTH as i32,
        window_height: WINDOW_HEIGHT as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut map_view = MapView::new();
    let world = World::default();

    loop {
        let dt = get_frame_time();

        // Process input aka Controller
        #[cfg(not(target_arch = "wasm32"))]
        if is_key_down(KeyCode::Q) | is_key_down(KeyCode::Escape) {
            break;
        }
        if is_key_down(KeyCode::RightBracket) {
            map_view.zoom_in(dt);
        }
        if is_key_down(KeyCode::LeftBracket) {
            map_view.zoom_out(dt);
        }
        if is_key_down(KeyCode::Left) {
            map_view.move_left(dt);
        }
        if is_key_down(KeyCode::Right) {
            map_view.move_right(dt);
        }
        if is_key_down(KeyCode::Up) {
            map_view.move_up(dt);
        }
        if is_key_down(KeyCode::Down) {
            map_view.move_down(dt);
        }
        // Update world (nothing there yet)
        // Draw world
        map_view.draw(&world);
        
        next_frame().await
    }
}