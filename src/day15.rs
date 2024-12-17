#[cfg(test)]
mod test {
    use crate::util::read_file_to_string_array;


    fn load_map(path: &str) -> (Vec<Vec<char>>, Vec<char>) {
        let lines = read_file_to_string_array(path).unwrap();
        let mut map: Vec<Vec<char>> = Vec::new();
        let mut directions: Vec<char> = Vec::new();
        let mut found_blank: bool = false;
        for line in lines {
            if line.is_empty() {
                found_blank = true;
                continue;
            }
            if !found_blank {
                map.push(line.chars().collect());
            } else {
                directions.extend(line.chars());
            }
        }
        (map, directions)
    }

    fn get_robot(map: &mut Vec<Vec<char>>) -> (usize, usize) {
        let mut robot: (usize, usize) = (0, 0);
        for (y, row) in map.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell == '@' {
                    robot = (x, y);
                    break;
                }
            }
        }
        map[robot.1][robot.0] = '.';
        robot
    }

    fn move_thing(mover: &(usize, usize), map: &mut Vec<Vec<char>>, direction: char) -> ((usize, usize), bool) {
        let (x, y) = *mover;
        let (dx, dy) = match direction {
            '^' => (0, -1),
            'v' => (0, 1),
            '<' => (-1, 0),
            '>' => (1, 0),
            _ => (0, 0),
        };
        let (nx, ny) = (x as isize + dx, y as isize + dy);
        if nx >= 0 && ny >= 0 {
            let (nx, ny) = (nx as usize, ny as usize);
            if map[ny][nx] == '.' {
                return ((nx, ny), true)
            } else {
                // Check and see if the box can move, if so update the mox position and this thing
                if map[ny][nx] == 'O' {
                    let (thing2, success) = move_thing(&(nx, ny), map, direction);
                    if success{
                        map[(ny as isize + dy) as usize][(nx as isize + dx) as usize] = map[ny][nx];
                        map[ny][nx] = '.';
                        return ((nx, ny), true)
                    }
                }
            }
        }

        (*mover, false)
    }

    // Two wide characters
    fn can_move_carton(carton: &(usize, usize), map: &mut Vec<Vec<char>>, direction: char, actually_move:bool) -> bool {
        let (x, y) = *carton;
        let (dx, dy) = match direction {
            '^' => (0, -1),
            'v' => (0, 1),
            '<' => (-1, 0),
            '>' => (2, 0),
            _ => (0, 0),
        };
        let (nx, ny) = (x as isize + dx, y as isize + dy);
        if nx >= 0 && ny >= 0 && nx < map[0].len() as isize && ny < map.len() as isize {
            let (nx, ny) = (nx as usize, ny as usize);
            let l = map[ny][nx];
            let r = map[ny][nx+1];
            if map[ny][nx] == '.' && ((direction == '<' || direction == '>') || map[ny][nx+1] == '.'){
                if actually_move {
                    let bx = if direction == '>' {nx-1} else {nx};
                    let c1 = map[y][x];
                    map[y][x] = '.';
                    let c2 = map[y][x+1];
                    map [y][x+1] = '.';
                    map[ny][bx] = c1;
                    map[ny][bx+1] = c2;
                }
                return true
            } else {
                if (direction == '^' || direction == 'v') && map[ny][nx] == ']' {
                    let can_move_left_box = can_move_carton(&(nx-1, ny), map, direction, actually_move);
                    if !can_move_left_box {
                        return false
                    }
                }
                if direction == '>' || map[ny][nx] == '[' {
                    let can_move_center_box = can_move_carton(&(nx, ny), map, direction, actually_move);
                    if !can_move_center_box {
                        return false
                    }
                }
                if direction == '<' || map[ny][nx] == ']' {
                    let can_move_right_box = can_move_carton(&(nx-1, ny), map, direction, actually_move);
                    if !can_move_right_box {
                        return false
                    }
                }
                if (direction == '^' || direction == 'v') && map[ny][nx+1] == '[' {
                    let can_move_right_box = can_move_carton(&(nx+1, ny), map, direction, actually_move);
                    if !can_move_right_box {
                        return false
                    }
                }
                if map[ny][nx] == '#' || map[ny][nx+1] == '#' {
                    return false
                }
                if actually_move {
                    let bx = if direction == '>' {nx-1} else {nx};
                    let c1 = map[y][x];
                    map[y][x] = '.';
                    let c2 = map[y][x+1];
                    map [y][x+1] = '.';
                    map[ny][bx] = c1;
                    map[ny][bx+1] = c2;
                }
                return true
            }
        }
        false
    }

    // Two wide characters
    fn move_robot2(robot: &(usize, usize), map: &mut Vec<Vec<char>>, direction: char) -> ((usize, usize), bool) {
        let (x, y) = *robot;
        let (dx, dy) = match direction {
            '^' => (0, -1),
            'v' => (0, 1),
            '<' => (-1, 0),
            '>' => (1, 0),
            _ => (0, 0),
        };
        let (nx, ny) = (x as isize + dx, y as isize + dy);
        if nx >= 0 && ny >= 0 {
            let (nx, ny) = (nx as usize, ny as usize);
            if map[ny][nx] == '.' {
                return ((nx, ny), true)
            } else {

                if map[ny][nx] == '[' || map[ny][nx] == ']' {
                    let bx = if map[ny][nx] == ']' {nx-1} else {nx};
                    if can_move_carton(&(bx, ny), map, direction, false) {
                        can_move_carton(&(bx, ny), map, direction, true);
                        return ((nx, ny), true)
                    }


                }
            }
        }

        (*robot, false)
    }

    fn calculate_gps_score(map: &mut Vec<Vec<char>>) -> usize {
        let score = map.iter().enumerate().map(|(y, row)| {
            row.iter().enumerate().map(|(x, cell)| {
                if *cell == 'O' || *cell == '[' {
                    100 * y + x
                } else {
                    0
                }
            }).sum::<usize>()
        }).sum::<usize>();
        score
    }

    fn print_state(map: &Vec<Vec<char>>, robot: &(usize, usize)) {
        print!("Robot: {:?}\n", robot);
        for (y, row) in map.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if x == robot.0 && y == robot.1 {
                    print!("@");
                } else {
                    print!("{}", cell);
                }
            }
            print!("\n");
        }
    }
    fn inflate_map(map: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let mut new_map = Vec::new();
        for (y, row) in map.iter().enumerate() {
            let mut new_row = Vec::new();
            for (x, cell) in row.iter().enumerate() {
                match cell {
                    '#' => {
                        new_row.push('#');
                        new_row.push('#');
                    }
                    '.' => {
                        new_row.push('.');
                        new_row.push('.');
                    }
                    'O' => {
                        new_row.push('[');
                        new_row.push(']');
                    }
                    _ => {
                        new_row.push(cell.clone());
                        new_row.push('.');
                    }
                }
            }
            new_map.push(new_row);
        }
        new_map
    }

    #[test]
    fn day15_part1() {
        let (mut map, directions) = load_map("src/day15_part1.data");
        let mut robot = get_robot(&mut map);
        let mut success: bool = false;
        print_state(&mut map, &mut robot);
        for direction in directions {
            (robot, _) = move_thing(&robot, &mut map, direction);
        }
        println!("\n\n");
        print_state(&map, &robot);
        let score = calculate_gps_score(&mut map);
        println!("Score: {:?}", score);
    }


    #[test]
    fn day15_part2() {
        let (mut map, directions) = load_map("src/day15_part1.data");
        map = inflate_map(map);
        let mut robot = get_robot(&mut map);
        let mut success: bool = false;
        print_state(&mut map, &mut robot);
        for direction in directions {
            (robot, success) = move_robot2(&robot, &mut map, direction);
        }
        println!("\n\n");
        print_state(&map, &robot);
        let score = calculate_gps_score(&mut map);
        println!("Score: {:?}", score);
    }
}