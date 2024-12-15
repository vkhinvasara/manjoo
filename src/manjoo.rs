use ratatui::{
 style::Color, widgets::canvas::{Painter, Shape}
};
use crate::constants::{PixelData, get_pixel_data};
pub struct Manjoo {
    pub scale: u8,
    pub is_static: bool,
    pub x_position: usize,
    pub running_flag: bool,
	pub has_tomato:bool
}

impl Shape for Manjoo {
    fn draw(&self, painter: &mut Painter) {
        
        let pixels = if self.is_static {
            get_pixel_data(PixelData::Static)
        } else {
            if self.running_flag && !self.has_tomato{
                get_pixel_data(PixelData::Running1)
            } else if !self.running_flag && !self.has_tomato{
                get_pixel_data(PixelData::Running2)
            }
			else if self.running_flag && self.has_tomato{
				get_pixel_data(PixelData::RunningWTomato1)
			}
			else{
				get_pixel_data(PixelData::RunningWTomato2)
			}
        };
        let scale = self.scale as usize;
        let height = 32;
        let width = 32;
        for y in 0..height {
            for x in 0..width {
                let index = (y * width + x) as usize;
                let color = pixels[index];

                let color = match color {
                    0xfff39442 => Color::Indexed(172),
                    0xffffffff => Color::White,
                    0xff000000 => Color::Black,
                    0xffff9c9c => Color::Indexed(210),
                    0xfff8b1b1 => Color::Indexed(210),
                    0xff3434fe => Color::Red,
					0xfff34242 => Color::Red,
					0xff3e8f00 => Color::Green,
                    _ => continue,
                };
                for dy in 0..scale {
                    for dx in 0..scale {
						
                        painter.paint(
                            (self.x_position + x * scale + dx) as usize,
                            (y * scale + dy) as usize,
                            color,
                        );
                    }
                }
            }
        }
    }
}
