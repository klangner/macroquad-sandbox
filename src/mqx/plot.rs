// Plot chart
//
use macroquad::prelude::*;

use crate::mqx::drawx::draw_rounded_rectangle;


pub struct Plot {
    title: String,
    pos: Vec2,
    size: Vec2,
}

impl Plot {
    pub fn new(title: &str, pos: Vec2, size: Vec2) -> Self {
        Self { title: title.to_owned(), pos, size }
    }

    pub fn plot(&self, xs: &Vec<f32>, color: Color) {
        draw_rounded_rectangle(self.pos, self.size, 10., WHITE);
        let chart_area = Rect::new(
            self.pos.x + 20., 
            self.pos.y + 30., self.size.x - 40., self.size.y - 40.);

        // Title
        let text_dims = measure_text(&self.title, Option::None, 20, 1.0);
        draw_text(
            &self.title, 
            self.pos.x + (self.size.x - text_dims.width) / 2., 
            self.pos.y +  text_dims.height +  5., 
            20.0, BLACK);
        // x line
        draw_line(
            chart_area.x, 
            chart_area.y + chart_area.h, 
            chart_area.x + chart_area.w,
            chart_area.y + chart_area.h, 
            1., 
            BLACK);
        // y line
        draw_line(
            chart_area.x, 
            chart_area.y, 
            chart_area.x,
            chart_area.y + chart_area.h, 
            1., 
            BLACK);

        // Plot points
        if xs.len() > 1 {
            let dx = chart_area.w / xs.len() as f32;
            let dy_min = xs.iter()
                .map(|&x| x.into())
                .reduce(f32::min)
                .unwrap();
            let dy_max = xs.iter()
                .map(|&x| x.into())
                .reduce(f32::max)
                .unwrap();
            let scale_y = chart_area.h / (dy_max - dy_min);

            let ps: Vec<Vec2> = xs.iter().enumerate()
                .map(|(i, x)| 
                    Vec2::new(
                        chart_area.x + i as f32 * dx, 
                        chart_area.y + chart_area.h - x * scale_y))
                .collect();

            for i in 1..ps.len() {
                let p1 = ps[i-1];
                let p2 = ps[i];
                draw_line(p1.x, p1.y, p2.x,p2.y, 1., color);
            }
        }
    }
}