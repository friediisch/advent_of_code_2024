use std::fs::read_to_string;

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

fn predict_visited_fields(map: &Vec<Vec<char>>) -> u64 {
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
    map_notes.iter().flatten().filter(|&&c| c == 'X').count() as u64
}

pub fn day06() {
    print!(r"| ||| @@##'''...|        |...     .'  '.'''../..|  6: ");
    let file = read_to_string("src/data/day06.txt").expect("Unable to open file");
    let map: Vec<Vec<char>> = file.lines().map(|line| line.chars().collect()).collect();
    println!("Visited fields: {}", predict_visited_fields(&map));
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
    fn test_predict_visited_fields() {
        let map = get_map();
        assert_eq!(predict_visited_fields(&map), 41);
    }
}
