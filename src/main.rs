use iced::window;

mod gui;

use gui::snake::Snake;

pub const X_COORDS: u32 = 800;
pub const Y_COORDS: u32 = 600;
pub const DISCRETIZATION_STEP: u32 = 10;

fn main() -> iced::Result {
    // Since we need to draw "pixels" with the element size we need
    // to cheat a bit and create a slighlty larger display window
    let x = X_COORDS + DISCRETIZATION_STEP;
    let y = Y_COORDS + DISCRETIZATION_STEP;

    iced::application(Snake::new, Snake::update, Snake::view)
        .title("Snake")
        .subscription(Snake::subscription)
        .window(window::Settings {
            size: iced::Size::new(x as f32, y as f32),
            ..Default::default()
        })
        .run()
}
