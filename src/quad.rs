// Helper function for macroquad
use macroquad::prelude::*;


/// Draw grid on full screen
pub fn plot_grid(num_cols: usize, num_rows: usize) {
    let margin: f32 = 5.;
    let max_width = screen_width() - 2. * margin;
    let max_height = screen_height() - 2. * margin;
    let cell_dx =  max_width / num_cols as f32;
    let cell_dy = max_height / num_rows as f32;
    let width = cell_dx * num_cols as f32;
    let height = cell_dy * num_rows as f32;

    clear_background(DARKGRAY);
    draw_rectangle(margin, margin, width, height, WHITE);

    for i in 0..(num_cols + 1) {
        let x = i as f32 * cell_dx + 5.;
        draw_line(x, margin, x, height + margin, 1.0, DARKGRAY)
    }
    for i in 0..(num_rows + 1) {
        let y = i as f32 * cell_dy + 5.;
        draw_line(margin, y, width + margin, y, 1.0, DARKGRAY)
    }
}