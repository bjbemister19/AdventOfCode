use std::fs::File;

mod grid;
use grid::Grid;

fn is_visible(grid: &Grid, col: usize, row: usize, value: i32) -> bool {
    let mut i = row;
    let mut a = true;
    while i > 0 {
        if grid.get(col, i-1).unwrap() >= value {
            a = false;
        }
        i = i - 1;
    }

    i = row + 1;
    let mut b = true;
    while i < grid.num_rows() {
        if grid.get(col, i).unwrap() >= value {
            b = false;
        }
        i = i + 1;
    }

    i = col;
    let mut c = true;
    while i > 0 {
        if grid.get(i-1, row).unwrap() >= value {
            c = false;
        }
        i = i - 1;
    }

    i = col + 1;
    let mut d = true;
    while i < grid.num_cols() {
        if grid.get(i, row).unwrap() >= value {
            d = false;
        }
        i = i + 1;
    }
    a || b || c || d
}

fn p1() {
    let file = File::open("data/input.txt").unwrap();
    let grid = Grid::from_file(&file).unwrap();

    let mut count = 0;
    for (col, row, value) in grid.clone() {
        if is_visible(&grid, col, row, value) {
            count = count + 1;
        }
    }
    println!("Count: {}", count);
}

fn scenic_score(grid: &Grid, col: usize, row: usize, value: i32) -> usize {
    let mut i = row;
    let mut a = 0;
    while i > 0 {
        a = a + 1;
        if grid.get(col, i-1).unwrap() >= value {
            break;
        }
        i = i - 1;
    }

    i = row + 1;
    let mut b = 0;
    while i < grid.num_rows() {
        b = b + 1;
        if grid.get(col, i).unwrap() >= value {
            break;
        }
        i = i + 1;
    }

    i = col;
    let mut c = 0;
    while i > 0 {
        c = c + 1;
        if grid.get(i-1, row).unwrap() >= value {
            break;
        }
        i = i - 1;
    }

    i = col + 1;
    let mut d = 0;
    while i < grid.num_cols() {
        d = d + 1;
        if grid.get(i, row).unwrap() >= value {
            break;
        }
        i = i + 1;
    }
    a * b * c * d
}

fn p2() {
    let file = File::open("data/input.txt").unwrap();
    let grid = Grid::from_file(&file).unwrap();

    let mut highest_score = 0;
    for (col, row, value) in grid.clone() {
        let score = scenic_score(&grid, col, row, value);
        if score > highest_score {
            highest_score = score;
        }
    }
    println!("Highest Score: {}", highest_score);
}

fn main() {
    p1();
    p2();
}