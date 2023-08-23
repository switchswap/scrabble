use crate::bag::Bag;
use crate::board::Board;

mod bag;
mod board;
mod console_printer;
mod dictionary;
mod square;
mod tile;

fn main() {
    println!("Hello Scrabble!");
    let mut board = Board::new("board.txt");
    console_printer::print_board(&mut board);
}
