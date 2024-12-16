#[cfg(test)]
mod test {
    use crate::util::read_file_to_string_array;
    use std::collections::HashSet;

    fn find_area_perimeter_and_sides(
        grid: &Vec<Vec<char>>,
        visited: &mut HashSet<(usize, usize)>,
        x: usize,
        y: usize,
        plant_type: char,
    ) -> (usize, usize, usize) {
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut stack = vec![(x, y)];
        let mut area = 0;
        let mut perimeter = 0;
        let mut sides = Vec::new();

        //Breadth first algorith to find area and perimtere, just like you were flood fill painting
        while let Some((cx, cy)) = stack.pop() {
            if visited.contains(&(cx, cy)) {
                continue;
            }
            visited.insert((cx, cy));
            area += 1;

            for (i, &(dx, dy)) in directions.iter().enumerate() {
                let nx = cx as isize + dx;
                let ny = cy as isize + dy;
                if nx >= 0 && ny >= 0 {
                    let (nx, ny) = (nx as usize, ny as usize);
                    if nx < grid.len() && ny < grid[0].len() {
                        if grid[nx][ny] == plant_type && !visited.contains(&(nx, ny)) {
                            stack.push((nx, ny));
                        } else if grid[nx][ny] != plant_type {
                            perimeter += 1;
                            sides.push((cx, cy, i));
                        }
                    } else {
                        perimeter += 1;
                        sides.push((cx, cy, i));
                    }
                } else {
                    perimeter += 1;
                    sides.push((cx, cy, i));
                }
            }
        }

        // Now lets grab all the unique sides and make contiguious sides
        // Another breadth first search should do it
        let mut unique_sides = HashSet::new();
        for &(cx, cy, dir) in &sides {
            unique_sides.insert((cx, cy, dir));
        }

        let mut straight_sides = 0;
        let mut processed = HashSet::new();

        for &(cx, cy, dir) in &sides {
            if processed.contains(&(cx, cy, dir)) {
                continue;
            }

            let mut queue = vec![(cx, cy)];
            processed.insert((cx, cy, dir));

            while let Some((px, py)) = queue.pop() {
                for &(dx, dy) in &directions {
                    let nx = px as isize + dx;
                    let ny = py as isize + dy;
                    if nx >= 0 && ny >= 0 {
                        let (nx, ny) = (nx as usize, ny as usize);
                        if unique_sides.contains(&(nx, ny, dir)) && !processed.contains(&(nx, ny, dir)) {
                            queue.push((nx, ny));
                            processed.insert((nx, ny, dir));
                        }
                    }
                }
            }

            straight_sides += 1;
        }

        (area, perimeter, straight_sides)
    }

    #[test]
    fn day12_part1(){
        let plots:Vec<Vec<char>> = read_file_to_string_array("src/day12_part1.data").unwrap().iter().map(|x| x.chars().collect()).collect();
        let mut visited = HashSet::new();
        let mut results = Vec::new();

        for x in 0..plots.len() {
            for y in 0..plots[0].len() {
                if !visited.contains(&(x, y)) {
                    let plant_type = plots[x][y];
                    let (area, perimeter, sides) = find_area_perimeter_and_sides(&plots, &mut visited, x, y, plant_type);
                    results.push((area, perimeter, sides));
                }
            }
        }

        println!("The price of the {} plots is: {}", results.len(), results.iter().map(|(area, perimeter,_)| area * perimeter).sum::<usize>());
    }

    #[test]
    fn day12_part2(){
        let plots:Vec<Vec<char>> = read_file_to_string_array("src/day12_part1.data").unwrap().iter().map(|x| x.chars().collect()).collect();
        let mut visited = HashSet::new();
        let mut results = Vec::new();

        for x in 0..plots.len() {
            for y in 0..plots[0].len() {
                if !visited.contains(&(x, y)) {
                    let plant_type = plots[x][y];
                    let (area, perimeter, sides) = find_area_perimeter_and_sides(&plots, &mut visited, x, y, plant_type);
                    results.push((area, perimeter, sides));
                }
            }
        }

        println!("The price of the {} plots is: {}", results.len(), results.iter().map(|(area, _, sides)| area * sides).sum::<usize>());
    }
}