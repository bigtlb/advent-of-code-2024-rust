#[cfg(test)]
mod test {
    use crate::util::read_file_to_string_array;
    use std::collections::{HashMap, HashSet};

    fn get_antennas(lines: &Vec<String>) -> HashMap<char, Vec<(i32, i32)>> {
        let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
        for (row, line) in lines.iter().enumerate() {
            for (col, c) in line.chars().enumerate() {
                if c != '.' {
                    antennas
                        .entry(c)
                        .or_insert(vec![])
                        .push((row as i32, col as i32));
                }
            }
        }
        antennas
    }

    fn get_antinodes(
        bounds: &(i32, i32),
        points: &Vec<(i32, i32)>,
        include_resonance: bool,
    ) -> Vec<(i32, i32)> {
        let mut antinodes: Vec<(i32, i32)> = vec![];
        for i in 0..points.len() {
            for j in i + 1..points.len() {
                let (mut x1, mut y1) = points[i];
                let (mut x2, mut y2) = points[j];

                let x_diff = x2 - x1;
                let y_diff = y2 - y1;

                loop {
                    let next_left = (x1 - x_diff, y1 - y_diff);
                    if next_left.0 < 0
                        || next_left.1 < 0
                        || next_left.0 >= bounds.0
                        || next_left.1 >= bounds.1
                    {
                        break;
                    }
                    antinodes.push(next_left);
                    x1 = next_left.0;
                    y1 = next_left.1;
                    if !include_resonance {
                        break;
                    }
                }

                loop {
                    let next_right = (x2 + x_diff, y2 + y_diff);
                    if next_right.0 < 0
                        || next_right.1 < 0
                        || next_right.0 >= bounds.0
                        || next_right.1 >= bounds.1
                    {
                        break;
                    }
                    antinodes.push(next_right);
                    x2 = next_right.0;
                    y2 = next_right.1;
                    if !include_resonance {
                        break;
                    }
                }
            }
        }
        antinodes
    }

    #[test]
    fn day8_part1() {
        let lines = read_file_to_string_array("src/day8_part1.data").unwrap();
        let antennas = get_antennas(&lines);
        let bounds = (lines.len() as i32, lines[0].len() as i32);
        let mut antinodeset: HashSet<(i32, i32)> = HashSet::new();

        for (_, points) in antennas.iter() {
            let antinodes: Vec<(i32, i32)> = get_antinodes(&bounds, &points, false)
                .iter()
                .cloned()
                .filter(|(x, y)| x >= &0 && y >= &0 && x < &bounds.0 && y < &bounds.1)
                .collect();
            antinodeset.extend(antinodes.iter().cloned());
        }
        println!("Antinode count: {}", antinodeset.len());
    }

    #[test]
    fn day8_part2() {
        // let lines = read_file_to_string_array("src/day8_test.data").unwrap();
        let lines = read_file_to_string_array("src/day8_part1.data").unwrap();
        let antennas = get_antennas(&lines);
        let bounds = (lines.len() as i32, lines[0].len() as i32);
        let mut antinodeset: HashSet<(i32, i32)> = HashSet::new();

        for (_, points) in antennas.iter() {
            let antinodes: Vec<(i32, i32)> = get_antinodes(&bounds, &points, true)
                .iter()
                .cloned()
                .filter(|(x, y)| x >= &0 && y >= &0 && x < &bounds.0 && y < &bounds.1)
                .collect();
            antinodeset.extend(antinodes.iter().cloned());
            antinodeset.extend(points.iter().cloned());
        }

        println!("Antinode count: {}", antinodeset.len());
    }
}
