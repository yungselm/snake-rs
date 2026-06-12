use iced::{Element, Subscription, Task};
use rand::seq::SliceRandom;

use super::message::Message;
use crate::{X_COORDS, Y_COORDS, DISCRETIZATION_STEP};

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
    direction: Direction,
    velocity: u32, //pixels per second
    color: (u8 , u8, u8),
}

impl Player {
    pub fn new() -> Self {
        Player {
            position: vec![
                Coords {x: 405,y: 305,}, // head 
                Coords {x: 395, y: 305}, // tail
            ],
            length: 2,
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
            Direction::Up => Coords { x: old_head.x, y: old_head.y -DISCRETIZATION_STEP },
            Direction::Down => Coords { x: old_head.x, y: old_head.y + DISCRETIZATION_STEP },
            Direction::Left => Coords { x: old_head.x - DISCRETIZATION_STEP, y: old_head.y },
            Direction::Right => Coords { x: old_head.x + DISCRETIZATION_STEP, y: old_head.y },
        };
        self.direction = direction;
        self.position.insert(0, new_head);
        self.position.pop();
        self
    }
}

pub struct Game {
    size: (f64, f64),
    snake_obj: Player,
    fruit_pos: Coords,
}

impl Game {
    pub fn new() -> Self {
        let snake = Player::new();
        let fruit = Game::set_fruit(&snake.position, (800.0, 600.0));
        Game { size: (800.0, 600.0), snake_obj: snake, fruit_pos: fruit }
    }
    
    fn set_fruit(snake_positions: &[Coords], size: (f64, f64)) -> Coords {
        let x_steps = X_COORDS / DISCRETIZATION_STEP;
        let y_steps = Y_COORDS / DISCRETIZATION_STEP;

        let candidates: Vec<Coords> = (0..x_steps)
            .flat_map(|xi| {
                (0..y_steps).map(move |yi| Coords {
                    x: 5 + xi * DISCRETIZATION_STEP,
                    y: 5 + yi * DISCRETIZATION_STEP,
                })
            })
            .filter(|c| !snake_positions.iter().any(|s| s.x == c.x && s.y == c.y))
            .collect();
        
        let mut rng = rand::thread_rng();
        candidates.choose(&mut rng).cloned().expect("no valid position")
    }
}

pub enum GameState {
    Menu,
    Running,
    GameOver,
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
