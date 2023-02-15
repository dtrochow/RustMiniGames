pub mod runner;
pub mod player;

pub enum GameTitle {
    GuessNumber,
}

#[no_mangle]
pub extern "C" fn run_game(game_title: GameTitle) {
    let game = match game_title {
        GuessNumber => GuessNumberGame{},
    };
    let game_runner = runner::GameRunner { game };
}
