mod cell;
mod draw;
mod game;

use cell::Cell;
use draw::cell_to_gui_coords;
use game::Game;
use piston_window::*;

const WINDOW_TITLE: &str = "Conway's Game of Life";
const DEFAULT_WIDTH: u32 = 160;
const DEFAULT_HEIGHT: u32 = 80;

fn main() {
    let mut game = Game::new(DEFAULT_WIDTH, DEFAULT_HEIGHT);
    let size = [
        cell_to_gui_coords(game.width),
        cell_to_gui_coords(game.height),
    ];
    let mut window: piston_window::PistonWindow =
        piston_window::WindowSettings::new(WINDOW_TITLE, size)
            .exit_on_esc(true)
            .build()
            .unwrap();

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::R => game.fill_random(),
                _ => (),
            }
        }

        window.draw_2d(&event, |c, g, _| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            game.draw(&c, g);
        });

        event.update(|arg| game.update(arg.dt));
    }
}
