use iced::{Element, Subscription, Task, Rectangle, Renderer, Theme, mouse};
use iced::Event::Keyboard;
use iced::widget::canvas;
use rand::seq::IndexedRandom;

use super::message::Message;
use crate::{X_COORDS, Y_COORDS, DISCRETIZATION_STEP};

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
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

#[derive(Clone, PartialEq)]
pub struct Coords {
    x: u32,
    y: u32,
}

#[derive(Clone)]
pub struct Player {
    position: Vec<Coords>,
    direction: Direction,
}

impl Player {
    pub fn new() -> Self {
        Player {
            position: vec![
                Coords { 
                    x: X_COORDS / 2 + DISCRETIZATION_STEP / 2 , 
                    y: Y_COORDS / 2 + DISCRETIZATION_STEP / 2 }, // head
                Coords { 
                    x: X_COORDS / 2 - DISCRETIZATION_STEP / 2, 
                    y: Y_COORDS / 2 + DISCRETIZATION_STEP / 2 }, // tail
            ],
            direction: Direction::Right,
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

    fn set_direction(&mut self, new_direction: Direction) {
        if !self.direction.is_opposite(new_direction) {
            self.direction = new_direction;
        }
    }

    fn add_fruit_head(&mut self) {
        let old_head = self.position[0].clone();
        let new_head = match self.direction {
            Direction::Up    => Coords { x: old_head.x,                        y: old_head.y - DISCRETIZATION_STEP },
            Direction::Down  => Coords { x: old_head.x,                        y: old_head.y + DISCRETIZATION_STEP },
            Direction::Left  => Coords { x: old_head.x - DISCRETIZATION_STEP,  y: old_head.y },
            Direction::Right => Coords { x: old_head.x + DISCRETIZATION_STEP,  y: old_head.y },
        };
        self.position.insert(0, new_head);
    }
}

#[derive(Copy, Clone)]
pub enum GameState {
    Running,
    GameOver,
}

#[derive(Clone)]
pub struct Game {
    snake_obj: Player,
    fruit_pos: Coords,
    color_snake: (u8, u8, u8),
    color_fruit: (u8, u8, u8),
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
        }
    }

    fn next_coords(pos: &Coords, direction: Direction) -> Option<Coords> {
        match direction {
            Direction::Up => {
                pos.y.checked_sub(DISCRETIZATION_STEP).map(|y| Coords { x: pos.x, y })
            }
            Direction::Down => {
                let y = pos.y + DISCRETIZATION_STEP;
                if y >= Y_COORDS { None } else { Some(Coords { x: pos.x, y }) }
            }
            Direction::Left => {
                pos.x.checked_sub(DISCRETIZATION_STEP).map(|x| Coords { x, y: pos.y })
            }
            Direction::Right => {
                let x = pos.x + DISCRETIZATION_STEP;
                if x >= X_COORDS { None } else { Some(Coords { x, y: pos.y }) }
            }
        }
    }

    pub fn check_game_state(&self) -> GameState {
        let head = &self.snake_obj.position[0];
        let direction = self.snake_obj.direction;

        let next = match Self::next_coords(head, direction) {
            None => return GameState::GameOver,
            Some(c) => c,
        };

        if next == self.fruit_pos {
            if Self::next_coords(&next, direction).is_none() {
                return GameState::GameOver;
            }
        }

        GameState::Running
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
                match self.game.check_game_state() {
                    GameState::GameOver => panic!(),
                    GameState::Running => {
                        self.game.snake_obj.move_direction(current_direction);
                        let head = &self.game.snake_obj.position[0];
                        if head.x == self.game.fruit_pos.x && head.y == self.game.fruit_pos.y {
                            self.game.snake_obj.add_fruit_head();
                            self.game.fruit_pos = Game::set_fruit(&self.game.snake_obj.position);
                        }
                    }
                };
                Task::none() 
            },
            Message::UpArrowPressed => {
                self.game.snake_obj.set_direction(Direction::Up);
                Task::none()
            },
            Message::DownArrowPressed => {
                self.game.snake_obj.set_direction(Direction::Down);
                Task::none()
            },
            Message::LeftArrowPressed => {
                self.game.snake_obj.set_direction(Direction::Left);
                Task::none()
            },
            Message::RightArrowPressed => {
                self.game.snake_obj.set_direction(Direction::Right);
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
            iced::time::every(Duration::from_millis(100)).map(|_| Message::TimeMove),
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
