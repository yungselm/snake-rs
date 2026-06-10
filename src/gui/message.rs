#[derive(Debug, Clone)]
pub enum Message {
    UpArrowPressed,
    DownArrowPressed,
    LeftArrowPressed,
    RightArrowPressed,
    FruitEaten,
    Collided,
}
