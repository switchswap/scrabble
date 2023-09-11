use crate::board::Board;
use crate::player::Player;
use crate::square::Square;

use comfy_table::presets::UTF8_FULL;
use comfy_table::*;

pub fn print_board(board: &Board) {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);

    for row in 0..board.rows() {
        let mut row_cells: Vec<Cell> = Vec::new();
        for col in 0..board.cols() {
            let square = board.square_at(col, row).unwrap();
            let cell_color = if square.is_start() {
                Color::Yellow
            } else if square.letter_multiplier() == 2 {
                Color::Cyan
            } else if square.letter_multiplier() == 3 {
                Color::Blue
            } else if square.word_multiplier() == 2 {
                Color::Magenta
            } else if square.word_multiplier() == 3 {
                Color::Red
            } else {
                Color::Reset
            };
            let cell = Cell::new(get_square_text(square))
                .bg(cell_color)
                .fg(Color::Black);
            row_cells.push(cell);
        }
        table.add_row(row_cells);
    }
    println!("{table}")
}

fn get_square_text(square: &Square) -> String {
    let multiplier_text = if square.word_multiplier() == 2 {
        "W2"
    } else if square.word_multiplier() == 3 {
        "W3"
    } else if square.letter_multiplier() == 2 {
        "L2"
    } else if square.letter_multiplier() == 3 {
        "L3"
    } else {
        ""
    };

    format!("{:>5}\n{:^5}\n{:>5}", " ", square.letter(), multiplier_text)
}

pub fn print_hand(player: &Player) {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    let mut row_cells: Vec<Cell> = Vec::new();
    for tile in player.hand_tiles() {
        let cell_text = format!(
            "{:>5}\n{:^5}\n{:>5}",
            " ",
            tile.get_letter(),
            tile.get_points()
        );
        row_cells.push(Cell::new(cell_text))
    }
    table.add_row(row_cells);
    println!("{table}")
}
