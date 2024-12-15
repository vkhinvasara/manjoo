mod manjoo;
mod tomato;
mod constants;
use manjoo::Manjoo;
use ratatui::{
    backend::TermwizBackend, layout::Alignment, symbols::Marker, widgets::{canvas::Canvas, Block}, Terminal
};
use std::{
    error::Error,
    thread,
    time::{Duration, Instant},
};
use tomato::Tomato;

fn main() -> Result<(), Box<dyn Error>> {
    let backend = TermwizBackend::new()?;
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    terminal.clear()?;
    
    let now = Instant::now();
    let mut sleep_duration = 10;
    let mut render_static = true;
    let mut running_manjoo_x: usize = 0;
    let mut running_flag = false;
    let mut has_tomato = false;
    let tomato_radius = 6.0;
    let mut size_global = ratatui::layout::Rect::default();

    while now.elapsed() < Duration::from_secs(17) {
        terminal.draw(|f| {
            let size = f.area();
            size_global = size.clone();
            let canvas = Canvas::default()
                .block(
                    Block::default()
                        .borders(ratatui::widgets::Borders::ALL)
                        .title("WheekHigh, RIP MANJOO").title_alignment(Alignment::Center),
                )
                .x_bounds([0.0, (size.width * 2) as f64])
                .y_bounds([0.0, (size.height * 2) as f64])
                .marker(Marker::HalfBlock)
                .paint(|ctx| {
                    if !has_tomato {
                        ctx.draw(&Tomato {
                            x: size.width as f64 / 2.0,
                            y: size.height as f64 / 2.0,
                            radius: tomato_radius,
                        });
                    }
                    if render_static {
                        ctx.draw(&Manjoo {
                            scale: 2,
                            is_static: true,
                            x_position: running_manjoo_x,
                            running_flag,
                            has_tomato: false,
                        });
                    } else {
                        ctx.draw(&Manjoo {
                            scale: 2,
                            is_static: false,
                            x_position: running_manjoo_x,
                            running_flag,
                            has_tomato,
                        })
                    }
                });
            f.render_widget(canvas, size);
        })?;
        if render_static {
            render_static = !render_static;
            thread::sleep(Duration::from_secs(2));
        }
        if !render_static {
            running_flag = !running_flag;
            running_manjoo_x += 1;
            if running_manjoo_x > (size_global.width * 2) as usize {
                running_manjoo_x = 0;
            }
            let tomato_x = (size_global.width as f64 / 2.0) as usize - (tomato_radius as usize) * 8;
            if running_manjoo_x >= (tomato_x) {
                sleep_duration = 100;
                has_tomato = true;
            }
        }
        thread::sleep(Duration::from_millis(sleep_duration));
    }

    terminal.show_cursor()?;
    terminal.flush()?;
    Ok(())
}
