use crate::bag::Bag;
use crate::board::Board;
use crate::console_printer;
use crate::dictionary::Dictionary;
use crate::player::Player;
use crate::r#move::Move;
use crate::r#move::Move::Pass;

pub struct Game {
    bag: Bag,
    board: Board,
    dictionary: Dictionary,
    players: Vec<Player>,
    winners: Vec<Player>,
    curr_player_index: usize,
    pass_count: usize,
}

impl Game {
    pub fn new(config_file: &str) -> Game {
        // TODO: Parse config file for this info
        let bag = Bag::new("bag.txt", 2023);
        let board = Board::new("board.txt");
        let dictionary = Dictionary::new("dictionary.txt");
        let player1 = Player::new("Player 1", 7);
        let player2 = Player::new("Player 2", 7);
        let players: Vec<Player> = Vec::from([player1, player2]);

        Game {
            bag,
            board,
            dictionary,
            players,
            winners: Vec::new(),
            curr_player_index: 0,
            pass_count: 0,
        }
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn init_game(&mut self) {
        for player in self.players.iter_mut() {
            let mut tiles = self.bag.draw_tiles(player.max_tiles());
            player.add_tiles(&mut tiles);
        }
    }

    pub fn players(&self) -> &Vec<Player> {
        &self.players
    }

    pub fn winners(&self) -> &Vec<Player> {
        &self.winners
    }

    pub fn next_turn(&mut self) {
        println!(
            "Player: {}\n\
            Current score: {}",
            self.players[self.curr_player_index].name(),
            self.players[self.curr_player_index].score()
        );
        console_printer::print_board(&self.board);
        console_printer::print_hand(&self.players[self.curr_player_index]);

        // TODO: Consider extracting to function
        // Get move from player
        loop {
            println!("Your move:");
            // Get user input and split to tokens
            let mut move_string = String::new();
            std::io::stdin()
                .read_line(&mut move_string)
                .expect("Failed to read input!");

            let player_move = Move::parse_move(move_string.as_str());
            match player_move {
                None => {
                    println!("Invalid syntax!");
                    continue;
                }
                Some(player_move) => {
                    let move_result = player_move.execute(
                        &mut self.players[self.curr_player_index],
                        &mut self.board,
                        &mut self.bag,
                        &self.dictionary,
                    );
                    match move_result {
                        Ok(_) => {
                            if let Pass = player_move {
                                self.pass_count += 1;
                            }
                        }
                        Err(e) => {
                            println!("Invalid move: {e}");
                            continue;
                        }
                    }
                }
            }
            break;
        }

        console_printer::print_board(&self.board);
        console_printer::print_hand(&self.players[self.curr_player_index]);

        // Increment player but wrap around count
        self.curr_player_index = (self.curr_player_index + 1) % self.players.len();

        println!("Press [ENTER] to continue...");
        std::io::stdin().read_line(&mut String::new());
    }
}
