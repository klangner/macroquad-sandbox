// Draw map on the screen
//
// TODO
// * Lines
// * Cars 
// * Circle based lines
// * Stations

use macroquad::prelude::*;
use macroquad_sandbox::transnet::{Edge, Node, GraphPos, Graph};

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
    pos: GraphPos,
    speed: f32,
    schedule_id: usize,
}

struct World {
    map: Map,
    trans_net: Graph,
    schedules: Vec<Vec<usize>>,
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
    fn new(speed: f32, station_id: usize, route_id: usize) -> Self {
        Self { 
            pos: GraphPos::init(station_id),
            speed,
            schedule_id: route_id,
        }
    }

    fn update_pos(&mut self, new_pos: GraphPos) {
        self.pos = new_pos;
    }
}

impl World {
    fn new(map: Map, trans_net: Graph, schedules: Vec<Vec<usize>>, trains: Vec<Train>) -> Self {
        Self { 
            map, 
            trans_net,
            schedules,
            trains,
        }
    }
    
    fn update(&mut self, dt: f32) {
        for train in self.trains .iter_mut() {
            let route = &self.schedules[train.schedule_id];
            train.update_pos(self.trans_net.update_pos(&train.pos, route, train.speed * dt));
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
            let pos = world.trans_net.pos_to_location(&train.pos);
            self.draw_train(&pos, COLORS[idx]);
        }
    }
    
    fn draw_connections(&self, tracks: &Graph) {
        let thickness = 5.;
        for track in &tracks.edges {
            let p1 = &tracks.nodes[track.from_node_id];
            let p2 = &tracks.nodes[track.to_node_id];
            draw_line(p1.x, p1.y, p2.x, p2.y, thickness, BROWN);
        }
    }
    
    fn draw_train(&self, pos: &Node, color: Color) {
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
    let nodes = vec![
        Node::new(100., 100.),
        Node::new(700., 100.),
        Node::new(700., 500.),
        Node::new(100., 500.),
    ];

    let edges = vec![
        Edge::new(0, 1, &nodes),
        Edge::new(1, 3, &nodes),
        Edge::new(3, 0, &nodes),
        Edge::new(1, 2, &nodes),
    ];
    let graph = Graph::new(nodes, edges);

    let schedules = vec![
        vec![0, 1, 2],
        vec![3],
        vec![0, 1],
    ];

    let trains = vec![
            // Train::new(100., 0, 0),
            Train::new(200., 0, 0),
            Train::new(140., 3, 1),
            // Train::new(90., 3, 1),
            Train::new(150., 0, 2),
        ];

    World::new(Map::default(), graph, schedules, trains)
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