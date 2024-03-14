// Draw map on the screen

use macroquad::prelude::*;


const WINDOW_WIDTH: usize = 800;
const WINDOW_HEIGHT: usize = 600;


#[derive(Clone)]
struct Tile {
    color: Color,
}

#[derive(Clone)]
enum Track {
    EastWest, // ━
    NorthSouth, // │
    NorthWest, // ┐
    NorthEast, // ┌
    SouthWest, // ┘
    SouthEast, // └
}

struct Map {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

struct World {
    map: Map,
    tracks: Vec<Option<Track>>,
}


impl Tile {
    fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Map {
    fn new(width: usize, height: usize, tiles: Vec<Tile>) -> Self {
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


impl World {
    fn new(map: Map, tracks: Vec<Option<Track>>) -> Self {
        Self { map, tracks }
    }

    fn track_at(&self, x: usize, y: usize) -> Option<&Track> {
        let idx = self.xy_to_idx(x, y);
        self.tracks.get(idx).map(|v| v.as_ref()).unwrap_or(None)
    }

    fn xy_to_idx(&self, x: usize, y: usize) -> usize {
        y * self.map.width + x
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

                if let Some(track) = world.track_at(x, y) {
                    self.draw_track(rect_x, rect_y, cell_dx, cell_dy, track)
                }
            }
        }
    }

    fn draw_track(&self, x: f32, y: f32, dx: f32, dy: f32, track: &Track) {
        let center_x = x + dx /2.;
        let center_y = y + dy /2.;
        let end_x = x + dx;
        let end_y = y + dy;
        let thickness = 5.;
        match track {
            Track::EastWest => {
                draw_line(x, center_y, end_x, center_y, thickness, GRAY);
            }
            Track::NorthSouth => {
                draw_line(center_x, y, center_x, end_y, thickness, GRAY);
            }
            Track::NorthWest => {
                draw_line(center_x, end_y, center_x, center_y, thickness, GRAY);
                draw_line(center_x, center_y, x, center_y, thickness, GRAY);
            }
            Track::NorthEast => {
                draw_line(center_x, end_y, center_x, center_y, thickness, GRAY);
                draw_line(center_x, center_y, end_x, center_y, thickness, GRAY);
            }
            Track::SouthWest => {
                draw_line(center_x, y, center_x, center_y, thickness, GRAY);
                draw_line(center_x, center_y, x, center_y, thickness, GRAY);
            }
            Track::SouthEast => {
                draw_line(center_x, y, center_x, center_y, thickness, GRAY);
                draw_line(center_x, center_y, end_x, center_y, thickness, GRAY);
            }
        }
    }
}

fn init_world() -> World {
    let width = 8;
    let height = 6;
    let tiles = vec![Tile::new(LIME); width*height];
    let map = Map::new(width, height, tiles);
    let mut tracks = vec![None; width*height];
    tracks[1] = Some(Track::NorthEast);
    tracks[2] = Some(Track::EastWest);
    tracks[3] = Some(Track::EastWest);
    tracks[4] = Some(Track::EastWest);
    tracks[5] = Some(Track::EastWest);
    tracks[6] = Some(Track::NorthWest);

    tracks[9] = Some(Track::NorthSouth);
    tracks[14] = Some(Track::NorthSouth);

    tracks[17] = Some(Track::NorthSouth);
    tracks[22] = Some(Track::NorthSouth);

    tracks[25] = Some(Track::SouthEast);
    tracks[26] = Some(Track::EastWest);
    tracks[27] = Some(Track::EastWest);
    tracks[28] = Some(Track::EastWest);
    tracks[29] = Some(Track::EastWest);
    tracks[30] = Some(Track::SouthWest);

    World::new(map, tracks)
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
    let world = init_world();

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