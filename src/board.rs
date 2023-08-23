use crate::square::Square;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Board {
    grid: Vec<Square>,
    cols: usize,
    rows: usize,
    start_x: usize,
    start_y: usize,
}

impl Board {
    pub fn new(file_name: &str) -> Board {
        Self::parse_file(file_name)
    }

    fn parse_file(file_name: &str) -> Board {
        let file = File::open(file_name).unwrap();
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader.lines().map_while(Result::ok).collect();

        let (cols, rows) = Self::parse_coords(&lines[0]);
        let (start_x, start_y) = Self::parse_coords(&lines[1]);

        let mut grid: Vec<Square> = lines
            .iter()
            .skip(2)
            .filter(|line| !line.is_empty())
            .flat_map(|line| Self::parse_line(line.as_str()))
            .collect();

        // Assign the start square
        // Both indexes are subtracted by 1 to 0-index them
        if let Some(square) = grid.get_mut(cols * (start_y - 1) + start_x - 1) {
            square.set_start();
        }

        Board {
            grid,
            cols,
            rows,
            start_x,
            start_y,
        }
    }

    fn parse_line(line: &str) -> Vec<Square> {
        line.chars()
            .map(|character| match character {
                'd' => Square::new(1, 2, false),
                't' => Square::new(1, 3, false),
                '2' => Square::new(2, 1, false),
                '3' => Square::new(3, 1, false),
                _ => Square::new(1, 1, false),
            })
            .collect()
    }

    fn parse_coords(coords: &str) -> (usize, usize) {
        let coords_vec: Vec<&str> = coords.split_whitespace().collect();
        let x: usize = coords_vec[0].parse().unwrap();
        let y: usize = coords_vec[1].parse().unwrap();
        (x, y)
    }

    /// Returns the square at the given coordinates
    pub fn square_at(&mut self, x: usize, y: usize) -> Option<&mut Square> {
        let index = y * self.cols + x;
        self.grid.get_mut(index)
    }

    /// Returns the start square
    pub fn start_square(&mut self) -> Option<&mut Square> {
        self.square_at(self.start_x, self.start_y)
    }

    /// Returns the number of rows on the board
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Returns the number of columns on the board
    pub fn cols(&self) -> usize {
        self.cols
    }
}
