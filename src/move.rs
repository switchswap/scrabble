use crate::{Bag, Board, Dictionary, Player};

pub enum Move {
    Pass,
    Exchange {
        tile_string: String,
    },
    Place {
        x: usize,
        y: usize,
        horizontal: bool,
        tile_string: String,
    },
}

impl Move {
    pub fn parse_move(move_string: &str) -> Option<Move> {
        let tokens: Vec<&str> = move_string.split_whitespace().collect();

        if let Some(move_type) = tokens.first() {
            // Try to parse move from tokens
            match *move_type {
                "PASS" => return Some(Move::Pass),
                "EXCHANGE" => {
                    // EXCHANGE <tile string>
                    if let Some(exchange_move) = Move::parse_exchange_move(&tokens) {
                        return Some(exchange_move);
                    }
                }
                "PLACE" => {
                    // PLACE <direction> <x> <y> <tile string>
                    if let Some(place_move) = Move::parse_place_move(&tokens) {
                        return Some(place_move);
                    }
                }
                _ => {}
            };
        }
        None
    }

    fn parse_exchange_move(tokens: &Vec<&str>) -> Option<Move> {
        if tokens.len() != 2 {
            return None;
        }

        Some(Move::Exchange {
            tile_string: tokens[1].to_string(),
        })
    }

    fn parse_place_move(tokens: &Vec<&str>) -> Option<Move> {
        if tokens.len() != 5 {
            return None;
        }

        let direction = tokens[1].to_ascii_uppercase();
        let horizontal = match direction.as_str() {
            "ACROSS" => true,
            "DOWN" => false,
            _ => return None,
        };

        let x = match tokens[2].parse::<usize>() {
            Ok(num) => num,
            Err(_) => return None,
        };

        let y = match tokens[3].parse::<usize>() {
            Ok(num) => num,
            Err(_) => return None,
        };

        Some(Move::Place {
            x,
            y,
            horizontal,
            tile_string: tokens[4].to_string(),
        })
    }

    pub fn execute(
        &self,
        player: &mut Player,
        board: &mut Board,
        bag: &mut Bag,
        dictionary: &Dictionary,
    ) -> Result<(), &'static str> {
        match self {
            Move::Pass => {}
            Move::Exchange { tile_string } => {
                if !player.has_tiles(tile_string, false) {
                    return Result::Err("Tile not in hand!");
                }

                let mut exchange_tiles = player.take_tiles(tile_string, false);
                if !bag.tiles_remaining() < exchange_tiles.len() {
                    // Add the tiles back since we're not using them anymore
                    // TODO: I don't like this. Can't we do this check with just the tile string?
                    player.add_tiles(&mut exchange_tiles);
                    return Result::Err("Not enough tiles in bag!");
                }

                player.add_tiles(&mut bag.draw_tiles(exchange_tiles.len()));
                bag.add_tiles(&mut exchange_tiles);
            }
            Move::Place {
                x,
                y,
                horizontal,
                tile_string,
            } => {
                // pass
                return Result::Err("Not yet implemented!");
            }
        }
        Result::Ok(())
    }
}
