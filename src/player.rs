use crate::tile::Tile;

pub struct Player {
    name: String,
    max_tiles: usize,
    tiles: Vec<Tile>,
    score: u32,
}

impl Player {
    pub fn new(name: &str, max_tiles: usize) -> Player {
        Player {
            name: String::from(name),
            max_tiles,
            tiles: Vec::new(),
            score: 0,
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn max_tiles(&self) -> usize {
        self.max_tiles
    }

    pub fn hand_tiles(&self) -> &Vec<Tile> {
        &self.tiles
    }

    pub fn add_tiles(&mut self, tiles: &mut Vec<Tile>) {
        self.tiles.append(tiles)
    }

    pub fn has_tiles(&self, tile_string: &str, resolve_blanks: bool) -> bool {
        let mut iter = tile_string.chars();
        while let Some(curr_letter) = iter.next() {
            let has_tile = Player::has_tile(self, curr_letter);
            if !has_tile {
                return false;
            }

            if curr_letter == '?' && resolve_blanks {
                // If we're resolving blanks, consume the next letter since we'll use this blank to
                // create it
                iter.next();
            }
        }
        true
    }

    fn has_tile(&self, tile_letter: char) -> bool {
        return self
            .tiles
            .iter()
            .any(|tile| tile.get_letter() == tile_letter);
    }

    /// Takes tiles out of hand and returns them
    ///
    /// # Arguments
    ///
    /// * `tile_string`: Tile string such as "cdef?g"
    /// * `resolve_blanks`: Whether to resolve blanks or not
    ///
    /// returns: Vec<Tile, Global>
    pub fn take_tiles(&mut self, tile_string: &str, resolve_blanks: bool) -> Vec<Tile> {
        let mut tiles = Vec::new();
        let mut iter = tile_string.chars();
        while let Some(curr_letter) = iter.next() {
            let tile_index = self
                .tiles
                .iter()
                .position(|tile| tile.get_letter() == curr_letter)
                .expect("Tile not found in hand!");

            let mut tile = self.tiles.remove(tile_index);
            // Set usage if blank tile
            if curr_letter == '?' && resolve_blanks {
                // iter.next() will consume the next tile
                tile.use_as(iter.next().expect("No usage instructions after blank!"))
            }
            tiles.push(tile);
        }
        tiles
    }
}
