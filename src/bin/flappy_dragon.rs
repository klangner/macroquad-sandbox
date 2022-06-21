// Flappy Dragon implementation from th book
// [Hands on Rust](https://pragprog.com/titles/hwrust/hands-on-rust/)
//

use macroquad::prelude::*;
use macroquad_sandbox::state::GameState;
use macroquad::rand::gen_range;

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 800;
const GRAVITY_FORCE: f32 = 2.0;

enum GameMode {
    Menu,
    Playing,
    End,
}

struct Player {
    x: f32,
    y: f32,
    velocity: f32,
}

impl Player {
    fn new(x: f32, y: f32) -> Self {
        Player { 
            x,
            y, 
            velocity: 0.0 
        }
    }

    fn render(&self) {
        draw_circle(10.0, self.y, 5.0, RED);
    }

    fn gravity_and_move(&mut self, dt: f32) {
        if self.velocity < 10.0 * GRAVITY_FORCE { 
            self.velocity += dt * GRAVITY_FORCE;
        }
        self.y += self.velocity;
        self.x += 1.0;
        if self.y < 0.0 {
            self.y = 0.0;
        }
    }

    fn flap(&mut self, dt: f32) {
        self.velocity -= dt * 2.0 * GRAVITY_FORCE;
    }
}

struct Obstacle {
    x: f32,
    gap_y: f32,
    size: f32,
}

impl Obstacle {
    fn new(x: f32, score: i32 ) -> Self {
        let gap_y = gen_range(20.0, 400.0);
        Obstacle { x, gap_y, size: (20 - score) as f32 * 10.0 }
    }

    fn render(&self, player_x: f32) {
        let pos_x = self.x - player_x;
        let half_width = 5.0;
        if pos_x + half_width > 0.0 {
            draw_rectangle(pos_x-half_width, 0.0, 2.0*half_width, self.gap_y, DARKGRAY);
            draw_rectangle(pos_x-half_width, self.gap_y+self.size, 2.0*half_width, SCREEN_HEIGHT as f32, DARKGRAY);
        }
    }
        
    fn hit_obstacle(&self, player: &Player) -> bool {
        let half_width = 5.0;
        let x_match = self.x > - half_width && self.x < half_width;
        let above_gap = player.y < self.gap_y;
        let below_gap = player.y > self.gap_y + self.size;
        x_match && (above_gap || below_gap)
    }
}

struct State {
    game_mode: GameMode,
    player: Player,
    obstacle: Obstacle,
    score: i32,
}

impl State {
    fn new() -> Self {
        State { 
            game_mode: GameMode::Menu,
            player: Player::new(5.0, 25.0),
            obstacle: Obstacle::new(400.0, 10),
            score: 0
        }
    }
    
    fn main_menu(&mut self) {
        clear_background(LIGHTGRAY);

        draw_text("Welcome to Flappy Dragon!", 80.0, 60.0, 50.0, DARKGRAY);
        draw_text("(P) Play game", 300.0, 210.0, 30.0, DARKGRAY);
        draw_text("(Q) Quit Game", 300.0, 250.0, 30.0, DARKGRAY);

         if is_key_down(KeyCode::P) {
            self.restart();
        }
    }
    
    fn play(&mut self) {
        clear_background(BLUE);

        let dt = get_frame_time();
        self.player.gravity_and_move(dt);
        if is_key_down(KeyCode::Space) {
            self.player.flap(dt);
        }
        self.player.render();
        if self.player.y > SCREEN_HEIGHT as f32 {
            self.game_mode = GameMode::End;
        }

        self.obstacle.render(self.player.x);
    }
    
    fn end(&mut self) {
        clear_background(GREEN);
        draw_text("you are dead", 80.0, 60.0, 50.0, DARKGRAY);
        draw_text("(P) Play game", 300.0, 210.0, 30.0, DARKGRAY);
        draw_text("(Q) Quit Game", 300.0, 250.0, 30.0, DARKGRAY);

         if is_key_down(KeyCode::P) {
            self.restart();
        }
    }

    fn restart(&mut self) {
        self.player = Player::new(5.0, 25.0);
        self.game_mode = GameMode::Playing;
    }
}

impl GameState for State {
    fn tick(&mut self) {
        match self.game_mode {
            GameMode::Menu => self.main_menu(),
            GameMode::Playing => self.play(),
            GameMode::End => self.end(),
        }
    }
}


fn window_conf() -> Conf {
    Conf {
        window_title: "Flappy dragon".to_owned(),
        fullscreen: false,
        window_width: SCREEN_WIDTH,
        window_height: SCREEN_HEIGHT,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    let mut state = State::new();

    loop {
        // Process input
        #[cfg(not(target_arch = "wasm32"))]
        if is_key_down(KeyCode::Q) | is_key_down(KeyCode::Escape) {
            break;
        }

        state.tick();

        next_frame().await
    }
}