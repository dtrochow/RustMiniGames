use crate::game_base::BasicGame;



pub trait GameRunner {
    fn run(&self, game: &dyn BasicGame) {

    }
}

