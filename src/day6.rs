#[cfg(test)]
mod test {
    use crate::util::read_file_to_string_array;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum Direction {
        Up,
        Down,
        Left,
        Right
    }

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    struct GuardState {
        position: (i32, i32),  // Row, Col
        direction: Direction
    }

    fn get_guard_state(vec: &mut Vec<String>) -> GuardState {
        let mut guard_state = GuardState { position: (0, 0), direction: Direction::Up };
        for (idx, line) in vec.iter().enumerate() {
            let mut new_line = line.clone();
            if let Some(pos) = line.find(|c: char| c == '^' || c == '>' || c == 'v' || c == '<') {
                let guard = line.chars().nth(pos).unwrap();
                guard_state.position = (idx as i32, pos as i32);
                match guard {
                    '^' => guard_state.direction = Direction::Up,
                    '>' => guard_state.direction = Direction::Right,
                    'v' => guard_state.direction = Direction::Down,
                    '<' => guard_state.direction = Direction::Left,
                    _ => panic!("Unknown guard character")
                }
                new_line = new_line.replace(guard, "X");
                vec[idx] = new_line;
                break;
            }
        }
        guard_state
    }

    fn debug_dump(lines: &[String], guard_state: &GuardState) {
        println!("{:?}", guard_state);
        for line in lines {
            println!("{}", line);
        }
    }

    fn count_path(lines: &[String]) -> i32 {
        let mut count = 0;
        for line in lines {
            count += line.chars().filter(|c| *c == 'X').count() as i32;
        }
        count
    }

    fn move_next(lines: &mut Vec<String>, guard_state: &GuardState, mark: bool) -> Option<GuardState> {
        let (row, col) = guard_state.position;
        let (mut new_row, mut new_col) = match guard_state.direction {
            Direction::Up => (row - 1, col),
            Direction::Down => (row + 1, col),
            Direction::Left => (row, col - 1),
            Direction::Right => (row, col + 1)
        };
        if new_row < 0 || new_row >= lines.len() as i32 || new_col < 0 || new_col >= lines[0].len() as i32 {
            return None;
        }
        let next_char = lines[new_row as usize].chars().nth(new_col as usize).unwrap();
        let new_direction = match next_char {
            '#' | 'O' => {
                new_row = row;
                new_col = col;
                match guard_state.direction {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down
                }
            },
            '.' | 'X' => {
                if mark {
                    lines[new_row as usize].replace_range(new_col as usize..new_col as usize + 1, "X");
                }
                guard_state.direction
            },
            _ => panic!("Unknown character")
        };
        Some(GuardState { position: (new_row, new_col), direction: new_direction })
    }

    // Find every row and column in the lines that has 'X', (except for the guard_state position), and return a Vec of tuples
    fn enumerate_path_positions(lines: &[String], guard_state: &GuardState) -> Vec<(i32, i32)> {
        let mut path = Vec::new();
        for (row, line) in lines.iter().enumerate() {
            for (col, c) in line.chars().enumerate() {
                if c == 'X' && (row as i32, col as i32) != guard_state.position {
                    path.push((row as i32, col as i32));
                }
            }
        }
        path
    }

    fn causes_loop(lines: &[String], guard_state: &GuardState, row: i32, col: i32) -> bool {
        let mut guard_states = std::collections::HashSet::new();
        let mut test_lines = lines.to_vec();
        test_lines[row as usize].replace_range(col as usize..col as usize + 1, "O");

        let mut test_guard_state = guard_state.clone();
        loop {
            match move_next(&mut test_lines, &test_guard_state, true) {
                Some(new_guard_state) => {
                    if !guard_states.insert(new_guard_state.clone()) {
                        // debug_dump(&test_lines, &new_guard_state);
                        // println!("BLocker at ({},{})\n\n", row, col);
                        return true;
                    }
                    test_guard_state = new_guard_state;
                },
                None => break
            }
        }
        false
    }

    #[test]
    fn day6_part1() {
        let mut lines = read_file_to_string_array("src/day6_part1.data").unwrap();
        let mut guard_state = get_guard_state(&mut lines);
        loop {
            match move_next(&mut lines, &guard_state, true){
                Some(new_guard_state) => guard_state = new_guard_state,
                None => break
            }
        }
        println!("\n\nDone!");
        debug_dump(&lines, &guard_state);
        println!("Total guard path: {}", count_path(&lines));

    }


    #[test]
    fn day6_part2() {
        let mut lines = read_file_to_string_array("src/day6_part1.data").unwrap();
        let guard_state = get_guard_state(&mut lines);
        let mut original_path = lines.clone();
        let mut test_guard_state = guard_state;
        loop {
            match move_next(&mut original_path, &test_guard_state, true){
                Some(new_guard_state) => test_guard_state = new_guard_state,
                None => break
            }
        }
        let path = enumerate_path_positions(&original_path, &guard_state);
        let possible_obstacles: usize =  path.iter().filter(|(row, col)| causes_loop(&lines, &guard_state, *row, *col)).count();
        println!("Possible obstacles: {}", possible_obstacles);
    }
}