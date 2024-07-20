use std::collections::HashSet;

use macroquad::input::{is_key_down, is_key_pressed, KeyCode};


#[derive(Eq, Hash, PartialEq)]
pub enum Action {
    Quit,
    Left,
    Right,
    Up,
    Down,
    ZoomIn,
    ZoomOut,
}

pub struct Input {
    actions: HashSet<Action>,
}

impl Input {
    pub fn new() -> Self {
        Input { actions: HashSet::default() }
    }    

    // Scan inputs and saved them as actions
    pub fn process(&mut self) {

        self.actions.clear();

        if is_key_pressed(KeyCode::Q) {
            self.actions.insert(Action::Quit);
        }

        if is_key_down(KeyCode::RightBracket) {
            self.actions.insert(Action::ZoomIn);
        }
        if is_key_down(KeyCode::LeftBracket) {
            self.actions.insert(Action::ZoomOut);
        }
        if is_key_down(KeyCode::Left) || is_key_pressed(KeyCode::A) {
            self.actions.insert(Action::Left);
        }
        if is_key_down(KeyCode::Right) || is_key_pressed(KeyCode::D) {
            self.actions.insert(Action::Right);
        }
        if is_key_down(KeyCode::Up) || is_key_pressed(KeyCode::W) {
            self.actions.insert(Action::Up);
        }
        if is_key_down(KeyCode::Down) || is_key_pressed(KeyCode::S) {
            self.actions.insert(Action::Down);
        }
    }

    // Check if action was executed
    pub fn is_action_pressed(&self, action: Action) -> bool {
        self.actions.contains(&action)
    }
}