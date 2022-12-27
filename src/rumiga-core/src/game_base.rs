
pub enum MoveStatus {
    Success,
    InvalidMove
}

pub enum GameStatus {
    Playing,
    Draw,
    Player1Win,
    Player2Win
}

pub trait BasicGame {
    fn start(&self);
    fn do_move(&self) -> MoveStatus;
    fn undo_move(&self) -> MoveStatus;
    fn check_complete(&self) -> GameStatus;
}