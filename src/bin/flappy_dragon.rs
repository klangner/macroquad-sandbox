// Flappy Dragon implementation from th book
// [Hands on Rust](https://pragprog.com/titles/hwrust/hands-on-rust/)
//
// What is missing:
// ================
// * Hitting object
// * Scoring
// * Multiple obstacles
// * Background objects
// * Bird animation
//


use macroquad::prelude::*;
use macroquad_sandbox::state::GameState;
use macroquad::rand::gen_range;

const SCREEN_WIDTH: i32 = 1024;
const SCREEN_HEIGHT: i32 = 700;
const SINK_SPEED: f32 = 100.0;
const FLAP_SPEED: f32 = 150.0;
const PLAYER_WIDTH: f32 = 60.0;
const PLAYER_HEIGHT: f32 = 40.0;

enum GameMode {
    Menu,
    Playing,
    End,
}

struct Player {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

struct Obstacle {
    x: f32,
    gap_y: f32,
    size: f32,
    score: i32,
}

struct State {
    game_mode: GameMode,
    player: Player,
    obstacles: Vec<Obstacle>,
    score: i32,
}


impl Player {
    fn new(x: f32, y: f32) -> Self {
        Player { 
            x,
            y: y + PLAYER_HEIGHT / 2.0, 
            width: PLAYER_WIDTH,
            height: PLAYER_HEIGHT,
        }
    }

    fn render(&self) {
        draw_rectangle_lines(0.0, self.y-self.height/2.0, self.width, self.height, 4.0, RED);
    }

    fn gravity_and_move(&mut self, dt: f32, is_flapping: bool) {
        if is_flapping {
            self.y -= dt * FLAP_SPEED;
        } else {
            self.y += dt * SINK_SPEED;
        }
        self.x += 1.0;
        if self.y < 0.0 {
            self.y = 0.0;
        }
    }
}


impl Obstacle {
    fn new(x: f32, score: i32 ) -> Self {
        let gap_y = gen_range(20.0, 400.0);
        Obstacle { x, gap_y, size: (20 - score) as f32 * 10.0, score }
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
        let half_width = player.width;
        let x_match = self.x > player.x - half_width && self.x < player.x + half_width;
        let above_gap = player.y < self.gap_y;
        let below_gap = player.y > self.gap_y + self.size;
        x_match && (above_gap || below_gap)
    }
}

impl State {
    fn new() -> Self {
        let obstacles = vec![Obstacle::new(400.0, 10), Obstacle::new(800.0, 11), Obstacle::new(1200.0, 12)];
        State { 
            game_mode: GameMode::Menu,
            player: Player::new(0.0, 400.0),
            obstacles: obstacles,
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
        draw_text(&format!("Score: {}", self.score), 350.0, 30.0, 30.0, WHITE);

        let dt = get_frame_time();
        let is_flapping = is_key_down(KeyCode::Space);
        self.player.gravity_and_move(dt, is_flapping);
        // check if bird collides
        for obstacle in self.obstacles.iter() {
            if obstacle.hit_obstacle(&self.player) || self.player.y > SCREEN_HEIGHT as f32 {
                self.game_mode = GameMode::End;
            }

        }
        // Replace obstacle if needed
        if self.obstacles[0].x < self.player.x {
            self.score += self.obstacles[0].score;
            self.obstacles.remove(0);
            self.obstacles.push(Obstacle::new(self.player.x + 1200., 10));
        }

        self.player.render();
        for obstacle in self.obstacles.iter() {
            obstacle.render(self.player.x);
        }
    }
    
    fn end(&mut self) {
        clear_background(GREEN);
        draw_text(&format!("Game Over. You score: {}", self.score), 250.0, 60.0, 50.0, DARKGRAY);
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