#[cfg(test)]
mod test {
    use crate::util::read_file_to_string_array;
    use std::collections::{HashSet, VecDeque};

    fn find_best_path_after_n_falling_bytes(
        falling_bytes: &Vec<(i32, i32)>,
        bounds: (i32, i32),
        start: (i32, i32),
        end: (i32, i32),
        n: i32,
    ) -> (Vec<Vec<char>>, Vec<(i32,i32)>){
        let directions = vec![(0,1), (0,-1), (1,0), (-1,0)];
        let mut board = vec![vec!['.'; (bounds.0+1) as usize]; (bounds.1+1) as usize];
        for i in 0..n {
            let (x, y) = falling_bytes[i as usize];
            board[y as usize][x as usize] = '#';
        }

        let mut queue:VecDeque<((i32,i32),Vec<(i32,i32)>)> = VecDeque::new();
        queue.push_back((start, vec![start]));
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        visited.insert(start);

        let mut best_path = Vec::new();
        while !queue.is_empty() {
            let (current, path) = queue.pop_front().unwrap();
            if current == end {
                best_path = path;
                break;
            }

            for direction in directions.iter() {
                let new_x = current.0 + direction.0;
                let new_y = current.1 + direction.1;
                let new_pos = (new_x, new_y);
                if new_x >= 0 && new_x < bounds.0+1 && new_y >= 0 && new_y < bounds.1+1 && board[new_y as usize][new_x as usize] == '.' && !visited.contains(&new_pos) {
                    let mut new_path = path.clone();
                    new_path.push(new_pos);
                    queue.push_back((new_pos, new_path));
                    visited.insert(new_pos);
                }
            }
        }

        (board, best_path)
    }

    fn print_board(board: &Vec<Vec<char>>, path: &Vec<(i32,i32)>, include_path: bool) {
        for (y, row) in board.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if include_path && path.contains(&(x as i32, y as i32)) {
                    print!("O");
                } else {
                    print!("{}", cell);
                }
            }
            println!();
        }
    }

    #[test]
    fn day18_part1() {
        // let falling_bytes: Vec<(i32, i32)> = read_file_to_string_array("src/day18_test.data")
            let falling_bytes: Vec<(i32, i32)> = read_file_to_string_array("src/day18_part1.data")
            .unwrap()
            .iter()
            .map(|line| {
                let parts: Vec<&str> = line.split(",").collect();
                (
                    parts[0].parse::<i32>().unwrap(),
                    parts[1].parse::<i32>().unwrap(),
                )
            })
            .collect(); // Origin is top left

        // let bounds = (6,6); // (0..=6) in both directions
        // let depth = 12;
        let bounds = (70,70); // (0..=70) in both directions
        let depth = 1024;

        let (board, best_path) = find_best_path_after_n_falling_bytes(&falling_bytes, bounds, (0,0), bounds, depth);
        // We don't count the starting position as a step
        println!("\n\nBest path is length: {}", best_path.len()-1);
        print_board(&board, &best_path, true);
    }


    #[test]
    fn day18_part2() {
        // let falling_bytes: Vec<(i32, i32)> = read_file_to_string_array("src/day18_test.data")
            let falling_bytes: Vec<(i32, i32)> = read_file_to_string_array("src/day18_part1.data")
            .unwrap()
            .iter()
            .map(|line| {
                let parts: Vec<&str> = line.split(",").collect();
                (
                    parts[0].parse::<i32>().unwrap(),
                    parts[1].parse::<i32>().unwrap(),
                )
            })
            .collect(); // Origin is top left

        // let bounds = (6,6); // (0..=6) in both directions
        // let mut depth = 12;
        let bounds = (70,70); // (0..=70) in both directions
        let mut depth = 1024;

        #[allow(unused_assignments)]
        let mut board:Vec<Vec<char>> = Vec::new();
        #[allow(unused_assignments)]
        let mut best_path:Vec<(i32,i32)> = Vec::new();

        loop {
            (board, best_path) = find_best_path_after_n_falling_bytes(&falling_bytes, bounds, (0,0), bounds, depth);
            if best_path.len() == 0 {
                depth -=1;
                break;
            }
            depth += 1;
        }

        // We don't count the starting position as a step
        println!("\n\nThe byte that fell and blocked everything was at index: {} {:?}", depth, falling_bytes[depth as usize]);
        print_board(&board, &best_path, true);
    }
}
