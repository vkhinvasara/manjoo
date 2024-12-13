mod tomato;
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

    let now = Instant::now();
    terminal.clear()?;
    while now.elapsed() < Duration::from_secs(10) {
        terminal.draw(|f| {
            let size = f.area();
            let canvas = Canvas::default()
                .block(Block::default().borders(ratatui::widgets::Borders::ALL).title("Tomato"))
                .x_bounds([0.0, size.width as f64])
                .y_bounds([0.0, size.height as f64])
                .marker(Marker::HalfBlock)
                .paint(|ctx| {
                    ctx.draw(&Tomato {
                        x: size.width as f64 / 2.0,
                        y: size.height as f64 / 2.0,
                        radius:5.0,
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