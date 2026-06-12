use iced::{Element, Subscription, Task, Rectangle, Renderer, Theme, mouse};
use iced::time::Duration;
use iced::Event::Keyboard;
use iced::widget::canvas;
use rand::seq::IndexedRandom;

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
    velocity: u32,
}

impl Player {
    pub fn new() -> Self {
        Player {
            position: vec![
                Coords { x: 405, y: 305 }, // head
                Coords { x: 395, y: 305 }, // tail
            ],
            length: 2,
            direction: Direction::Right,
            velocity: 10,
        }
    }

    fn move_direction(&mut self, direction: Direction) {
        let old_head = self.position[0].clone();
        let new_head = match direction {
            Direction::Up    => Coords { x: old_head.x,                        y: old_head.y - DISCRETIZATION_STEP },
            Direction::Down  => Coords { x: old_head.x,                        y: old_head.y + DISCRETIZATION_STEP },
            Direction::Left  => Coords { x: old_head.x - DISCRETIZATION_STEP,  y: old_head.y },
            Direction::Right => Coords { x: old_head.x + DISCRETIZATION_STEP,  y: old_head.y },
        };
        self.direction = direction;
        self.position.insert(0, new_head);
        self.position.pop();
    }
}

pub enum GameState {
    Running,
    GameOver,
}

pub struct Game {
    snake_obj: Player,
    fruit_pos: Coords,
    color_snake: (u8, u8, u8),
    color_fruit: (u8, u8, u8),
    game_state: GameState,
}

impl Game {
    pub fn new() -> Self {
        let snake = Player::new();
        let fruit = Game::set_fruit(&snake.position);
        Game {
            snake_obj: snake,
            fruit_pos: fruit,
            color_snake: (255, 255, 255),
            color_fruit: (255, 0, 0),
            game_state: GameState::Running,
        }
    }

    fn set_fruit(snake_positions: &[Coords]) -> Coords {
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

        let mut rng = rand::rng();
        candidates.choose(&mut rng).cloned().expect("no valid position")
    }

    pub fn check_game_state(self) -> Self {
        todo!()
    }
}

impl canvas::Program<Message> for Game {
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        let step = DISCRETIZATION_STEP as f32;

        frame.fill_rectangle(iced::Point::ORIGIN, bounds.size(), iced::Color::BLACK);

        self.snake_obj.position.iter().for_each(|s| {
            frame.fill_rectangle(
                iced::Point::new(s.x as f32 - 5.0, s.y as f32 - 5.0),
                iced::Size::new(step, step),
                iced::Color::from_rgb8(self.color_snake.0, self.color_snake.1, self.color_snake.2),
            );
        });

        frame.fill_rectangle(
            iced::Point::new(self.fruit_pos.x as f32 - 5.0, self.fruit_pos.y as f32 - 5.0),
            iced::Size::new(step, step),
            iced::Color::from_rgb8(self.color_fruit.0, self.color_fruit.1, self.color_fruit.2),
        );

        vec![frame.into_geometry()]
    }
}

pub struct Snake {
    game: Game,
}

impl Snake {
    pub fn new() -> (Self, Task<Message>) {
        (Snake { game: Game::new() }, Task::none())
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        let current_direction = self.game.snake_obj.direction.clone();

        match message {
            Message::TimeMove => { 
                self.game.snake_obj.move_direction(current_direction);
                Task::none() 
            },
            Message::UpArrowPressed => {
                if !self.game.snake_obj.direction.is_opposite(Direction::Up) {
                    self.game.snake_obj.direction = Direction::Up
                };
                Task::none()
            },
            Message::DownArrowPressed => {
                if !self.game.snake_obj.direction.is_opposite(Direction::Down) {
                    self.game.snake_obj.direction = Direction::Down
                };
                Task::none()
            },
            Message::LeftArrowPressed => {
                if !self.game.snake_obj.direction.is_opposite(Direction::Left) {
                    self.game.snake_obj.direction = Direction::Left
                };
                Task::none()
            },
            Message::RightArrowPressed => {
                if !self.game.snake_obj.direction.is_opposite(Direction::Right) {
                    self.game.snake_obj.direction = Direction::Right
                };
                Task::none()
            },
            Message::NoOp => Task::none(),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        canvas(&self.game)
            .width(X_COORDS + DISCRETIZATION_STEP)
            .height(Y_COORDS + DISCRETIZATION_STEP)
            .into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        use iced::keyboard::{Event as KeyEvent, Key, key};
        use iced::time::Duration;

        Subscription::batch([
            iced::time::every(Duration::from_millis(200)).map(|_| Message::TimeMove),
            iced::event::listen_with(|event, _, _| {
                if let Keyboard(KeyEvent::KeyPressed { key, .. }) = event {
                    match key {
                        Key::Named(key::Named::ArrowUp)    => Some(Message::UpArrowPressed),
                        Key::Named(key::Named::ArrowDown)  => Some(Message::DownArrowPressed),
                        Key::Named(key::Named::ArrowLeft)  => Some(Message::LeftArrowPressed),
                        Key::Named(key::Named::ArrowRight) => Some(Message::RightArrowPressed),
                        _ => Some(Message::NoOp),
                    }
                } else {
                    Some(Message::NoOp)
                }
            }),
        ])
    }
}
