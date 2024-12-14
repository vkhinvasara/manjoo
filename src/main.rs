mod tomato;
mod manjoo;
use manjoo::Manjoo;
use ratatui::{
    backend::TermwizBackend,
    style::Color,
    symbols::Marker,
    widgets::{canvas::{self, Canvas, Circle, Map}, Block, Paragraph},
    Terminal,
};
use tomato::Tomato;
use std::{
    error::Error,
    io::stdout,
    thread,
    time::{Duration, Instant},
};

fn main() -> Result<(), Box<dyn Error>> {
    let backend = TermwizBackend::new()?;
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    terminal.clear()?;
    let now = Instant::now();
    while now.elapsed() < Duration::from_secs(10) {
        terminal.draw(|f| {
            let size = f.area();
            let canvas = Canvas::default()
                .block(Block::default().borders(ratatui::widgets::Borders::ALL).title("Manjoo"))
                .x_bounds([0.0, (size.width*2) as f64])
                .y_bounds([0.0, (size.height*2) as f64])
                .marker(Marker::HalfBlock)
                .paint(|ctx| {
                    ctx.draw(&Tomato {
                        x: size.width as f64 / 2.0,
                        y: size.height as f64 / 2.0,
                        radius:6.0,
                    });
                    ctx.draw(&Manjoo{
                       scale:2
                    });
                });
            f.render_widget(canvas, size);
        })?;
        thread::sleep(Duration::from_millis(250));
    }

    terminal.show_cursor()?;
    terminal.flush()?;
    Ok(())
}