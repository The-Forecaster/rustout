// Author: Austin the Forecaster
// Since: 12/5/22
// Breakout in rust

use piston_window::{PistonWindow, WindowSettings, clear, rectangle};

fn main() {
    let mut app: PistonWindow = WindowSettings::new("Breakout", [500, 500]).exit_on_esc(true).build().unwrap();

    while let Some(event) = &mut app.next() {
        app.draw_2d(event, |context, graphics, _device| {
            clear([1.0; 4], graphics);

            rectangle([1.0, 0.0, 0.0, 1.0], [0.0, 0.0, 100.0, 100.0], context.transform, graphics);
        });
    }
}
