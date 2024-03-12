// Draw map on the screen

use macroquad::prelude::*;

const WINDOW_WIDTH: usize = 800;
const WINDOW_HEIGHT: usize = 600;


#[derive(Clone)]
struct Tile {
    color: Color,
}

#[derive(Clone, Copy)]
struct Point {
    x: f32,
    y: f32,
}


struct Map {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

struct Track {
    from_node: usize,
    to_node: usize,
    length: f32,
}

struct TrackNetwork {
    nodes: Vec<Point>,
    tracks: Vec<Track>,
}

#[derive(Clone, Copy)]
struct TrackPos {
    edge: usize,
    distance: f32,
}

struct World {
    map: Map,
    track_network: TrackNetwork,
    train_pos: TrackPos,
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

impl Track {
    fn new(from_node: usize, to_node: usize, nodes: &Vec<Point>) -> Self {
        let pos_a = nodes[from_node];
        let pos_b = nodes[to_node];
        let dx = pos_b.x - pos_a.x;
        let dy = pos_b.y - pos_a.y;
        let length = (dx.powi(2) + dy.powi(2)).sqrt();
        Self {
            from_node,
            to_node,
            length,
        }
    }
}

impl TrackPos {
    fn new(edge: usize, distance: f32) -> Self {
        Self { edge, distance }
    }
}

impl Default for TrackNetwork {
    fn default() -> Self {
        let nodes = vec![
            Point::new(100., 100.),
            Point::new(700., 100.),
            Point::new(700., 500.),
            Point::new(100., 500.),
        ];

        let tracks = vec![
            Track::new(0, 1, &nodes),
            Track::new(1, 2, &nodes),
            Track::new(2, 3, &nodes),
            Track::new(3, 0, &nodes),
        ];

        Self {
            nodes,
            tracks,
        }
    }
}

impl TrackNetwork {
    fn track_to_map(&self, track_pos: &TrackPos) -> Point {
        let track = &self.tracks[track_pos.edge];
        let pos_a = self.nodes[track.from_node];
        let pos_b = self.nodes[track.to_node];
        let dx = pos_b.x - pos_a.x;
        let dy = pos_b.y - pos_a.y;
        let frac = track_pos.distance / track.length;
        let pos_a = self.nodes[track.from_node];
        Point::new(pos_a.x + frac * dx, pos_a.y + frac * dy)
    }

    fn update_pos(&self, pos: &TrackPos, distance: f32) -> TrackPos {
        let track = &self.tracks[pos.edge];
        let new_distance = pos.distance + distance;

        if new_distance > track.length {
            let new_edge = self.tracks.iter().enumerate()
                .find(|(i, _)| *i == track.to_node)
                .map(|(i, _)| i)
                .unwrap();
            TrackPos::new(new_edge, pos.distance + distance - track.length)
        } else {
            TrackPos::new(pos.edge, pos.distance + distance)
        }
    }
}

impl Default for World {
    fn default() -> Self {
        let tracks = TrackNetwork::default();
        let train_pos = TrackPos::new(0, 0.);

        Self { 
            map: Default::default(), 
            track_network: tracks,
            train_pos,
        }
    }
}

impl World {
    fn train_map_pos(&self) -> Point {
        self.track_network.track_to_map(&self.train_pos)
    }

    fn update(&mut self, dt: f32) {
        self.train_pos = self.track_network.update_pos(&self.train_pos, 100. * dt);
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
        self.draw_tracks(&world.track_network);

        let pos = world.train_map_pos();
        self.draw_train(&pos);
    }
    
    fn draw_tracks(&self, tracks: &TrackNetwork) {
        let thickness = 10.;
        for track in &tracks.tracks {
            let p1 = &tracks.nodes[track.from_node];
            let p2 = &tracks.nodes[track.to_node];
            draw_line(p1.x, p1.y, p2.x, p2.y, thickness, BROWN);
        }
    }
    
    fn draw_train(&self, pos: &Point) {
        draw_circle(pos.x, pos.y, 10., YELLOW);
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
    let mut map_view = WorldView::new();
    let mut world = World::default();

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