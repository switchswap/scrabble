use crate::bag::Bag;
use crate::board::Board;
use crate::dictionary::Dictionary;
use crate::game::Game;
use crate::player::Player;

mod bag;
mod board;
mod console_printer;
mod dictionary;
mod game;
mod r#move;
mod player;
mod square;
mod tile;

fn main() {
    println!("Hello Scrabble!");

    // TODO: Get rid of separate init game and have Game read the config file
    let mut game = Game::new("");
    game.init_game();

    while game.winners().is_empty() {
        game.next_turn();
    }
}
