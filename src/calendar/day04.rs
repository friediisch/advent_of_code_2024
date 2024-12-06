use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use std::{
    fs::read_to_string,
    sync::atomic::{AtomicU64, Ordering},
};

#[derive(Debug)]
enum Move {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl Move {
    fn get(&self) -> (i32, i32) {
        match self {
            Move::Up => (0, -1),
            Move::UpRight => (1, -1),
            Move::Right => (1, 0),
            Move::DownRight => (1, 1),
            Move::Down => (0, 1),
            Move::DownLeft => (-1, 1),
            Move::Left => (-1, 0),
            Move::UpLeft => (-1, -1),
        }
    }

    fn iter() -> impl Iterator<Item = Move> {
        [
            Move::Up,
            Move::UpRight,
            Move::Right,
            Move::DownRight,
            Move::Down,
            Move::DownLeft,
            Move::Left,
            Move::UpLeft,
        ]
        .into_iter()
    }
}

struct Position {
    col: i32,
    row: i32,
}

fn count_word_in_grid(grid: Vec<Vec<char>>, search_word: &str) -> u64 {
    let word_count = AtomicU64::new(0);
    let n_cols = grid.len();
    let n_rows = grid.get(0).expect("No rows").len();
    grid.par_iter().enumerate().for_each(|(row_idx, col)| {
        if col.len() != n_rows {
            panic!("Not all rows have equal length")
        }
        col.par_iter().enumerate().for_each(|(col_idx, &c)| {
            if search_word.starts_with(c) {
                if search_word.len() == 1 {
                    word_count.fetch_add(1, Ordering::Relaxed);
                } else {
                    // explore moves in all directions for viable candidate directions
                    for _move in Move::iter() {
                        let (d_col, d_row) = _move.get();
                        let mut pos = Position {
                            col: col_idx as i32 + d_col,
                            row: row_idx as i32 + d_row,
                        };
                        let mut offset: usize = 1;
                        let mut next_search_word_char = search_word.chars().nth(offset).unwrap();
                        while !(pos.col < 0
                            || pos.row < 0
                            || pos.col >= n_cols as i32
                            || pos.row >= n_rows as i32)
                            && grid[pos.row as usize][pos.col as usize] == next_search_word_char
                            && offset < search_word.len()
                        {
                            offset += 1;
                            if let Some(search_word_char) = search_word.chars().nth(offset) {
                                next_search_word_char = search_word_char;
                            } else {
                                word_count.fetch_add(1, Ordering::Relaxed);
                                break;
                            };
                            pos.col += d_col;
                            pos.row += d_row;
                        }
                    }
                }
            }
        })
    });
    word_count.load(Ordering::Acquire)
}

pub fn day04() {
    println!("Day 4:");
    let file = read_to_string("src/data/day04.txt").expect("Unable to open file");

    let mut grid: Vec<Vec<char>> = vec![];
    for line in file.lines() {
        grid.push(line.chars().collect())
    }
    let xmas_count = count_word_in_grid(grid, "XMAS");

    println!("XMAS count: {}", xmas_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_word_in_grid() {
        let grid = vec![
            vec!['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M'],
            vec!['M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A'],
            vec!['A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M'],
            vec!['M', 'S', 'A', 'M', 'A', 'S', 'M', 'S', 'M', 'X'],
            vec!['X', 'M', 'A', 'S', 'A', 'M', 'X', 'A', 'M', 'M'],
            vec!['X', 'X', 'A', 'M', 'M', 'X', 'X', 'A', 'M', 'A'],
            vec!['S', 'M', 'S', 'M', 'S', 'A', 'S', 'X', 'S', 'S'],
            vec!['S', 'A', 'X', 'A', 'M', 'A', 'S', 'A', 'A', 'A'],
            vec!['M', 'A', 'M', 'M', 'M', 'X', 'M', 'M', 'M', 'M'],
            vec!['M', 'X', 'M', 'X', 'A', 'X', 'M', 'A', 'S', 'X'],
        ];
        let count = count_word_in_grid(grid, "XMAS");
        assert_eq!(count, 18);
    }
}
