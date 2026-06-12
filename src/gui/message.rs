#[derive(Debug, Clone)]
pub enum Message {
    TimeMove,
    UpArrowPressed,
    DownArrowPressed,
    LeftArrowPressed,
    RightArrowPressed,
    NoOp
}
