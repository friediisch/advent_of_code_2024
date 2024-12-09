use rayon::prelude::*;
use std::{
    collections::HashSet,
    fs::read_to_string,
    sync::atomic::{AtomicU64, Ordering},
};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn next_move(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        }
    }

    fn turn(&mut self) {
        *self = match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

impl From<Direction> for char {
    fn from(direction: Direction) -> char {
        match direction {
            Direction::Up => '^',
            Direction::Right => '>',
            Direction::Down => 'v',
            Direction::Left => '<',
        }
    }
}

impl From<char> for Direction {
    fn from(c: char) -> Direction {
        match c {
            '^' => Direction::Up,
            '>' => Direction::Right,
            'v' => Direction::Down,
            '<' => Direction::Left,
            _ => panic!("Invalid direction character: {}", c),
        }
    }
}

fn get_guard_state(map: &Vec<Vec<char>>) -> (i32, i32, Direction) {
    for (row_idx, row) in map.iter().enumerate() {
        for (col_idx, &cell) in row.iter().enumerate() {
            match cell {
                '^' | '>' | 'v' | '<' => return (row_idx as i32, col_idx as i32, cell.into()),
                _ => continue,
            }
        }
    }
    panic!("No guard found in map")
}

fn note_visited_fields(map: &mut Vec<Vec<char>>) -> Vec<Vec<char>> {
    let (mut row_idx, mut col_idx, mut direction) = get_guard_state(map);
    let mut map_notes = map.clone();
    loop {
        map_notes[row_idx as usize][col_idx as usize] = 'X';
        let (row_move, col_move) = direction.next_move();
        let new_row_idx = (row_idx + row_move) as usize;
        let new_col_idx = (col_idx + col_move) as usize;

        if let Some(row) = map.get(new_row_idx) {
            if let Some(&cell) = row.get(new_col_idx) {
                if cell == '#' {
                    direction.turn();
                } else {
                    row_idx += row_move;
                    col_idx += col_move;
                }
                continue;
            }
        }
        break;
    }
    // for row in &map_notes {
    //     println!("{}", row.iter().collect::<String>());
    // }
    map_notes
}

fn sum_visited_fields(map: &Vec<Vec<char>>) -> u64 {
    let map_notes = note_visited_fields(&mut map.clone());
    map_notes
        .par_iter()
        .flatten()
        .filter(|&&c| c == 'X')
        .count() as u64
}

/// determine if the guard is in a loop by checking whether the guard
/// - leaves the map eventually
/// - returns to the same position with the same direction at any point
fn is_looping(map: &Vec<Vec<char>>) -> bool {
    let (mut row_idx, mut col_idx, mut direction) = get_guard_state(&map);
    let mut visited_positions_directions: HashSet<(i32, i32, Direction)> = HashSet::new();
    loop {
        if visited_positions_directions.contains(&(row_idx, col_idx, direction)) {
            return true;
        }
        visited_positions_directions.insert((row_idx, col_idx, direction));
        let (row_move, col_move) = direction.next_move();
        let new_row_idx = (row_idx + row_move) as usize;
        let new_col_idx = (col_idx + col_move) as usize;

        if let Some(row) = map.get(new_row_idx) {
            if let Some(&cell) = row.get(new_col_idx) {
                if cell == '#' {
                    direction.turn();
                } else {
                    row_idx += row_move;
                    col_idx += col_move;
                }
                continue;
            }
        }
        break;
    }
    false
}

fn get_number_of_looping_obstacle_locations(map: &Vec<Vec<char>>) -> u64 {
    let map_notes = note_visited_fields(&mut map.clone());
    let (guard_row_idx, guard_col_idx, _direction) = get_guard_state(&map);
    let obstacle_location_count = AtomicU64::new(0);
    map_notes.par_iter().enumerate().for_each(|(row_idx, row)| {
        row.par_iter().enumerate().for_each(|(col_idx, &c)| {
            if c == 'X' && !(row_idx == guard_row_idx as usize && col_idx == guard_col_idx as usize)
            {
                let mut map_permutation = map.clone();
                map_permutation[row_idx][col_idx] = '#';
                if is_looping(&map_permutation) {
                    obstacle_location_count.fetch_add(1, Ordering::Relaxed);
                }
            }
        })
    });
    obstacle_location_count.load(Ordering::Acquire)
}

pub fn day06() {
    print!(r"| ||| @@##'''...|        |...     .'  '.'''../..|  6: ");
    let file = read_to_string("src/data/day06.txt").expect("Unable to open file");
    let map: Vec<Vec<char>> = file.lines().map(|line| line.chars().collect()).collect();
    print!("Visited fields: {}, ", sum_visited_fields(&map));
    let number_of_obstacle_locations = get_number_of_looping_obstacle_locations(&map);
    println!(
        "# of possible obstacle locations: {}",
        number_of_obstacle_locations
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_map() -> Vec<Vec<char>> {
        vec![
            vec!['.', '.', '.', '.', '#', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '#', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '#', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '#', '.', '.', '^', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '#', '.'],
            vec!['#', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '#', '.', '.', '.'],
        ]
    }

    #[test]
    fn test_sum_visited_fields() {
        let map = get_map();
        assert_eq!(sum_visited_fields(&map), 41);
    }

    #[test]
    fn test_get_number_of_obstacle_locations() {
        let map = get_map();
        assert_eq!(get_number_of_looping_obstacle_locations(&map), 6);
    }
}
