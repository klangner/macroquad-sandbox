// Draw map on the screen
//
// TODO
// * Lines
// * Cars 
// * Circle based lines
// * Stations

use macroquad::prelude::*;
use macroquad_sandbox::transnet::{Connection, Location, RoutePos, TransNet};

const WINDOW_WIDTH: usize = 800;
const WINDOW_HEIGHT: usize = 600;

const COLORS: [Color; 10] = [PINK, BLUE, BEIGE, YELLOW, DARKBROWN, ORANGE, PINK, RED, MAROON, DARKPURPLE]; 


#[derive(Clone)]
struct Tile {
    color: Color,
}

struct Map {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

struct Train {
    pos: RoutePos,
    speed: f32,
}

struct World {
    map: Map,
    trans_net: TransNet,
    trains: Vec<Train>,
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

impl Train {
    fn new(speed: f32) -> Self {
        Self { 
            pos: RoutePos::init(0),
            speed,
        }
    }

    fn update_pos(&mut self, new_pos: RoutePos) {
        self.pos = new_pos;
    }
}

impl World {
    fn new(map: Map, trans_net: TransNet, trains: Vec<Train>) -> Self {
        Self { 
            map, 
            trans_net,
            trains,
        }
    }
    
    fn update(&mut self, dt: f32) {
        for train in self.trains .iter_mut() {
            train.update_pos(self.trans_net.update_pos(&train.pos, train.speed * dt));
        }
    }
}


#[derive(Debug)]
struct WorldView {
    scale: f32,
    pos_x: f32,
    pos_y: f32,
}

impl WorldView {
    pub fn new() -> Self {
        Self {scale: 1., pos_x: 0.0, pos_y: 0.0}
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
        self.draw_connections(&world.trans_net);

        for (idx, train) in world.trains.iter().enumerate() {
            let pos = world.trans_net.track_to_map(&train.pos);
            self.draw_train(&pos, COLORS[idx]);
        }
    }
    
    fn draw_connections(&self, tracks: &TransNet) {
        let thickness = 5.;
        for track in &tracks.connections {
            let p1 = &tracks.locations[track.from_location];
            let p2 = &tracks.locations[track.to_location];
            draw_line(p1.x, p1.y, p2.x, p2.y, thickness, BROWN);
        }
    }
    
    fn draw_train(&self, pos: &Location, color: Color) {
        draw_circle(pos.x, pos.y, 10., color);
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

fn init_world() -> World {
    let locations = vec![
        Location::new(100., 100.),
        Location::new(700., 100.),
        Location::new(700., 500.),
        Location::new(100., 500.),
    ];

    let connections = vec![
        Connection::new(0, 1, &locations),
        Connection::new(1, 2, &locations),
        Connection::new(2, 3, &locations),
        Connection::new(3, 0, &locations),
    ];
    let trans_net = TransNet::new(locations, connections);

    let trains = vec![
            Train::new(50.),
            Train::new(100.),
            Train::new(200.),
            Train::new(10.),
            Train::new(90.),
            Train::new(150.),
        ];

    World::new(Map::default(), trans_net, trains)
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut map_view = WorldView::new();
    let mut world = init_world();

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
        world.update(dt);
        // Draw world
        map_view.draw(&world);
        
        next_frame().await
    }
}