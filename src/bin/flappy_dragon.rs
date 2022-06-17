// Flappy Dragon implementation from th book
// [Hands on Rust](https://pragprog.com/titles/hwrust/hands-on-rust/)
//

use macroquad::prelude::*;
use macroquad_sandbox::state::GameState;

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

struct State {
    game_mode: GameMode,
    player: Player,
}

impl State {
    fn new() -> Self {
        State { 
            game_mode: GameMode::Menu,
            player: Player::new(5.0, 25.0),
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