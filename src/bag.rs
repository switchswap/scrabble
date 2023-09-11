use crate::tile::Tile;
use rand::prelude::StdRng;
use rand::rngs::ThreadRng;
use rand::{Rng, SeedableRng};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Bag {
    tiles: Vec<Tile>,
    letters: HashSet<char>,
    init_count: HashMap<char, u32>,
    current_count: HashMap<char, u32>,
    seed: u64,
}

impl Bag {
    pub fn new(file_name: &str, seed: u64) -> Bag {
        let file = File::open(file_name).unwrap();
        let reader = BufReader::new(file);

        let mut tiles: Vec<Tile> = Vec::new();
        let mut letters: HashSet<char> = HashSet::new();
        let mut init_count: HashMap<char, u32> = HashMap::new();
        let mut current_count: HashMap<char, u32> = HashMap::new();
        for line in reader.lines() {
            let (letter, points, count) = Self::parse_line(&(line.unwrap()));
            for _ in 0..count {
                tiles.push(Tile::new(letter, points))
            }
            letters.insert(letter);
            init_count.insert(letter, count);
            current_count.insert(letter, count);
        }

        Bag {
            tiles,
            letters,
            init_count,
            current_count,
            seed,
        }
    }

    fn parse_line(line: &str) -> (char, u32, u32) {
        let tile_info: Vec<&str> = line.split_whitespace().collect();
        let letter: char = tile_info[0].chars().next().unwrap();
        let points: u32 = tile_info[1].parse().unwrap();
        let count: u32 = tile_info[2].parse().unwrap();
        (letter, points, count)
    }

    pub fn add_tile(&mut self, tile: Tile) {
        self.tiles.push(tile)
    }

    pub fn add_tiles(&mut self, tiles: &mut Vec<Tile>) {
        self.tiles.append(tiles)
    }

    pub fn draw_tiles(&mut self, number: usize) -> Vec<Tile> {
        let mut rng: StdRng = rand::SeedableRng::seed_from_u64(self.seed);

        let mut tiles_drawn = Vec::new();
        while !self.tiles.is_empty() && tiles_drawn.len() < number {
            let rand_index = rng.gen_range(0..self.tiles.len());
            let tile = self.tiles.swap_remove(rand_index);

            let new_count = self.current_count.get(&tile.get_letter()).unwrap() - 1;
            self.current_count.insert(tile.get_letter(), new_count);
            tiles_drawn.push(tile);
        }

        tiles_drawn
    }

    pub fn tiles_remaining(&self) -> usize {
        self.tiles.len()
    }

    pub fn initial_tile_count(&self) -> &HashMap<char, u32> {
        &self.init_count
    }

    pub fn current_tile_count(&self) -> &HashMap<char, u32> {
        &self.current_count
    }

    pub fn all_letters(&self) -> &HashSet<char> {
        &self.letters
    }
}

#[cfg(test)]
mod tests {
    // TODO: Add tests
}
