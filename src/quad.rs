// Helper function for macroquad
use macroquad::prelude::*;



pub fn draw_arrow(x: f32, y: f32, dx: f32, dy: f32, color: Color) {
    let ax = 0.1 * dx;
    let ay = 0.1 * dy;
    // draw_circle(x, y, 2., color);
    draw_line(x, y, x + dx, y + dy, 1.0, color);
    draw_line( x + dx, y + dy, x + 0.9*dx+ay, y + 0.9*dy-ax,  1.0, color);
    draw_line( x + dx, y + dy, x + 0.9*dx-ay, y + 0.9*dy+ax,  1.0, color);
}

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