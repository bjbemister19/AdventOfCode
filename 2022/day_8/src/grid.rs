use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug, Clone)]
pub struct Grid {
    grid: Vec<Vec<i32>>,
}

impl Grid {
    pub fn from_file(path: &File) -> Option<Grid> {
        let reader = BufReader::new(path);
        let vec: Vec<Vec<i32>> = reader
            .lines()
            .map(|line| {
                line.ok()
                    .unwrap()
                    .chars()
                    .map(|c| c.to_string().parse().unwrap())
                    .collect()
            })
            .collect();
        Some(Grid { grid: vec })
    }

    pub fn get(&self, col: usize, row: usize) -> Option<i32> {
        if self.grid.len() > 0 && col < self.grid[0].len() && row < self.grid.len() {
            return Some(self.grid[row][col]);
        }
        None
    }

    pub fn num_cols(&self) -> usize {
        self.grid[0].len()
    }

    pub fn num_rows(&self) -> usize {
        self.grid.len()
    }
}

pub struct GridIntoIter {
    grid: Grid,
    i: usize,
}

impl IntoIterator for Grid {
    type Item = (usize, usize, i32);
    type IntoIter = GridIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        GridIntoIter { grid: self, i: 0 }
    }
}

impl Iterator for GridIntoIter {
    type Item = (usize, usize, i32);

    fn next(&mut self) -> Option<Self::Item> {
        let num_rows = self.grid.grid.len();
        let num_cols = self.grid.grid[0].len();
        let max = num_rows * num_cols;
        let row = self.i / num_rows;
        let col = self.i % num_cols;

        if self.i < max {
            self.i = self.i + 1;
            return Some((col, row, self.grid.grid[row][col]));
        }
        None
    }
}
