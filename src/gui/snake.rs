use iced::{Element, Subscription, Task};

use super::message::Message;

#[derive(Copy, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn is_opposite(&self, other: Direction) -> bool {
        match (self, other) {
            (Direction::Up, Direction::Down) => true,
            (Direction::Down, Direction::Up) => true,
            (Direction::Left, Direction::Right) => true,
            (Direction::Right, Direction::Left) => true,
            _ => false,
        }
    }
}

#[derive(Clone)]
pub struct Coords {
    x: u32,
    y: u32,
}

pub struct Player {
    position: Vec<Coords>,
    length: u32,
    element_size: u32,
    direction: Direction,
    velocity: u32, //pixels per second
    color: (u8 , u8, u8),
}

impl Player {
    pub fn new() -> Self {
        Player {
            position: vec![
                Coords {x: 400,y: 300,}, // head 
                Coords {x: 390, y: 300}, // tail
            ],
            length: 2,
            element_size: 10,
            direction: Direction::Right,
            velocity: 10,
            color: (255, 255, 255),
        }
    }

    pub fn move_direction(self, new_direction: Direction) -> Self {
        match new_direction {
            d if self.direction.is_opposite(d) => self,
            d => self.add_unit_direction(d),
        }
    }

    fn add_unit_direction(mut self, direction: Direction) -> Self {
        let old_head = self.position[0].clone();
        let new_head = match direction {
            Direction::Up => Coords { x: old_head.x, y: old_head.y - self.element_size },
            Direction::Down => Coords { x: old_head.x, y: old_head.y + self.element_size },
            Direction::Left => Coords { x: old_head.x - self.element_size, y: old_head.y },
            Direction::Right => Coords { x: old_head.x + self.element_size, y: old_head.y },
        };
        self.direction = direction;
        self.position.insert(0, new_head);
        self.position.pop();
        self
    }
}

pub struct Game {
    size: (u8, u8),
    snake_obj: Player,
    fruit_pos: Vec<Coords>,
}

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
