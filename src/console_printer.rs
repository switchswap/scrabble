use crate::board::Board;
use comfy_table::presets::UTF8_FULL;
use comfy_table::*;

pub fn print_board(board: &mut Board) {
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
            let cell = Cell::new(square.letter().to_string().as_str())
                .bg(cell_color)
                .fg(Color::Black);
            row_cells.push(cell);
        }
        table.add_row(row_cells);
    }
    println!("{table}")
}
