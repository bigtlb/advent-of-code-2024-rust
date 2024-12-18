#[cfg(test)]
mod test {
    use crate::util::read_file_to_string_array;
    use std::collections::BinaryHeap;
    use std::cmp::Ordering;

    #[derive(Clone, Debug, PartialEq, Eq)]
    struct Point {
        x: usize,
        y: usize,
        score: usize,
        direction: char,
        path: Vec<(usize, usize, char)>,
    }
    impl Ord for Point {
        fn cmp(&self, other: &Self) -> Ordering {
            other.score.cmp(&self.score)
        }
    }

    impl PartialOrd for Point {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    fn find_start_and_end(map: &Vec<Vec<char>>) -> ((usize, usize), (usize, usize)) {
        let mut start = (0, 0);
        let mut end = (0, 0);

        for (y, row) in map.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell == 'S' {
                    start = (x, y);
                } else if cell == 'E' {
                    end = (x, y);
                }
            }
        }

        (start, end)
    }

    fn best_routes_and_score(
        map: &Vec<Vec<char>>,
        start: (usize, usize),
        start_direction: char,
        end: (usize, usize),
    ) -> (usize, Vec<Vec<(usize, usize, char)>>) {
        let directions = vec![
            (1, 0, '>'),  // right
            (0, 1, 'v'),  // down
            (-1, 0, '<'), // left
            (0, -1, '^'), // up
        ];
        let mut heap = BinaryHeap::new();
        // Need to keep track of location and direction when visited, and score at that point
        let mut visited = vec![vec![vec![usize::MAX; 4]; map[0].len()]; map.len()];
        let mut best_paths = Vec::new();
        let mut best_score = usize::MAX;

        let start_direction_index = directions.iter().position(|&(_, _, d)| d == start_direction).unwrap();
        heap.push(Point {
            x: start.0,
            y: start.1,
            score: 0,
            direction: start_direction,
            path: vec![(start.0, start.1, start_direction)],
        });
        visited[start.1][start.0][start_direction_index] = 0;

        while let Some(current) = heap.pop() {
            if (current.x, current.y) == end {
                if current.score < best_score {
                    best_score = current.score;
                    best_paths.clear();
                    best_paths.push(current.path.clone());
                } else if current.score == best_score {
                    best_paths.push(current.path.clone());
                }
                continue;
            }

            for (i, &(dx, dy, new_direction)) in directions.iter().enumerate() {
                let nx = current.x as isize + dx;
                let ny = current.y as isize + dy;

                if nx >= 0 && ny >= 0 {
                    let (nx, ny) = (nx as usize, ny as usize);

                    if map[ny][nx] != '#' {
                        let mut new_score = current.score + 1;
                        if current.direction != new_direction {
                            // If opposite direction continue for loop and skip this one
                            if (current.direction == '>' && new_direction == '<')
                                || (current.direction == '<' && new_direction == '>')
                                || (current.direction == '^' && new_direction == 'v')
                                || (current.direction == 'v' && new_direction == '^')
                            {
                                continue;
                            }
                            new_score += 1000;
                        }

                        if new_score <= visited[ny][nx][i] {
                            let mut new_path = current.path.clone();
                            new_path.push((nx, ny, new_direction));

                            heap.push(Point {
                                x: nx,
                                y: ny,
                                score: new_score,
                                direction: new_direction,
                                path: new_path,
                            });
                            visited[ny][nx][i] = new_score;
                        }
                    }
                }
            }
        }

        (best_score, best_paths)
    }

    fn print_map_and_paths(map: &Vec<Vec<char>>, paths: &Vec<Vec<(usize, usize, char)>>, use_os: bool) {
        let mut map = map.clone();
        for path in paths.iter() {
            for (x, y, direction) in path.iter().rev() {
                let curchar = map[*y][*x];
                if use_os || (curchar != 'S' && curchar != 'E') {
                    map[*y][*x] = if use_os { 'O' } else { *direction };
                }
            }
        }

        for row in map.iter() {
            println!("{}", row.iter().collect::<String>());
        }
    }

    #[test]
    fn day16_part1() {
        let map: Vec<Vec<char>> = read_file_to_string_array("src/day16_part1.data")
            .unwrap()
            .iter()
            .map(|x| x.chars().collect())
            .collect();

        println!("Initial State:");
        print_map_and_paths(&map, &vec![vec![]], false);
        let (start, end) = find_start_and_end(&map);
        let (score, paths) = best_routes_and_score(&map, start, '>', end);
        println!("\n\nEnd Score: {}, Possible paths: {}", score, paths.len());
        print_map_and_paths(&map, &vec![paths[0].clone()], false);
    }

    #[test]
    fn day16_part2() {
        let map: Vec<Vec<char>> = read_file_to_string_array("src/day16_part1.data")
            .unwrap()
            .iter()
            .map(|x| x.chars().collect())
            .collect();

        println!("Initial State:");
        print_map_and_paths(&map, &vec![vec![]], false);
        let (start, end) = find_start_and_end(&map);
        let (score, paths) = best_routes_and_score(&map, start, '>', end);
        println!("\n\nEnd Score: {}", score);
        let unique_path_locations = paths.iter().flatten().map(|(x, y, _)| (*x, *y)).collect::<std::collections::HashSet<(usize, usize)>>().len();
        println!("Possible paths: {}, Unique path positions: {}", paths.len(), unique_path_locations);
        print_map_and_paths(&map, &paths, true);
    }
}
