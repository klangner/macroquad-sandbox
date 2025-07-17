// Deck builder cards ussing ECS
//

use bevy_ecs::prelude::*;
use macroquad::prelude::*;
use macroquad_sandbox::mqx::drawx::draw_rounded_rectangle;


const WINDOW_WIDTH: i32 = 1440;
const WINDOW_HEIGHT: i32 = 900;

const CARD_WIDTH: f32 = 200.;
const CARD_HEIGHT: f32 = 300.;


#[derive(Component)]
#[allow(dead_code)]
struct Card {
    name: String,
    cost: u8,
    attack: u8,
    defense: u8,
    text: String,
}

impl Card {
    pub fn new() -> Self {
        Self { name: "Card name".to_owned(), cost: 1, attack: 1, defense: 1, text: "Card text".to_owned() }
    }

}

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
    z_order: u32,
}

impl Position {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y, z_order: 1 } 
    }
}

impl From<&Position> for Vec2 {
    fn from(pos: &Position) -> Self {
        vec2(pos.x, pos.y)
    }
}

#[derive(Component)]
struct Draggable {
    has_mouse_over: bool,
    is_dragging: bool,
    mouse_last_pos: Vec2,
}

impl Draggable {
    fn new() -> Self {
        Self { has_mouse_over: false, is_dragging: false, mouse_last_pos: vec2(0., 0.) }
    }
}

fn draw_cards(query: Query<(Entity, &Position, &Draggable)>) {
    let sorted = query.iter()
        .sort_by_key::<&Position, _>(|p| p.z_order);
    for (_entity, position, draggable) in sorted {
        let pos: Vec2 = position.into();
        let frame_color = 
            if draggable.is_dragging { SKYBLUE } 
            else if draggable.has_mouse_over { RED } 
            else { ORANGE };
        draw_rounded_rectangle(pos, vec2(CARD_WIDTH, CARD_HEIGHT), 10., frame_color);
        draw_rounded_rectangle(pos + 10., vec2(CARD_WIDTH-20., CARD_HEIGHT-20.), 10., WHITE);
    }
}

fn drag_and_drop(mut query: Query<(Entity, &mut Position, &mut Draggable)>) {
    let  mouse_pos: Vec2 = mouse_position().into();
    // Reset mouse over flag
    for (_, _, mut draggable) in &mut query {
        draggable.has_mouse_over = false;
    }
    // New selection
    let sorted = query.iter_mut()
        .sort_by_key::<&Position, _>(|p| p.z_order)
        .rev();
    for (_, mut position, mut draggable) in sorted {
        let rect = Rect::new(position.x, position.y, CARD_WIDTH, CARD_HEIGHT);
        if rect.contains(mouse_pos) {
            draggable.has_mouse_over = true;
            if is_mouse_button_pressed(MouseButton::Left) {
                draggable.mouse_last_pos = mouse_pos;
                draggable.is_dragging = true;
                position.z_order = u32::max_value();
            } else if is_mouse_button_released(MouseButton::Left) {
                draggable.is_dragging = false;
            } else if draggable.is_dragging {
                let delta = draggable.mouse_last_pos - mouse_pos;
                position.x -= delta.x;
                position.y -= delta.y;
                draggable.mouse_last_pos = mouse_pos;
            }
            // Only 1 card can be selected
            break;
        }
    }
}

fn z_oder_sort(mut query: Query<(Entity, &mut Position)>) {
    let sorted = query.iter_mut()
        .sort_by_key::<&Position, _>(|p| p.z_order);
    for (idx, (_, mut position)) in sorted.enumerate() {
        position.z_order = idx as u32 + 1;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Cards".to_owned(),
        fullscreen: false,
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut world = World::new();
    world.spawn((Card::new(), Position::new(10., 10.), Draggable::new()));
    world.spawn((Card::new(), Position::new(260., 10.), Draggable::new()));
    world.spawn((Card::new(), Position::new(510., 10.), Draggable::new()));

    // Create a new Schedule, which defines an execution strategy for Systems
    let mut schedule = Schedule::default();

    // Add our system to the schedule
    schedule
        .add_systems(drag_and_drop)
        .add_systems(z_oder_sort)
        .add_systems(draw_cards);


    loop {
        // let dt = get_frame_time();
        // Process input
        #[cfg(not(target_arch = "wasm32"))]
        if is_key_down(KeyCode::Q) | is_key_down(KeyCode::Escape) {
            break;
        }

        clear_background(LIGHTGRAY);
        // Run the schedule once. If your app has a "loop", you would run this once per loop
        schedule.run(&mut world);

        next_frame().await
    }
}