use ratatui::{
    style::Color,
    widgets::canvas::{Painter, Shape},
};

pub struct Tomato {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
}

impl Shape for Tomato {
    fn draw(&self, painter: &mut Painter) {
        let mut x = 0;
        let mut y = self.radius as i32;
        let mut d = 3 - 2 * y;

        let draw_circle_points = |painter: &mut Painter, cx: f64, cy: f64, x: i32, y: i32| {
            let points = [
                (cx + x as f64, cy + y as f64),
                (cx - x as f64, cy + y as f64),
                (cx + x as f64, cy - y as f64),
                (cx - x as f64, cy - y as f64),
                (cx + y as f64, cy + x as f64),
                (cx - y as f64, cy + x as f64),
                (cx + y as f64, cy - x as f64),
                (cx - y as f64, cy - x as f64),
            ];

            for &(px, py) in &points {
                painter.paint(px as usize, py as usize, Color::Red);
            }
        };

        while x <= y {
            draw_circle_points(painter, self.x, self.y, x, y);
            if d < 0 {
                d = d + 4 * x + 6;
            } else {
                d = d + 4 * (x - y) + 10;
                y -= 1;
            }
            x += 1;
        }
    }
}