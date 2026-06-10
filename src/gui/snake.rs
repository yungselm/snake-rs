use iced::{Element, Subscription, Task};

use super::message::Message;

pub struct Snake {}

impl Snake {
    pub fn new() -> (Self, Task<Message>) {
        todo!()
    }

    pub fn update(&mut self, _message: Message) -> Task<Message> {
        todo!()
    }

    pub fn view(&self) -> Element<'_, Message> {
        todo!()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        todo!()
    }
}
