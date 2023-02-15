use rumiga_core::{game_base, player::Player};
use std::cmp::Ordering;
use rand;

enum GuessNumberPlayerRole {
    Picker,
    Guesser,
}

struct GuessNumberMove {
    number: u8
}
struct GuessNumberPlayer {
    role: GuessNumberPlayerRole,
}

impl GuessNumberPlayer {
    fn new(role: GuessNumberPlayerRole) -> Self {
        Self { role }
    }
}

impl Player for GuessNumberPlayer {
    fn get_next_move(&self, game: &GuessNumberGame) -> Move {
        if self.role == GuessNumberPlayerRole::Picker && game.picked_number.is_none() {
            game.picked_number = rand::random()
        }
        println!()
    }
}
struct GuessNumberGame {
    picked_number: Option<u8>,
    last_guess_result: Option<Ordering>,
    attempts_count: u8,
}

impl GuessNumberGame {
    fn run(&self) {
        let picker = GuessNumberPlayer::new(Picker);
        let guesser = GuessNumberPlayer::new(Guesser);

        // Picker is picking number
    }
}

impl game_base::BasicGame for GuessNumberGame {
    fn start(&self) {
        todo!()
    }

    fn do_move(&self) -> game_base::MoveStatus {
        todo!()
    }

    fn check_complete(&self) -> game_base::GameStatus {
        todo!()
    }
}
