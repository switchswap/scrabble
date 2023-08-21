use crate::Tile;

pub struct Square {
    letter_multiplier: u32,
    word_multiplier: u32,
    orig_letter_multiplier: u32,
    orig_word_multiplier: u32,
    occupied: bool,
    is_start: bool,
    tile: Option<Tile>,
}

impl Square {
    /// Creates a new [Square]
    pub fn new(letter_multiplier: u32, word_multiplier: u32, is_start: bool) -> Square {
        Square {
            letter_multiplier,
            word_multiplier,
            orig_letter_multiplier: letter_multiplier,
            orig_word_multiplier: word_multiplier,
            occupied: false,
            is_start,
            tile: None,
        }
    }

    /// Places tile on [Square]
    pub fn place_tile(&mut self, tile: Tile) {
        self.occupied = true;
        self.tile = Some(tile);
    }

    /// Removes tile and resets multipliers
    pub fn remove_tile(&mut self) {
        self.occupied = false;
        self.tile = None;
        self.letter_multiplier = self.orig_letter_multiplier;
        self.word_multiplier = self.orig_word_multiplier;
    }

    /// Sets letter and word multipliers to 1 since they've been used and shouldn't be again
    pub fn use_square(&mut self) {
        self.letter_multiplier = 1;
        self.word_multiplier = 1;
    }

    /// Returns whether the tile is occupied
    pub fn is_occupied(&self) -> bool {
        self.occupied
    }

    /// Returns whether the tile is the start tile
    pub fn is_start(&self) -> bool {
        self.is_start
    }

    /// Makes the tile the start tile
    pub fn set_start(&mut self) {
        self.is_start = true
    }

    /// Returns tile score
    pub fn score(&self) -> u32 {
        match &self.tile {
            Some(tile) => tile.get_points(),
            None => 0,
        }
    }

    /// Returns the tile letter
    pub fn letter(&self) -> char {
        match &self.tile {
            Some(tile) => tile.get_usage(),
            None => ' ',
        }
    }

    /// Returns the letter multiplier
    pub fn letter_multiplier(&self) -> u32 {
        self.letter_multiplier
    }

    /// Returns the word multiplier
    pub fn word_multiplier(&self) -> u32 {
        self.word_multiplier
    }
}
