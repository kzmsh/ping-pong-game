use crate::draw::to_coord;
use crate::game::Game;
use piston_window::types::Color;
use piston_window::{
    clear, Button, PistonWindow, PressEvent, ReleaseEvent, Size, UpdateEvent, WindowSettings,
};

mod ball;
mod draw;
mod game;
mod paddle;

const BACKGROUND_COLOR: Color = [0.0, 0.0, 0.0, 1.0];

fn main() {
    let (width, height) = (50, 55);

    let mut window: PistonWindow = WindowSettings::new(
        "Ping Pong Game",
        Size {
            width: to_coord(width as f64),
            height: to_coord(height as f64),
        },
    )
    .exit_on_esc(true)
    .build()
    .unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();

    let mut glyphs = window
        .load_font(assets.join("FiraSans-Regular.ttf"))
        .unwrap();

    let mut game = Game::new(width, height);
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }
        if let Some(Button::Keyboard(_)) = event.release_args() {
            game.key_released();
        }

        window.draw_2d(&event, |ctx, g, device| {
            clear(BACKGROUND_COLOR, g);
            game.draw(&ctx, g, &mut glyphs);
            glyphs.factory.encoder.flush(device);
        });

        event.update(|arg| game.update(arg.dt));
    }
}
