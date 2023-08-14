pub struct Tile {
    letter: char,
    usage: char,
    points: u8,
}

// TODO: Make linter happy
impl Tile {
    pub fn new(letter: char, points: u8) -> Tile {
        Tile {
            letter,
            usage: letter,
            points
        }
    }

    pub fn get_letter(&self) -> char {
        self.letter
    }

    pub fn get_points(&self) -> u8 {
        self.points
    }

    pub fn is_blank(&self) -> bool {
        self.letter == '?'
    }

    pub fn use_as(&mut self, usage: char) {
        self.usage = usage
    }

    pub fn get_usage(&self) -> char {
        self.usage
    }
}

#[cfg(test)]
mod tests {
    use crate::Tile;

    #[test]
    fn test_get_letter() {
        let tile = Tile::new('?', 0);
        assert_eq!(tile.get_letter(), '?');
    }

    #[test]
    fn test_get_points() {
        let tile = Tile::new('?', 0);
        assert_eq!(tile.get_points(), 0);
    }

    #[test]
    fn test_is_blank() {
        let tile = Tile::new('?', 0);
        assert!(tile.is_blank());
    }

    #[test]
    fn test_get_usage() {
        let tile = Tile::new('a', 1);
        assert_eq!(tile.get_usage(), 'a');
    }

    #[test]
    fn test_use_as() {
        let mut tile = Tile::new('?', 0);
        tile.use_as('a');
        assert_eq!(tile.get_usage(), 'a');
    }
}