use crate::game_base::BasicGame;

pub trait Player<G: BasicGame> {
    fn get_next_move(&self, game: &G);
    fn notify(&self, message: &str);
}