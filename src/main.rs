use iced::window;

mod gui;

use gui::message::Message;
use gui::snake::Snake;

fn main() -> iced::Result {
    iced::application("Snake", Snake::update, Snake::view)
        .subscription(Snake::subscription)
        .window(window::Settings {
            size: iced::Size::new(800.0, 600.0),
            ..Default::default()
        })
        .run_with(Snake::new)
}
