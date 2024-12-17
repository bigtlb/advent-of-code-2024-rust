#[cfg(test)]
mod test {
    use crate::util::read_file_to_string_array;

    #[derive(Debug, Clone)]
    struct Robot {
        pos: (isize, isize),
        velocity: (isize, isize),
    }

    fn robots_after_seconds(robots: &Vec<Robot>, bounds: &(isize,isize), seconds: isize) -> Vec<Robot> {
        let mut newpositions = robots.clone();
        for robot in &mut newpositions {
            robot.pos.0 = (robot.pos.0 + robot.velocity.0 * seconds) % bounds.0;
            robot.pos.1 = (robot.pos.1 + robot.velocity.1 * seconds) % bounds.1;
            if robot.pos.0 < 0 {
                robot.pos.0 += bounds.0;
            }
            if robot.pos.1 < 0 {
                robot.pos.1 += bounds.1;
            }

        }
        newpositions
    }

    fn safety_score(robots: &Vec<Robot>, bounds: &(isize,isize)) -> usize {
        let mut sectors:Vec<usize> = vec![0,0,0,0];
        let x_sector_size = bounds.0 / 2;
        let y_sector_size = bounds.1 / 2;
        for robot in robots {
            // Don't count the midlines
            if robot.pos.0 == x_sector_size || robot.pos.1 == y_sector_size {
                continue;
            }
            let x_sector = if robot.pos.0 < x_sector_size {0} else {1};
            let y_sector = if robot.pos.1 < y_sector_size {0} else {1};
            sectors[x_sector + y_sector * 2] += 1;
        }
        sectors.iter().fold(1, |acc, x| acc * x)
    }

    fn load_robots(path: &str) -> Vec<Robot> {
        let robots: Vec<Robot> = read_file_to_string_array(path).unwrap()
            .iter()
            .map(|line| {
                let parts: Vec<&str> = line.split(|c| c == '=' || c == ' ' || c == ',').collect();
                let pos: (isize, isize) = (parts[1].parse().unwrap(), parts[2].parse().unwrap());
                let velocity: (isize, isize) = (parts[4].parse().unwrap(), parts[5].parse().unwrap());
                Robot { pos, velocity }
            })
            .collect();
        robots
    }

    fn print_grid(robots: &Vec<Robot>, bounds: &(isize,isize)) {
        let mut grid = vec![vec![0; bounds.1 as usize]; bounds.0 as usize];
        for robot in robots {
            grid[robot.pos.0 as usize][robot.pos.1 as usize] += 1;
        }
        for y in 0..bounds.1 {
            for x in 0..bounds.0 {
                print!("{}", if grid[x as usize][y as usize] == 0 {'.'} else {grid[x as usize][y as usize].to_string().chars().next().unwrap()});
            }
            println!();
        }
    }

    #[test]
    fn day14_part1() {
        let robots = load_robots("src/day14_part1.data");
        // let bounds:(isize,isize) = (11,7);
        let bounds:(isize,isize) = (101,103);
        print_grid(&robots, &bounds);

        let newpositions = robots_after_seconds(&robots, &bounds, 100);
        println!("\n\n");
        print_grid(&newpositions, &bounds);
        let score = safety_score(&newpositions, &bounds);
        println!("\n\nscore: {}", score);
    }

    // Check to see if all points in grid, within the triangle declared by the three points in the vector are non-zero
    fn is_filled(grid: &Vec<Vec<usize>>, point: (isize,isize), left: isize, right: isize, ytest: isize) -> bool {

        for y in point.1..ytest {
            for x in left..=right {
                if grid[y as usize][x as usize] == 0 {
                    return false;
                }
            }
        }
        true
    }
    fn triangle_exists(robots: &Vec<Robot>, bounds: &(isize,isize)) -> bool {
        let mut grid = vec![vec![0; bounds.0 as usize]; bounds.1 as usize];
        for robot in robots {
            grid[robot.pos.1 as usize][robot.pos.0 as usize] += 1;
        }
        for y in 0..bounds.1 {
            for x in 0..bounds.0 {
                if grid[y as usize][x as usize] == 0 {
                    continue;
                }
                let mut left = x;
                let mut right = x;
                let mut ytest = y;
                while left >= 0 && ytest < bounds.1 && grid[ytest as usize][left as usize] > 0 {
                    left -= 1;
                    ytest += 1;
                }
                ytest = y;
                while right < bounds.0 && ytest < bounds.1 && grid[ytest as usize][right as usize] > 0 {
                    right += 1;
                    ytest += 1;
                }
                if x-left == right-x && ytest-y > 3 && is_filled(&grid, (x,y), left+1, right-1, ytest-1) {
                    println!("Triangle found at apex {:?}, left {:?}, right {:?}", (x,y), (left+1,ytest-1), (right-1,ytest-1));

                    print_grid(&robots, &bounds);
                    return true;
                }
            }
        }
        false
    }
    #[test]
    fn day14_part2() {
        let robots = load_robots("src/day14_part1.data");
        let bounds:(isize,isize) = (101,103);
        let mut seconds:isize = 1;
        loop {
            let newpositions = robots_after_seconds(&robots, &bounds, seconds);
            seconds += 1;
            if triangle_exists(&newpositions, &bounds) {
                println!("\n\nSeconds: {}", seconds-1);
                break;
            }
        }
    }
}