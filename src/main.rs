use iced::window;

mod gui;

use gui::message::Message;
use gui::snake::Snake;

pub const X_COORDS: u32 = 800;
pub const Y_COORDS: u32 = 60;
pub const DISCRETIZATION_STEP: u32 = 10;

fn main() -> iced::Result {
    // Since we need to draw "pixels" with the element size we need
    // to cheat a bit and create a slighlty larger display window
    let x = X_COORDS + DISCRETIZATION_STEP;
    let y = Y_COORDS + DISCRETIZATION_STEP;

    iced::application("Snake", Snake::update, Snake::view)
        .subscription(Snake::subscription)
        .window(window::Settings {
            size: iced::Size::new(x, y),
            ..Default::default()
        })
        .run_with(Snake::new)
}
