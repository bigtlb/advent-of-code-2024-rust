#[cfg(test)]
mod test {
    use crate::util::read_file_to_string_array;
    use std::collections::{HashSet, VecDeque};

    fn can_reach_summit(grid: &Vec<Vec<u8>>, start: &(i32, i32), summit: &(i32, i32)) -> bool {
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut queue = VecDeque::new();
        let mut visited:HashSet<(i32, i32)> = HashSet::new();

        queue.push_back(start.clone());
        visited.insert(start.clone());

        while let Some((x, y)) = queue.pop_front() {
            let bounds = (grid.len() as i32, grid[0].len() as i32);
            let curpoint = grid[x as usize][y as usize];
            for &(dx, dy) in &directions {
                let nx = x  + dx;
                let ny = y  + dy;

                if nx >= 0 && ny >= 0 {
                    let nx = nx;
                    let ny = ny ;

                    if nx < bounds.0 && ny < bounds.1 {
                        let nextpoint = grid[nx as usize][ny as usize];
                        if nextpoint == curpoint + 1 && !visited.contains(&(nx, ny)) {
                            if nx == summit.0 && ny == summit.1 {
                                return true;
                            }
                            queue.push_back((nx, ny));
                            visited.insert((nx, ny));
                        }
                    }
                }
            }
        }

        false
    }

    fn find_trail_heads_and_summits(grid: &Vec<Vec<u8>>) -> (Vec<(i32, i32)>, Vec<(i32, i32)>) {
        let mut trail_heads = Vec::new();
        let mut summits = Vec::new();

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 0 {
                    trail_heads.push((i as i32, j as i32));
                } else if grid[i][j] == 9 {
                    summits.push((i as i32, j as i32));
                }
            }
        }

        (trail_heads, summits)
    }

    fn count_paths(grid: &Vec<Vec<u8>>, start: &(i32, i32), summits: &Vec<(i32, i32)>, visited: &mut HashSet<(i32, i32)>, path_count: &mut i64) {
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let bounds = (grid.len() as i32, grid[0].len() as i32);
        let curpoint = grid[start.0 as usize][start.1 as usize];
        for (dx, dy) in directions {
            let nx = start.0 + dx;
            let ny = start.1 + dy;

            if nx >= 0 && ny >= 0 {
                let nx = nx;
                let ny = ny;

                if nx < bounds.0 && ny < bounds.1 {
                    let nextpoint = grid[nx as usize][ny as usize];
                    if nextpoint == curpoint + 1 && !visited.contains(&(nx, ny)) {
                        if summits.contains(&(nx, ny)) {
                            *path_count += 1;
                        } else {
                            visited.insert((nx, ny));
                            count_paths(grid, &(nx, ny), summits, visited, path_count);
                            visited.remove(&(nx, ny));
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn day10_part1() {
        // let lines = read_file_to_string_array("src/day10_test2.data").unwrap();
        let lines = read_file_to_string_array("src/day10_part1.data").unwrap();
        let grid: Vec<Vec<u8>> = lines
            .iter()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect()
            })
            .collect();
        let (trail_heads, summits) = find_trail_heads_and_summits(&grid);
        let mut score:i64 = 0;
        for trail in &trail_heads {
            for summit in &summits {
                if can_reach_summit(&grid, trail, summit) {
                    // println!("Trail: {:?} Summit: {:?}", trail, summit);
                    score +=1;
                }
            }
        }
        println!("The score is: {}", score);
    }

    #[test]
    fn day10_part2() {
        // let lines = read_file_to_string_array("src/day10_test2.data").unwrap();
        let lines = read_file_to_string_array("src/day10_part1.data").unwrap();
        let grid: Vec<Vec<u8>> = lines
            .iter()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect()
            })
            .collect();
        let (trail_heads, summits) = find_trail_heads_and_summits(&grid);
        let mut score:i64 = 0;
        for trail in &trail_heads {
            let mut path_count:i64 = 0;
            count_paths(&grid, trail, &summits, &mut HashSet::new(), &mut path_count);
            score += path_count;
        }
        println!("The score is: {}", score);
    }
}
