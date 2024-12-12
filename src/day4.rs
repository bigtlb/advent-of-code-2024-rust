#[cfg(test)]
mod test {
    use strum::IntoEnumIterator;
    use strum_macros::EnumIter;
    use crate::util::read_file_to_string_array;

    #[derive(EnumIter, Debug)]
    enum Directions { Up, Down, Left, Right, UpRight, UpLeft, DownRight, DownLeft }
    const XMAS: &[char] = &['X', 'M', 'A', 'S'];

    impl Directions {
        fn get_next_coord(&self, lines: &[Vec<char>], row: i32, col: i32) -> Result<(i32, i32),&str> {
            let value = match self {
                Directions::Up => (row - 1, col),
                Directions::Down => (row + 1, col),
                Directions::Left => (row, col - 1),
                Directions::Right => (row, col + 1),
                Directions::UpRight => (row - 1, col + 1),
                Directions::UpLeft => (row - 1, col - 1),
                Directions::DownRight => (row + 1, col + 1),
                Directions::DownLeft => (row + 1, col - 1),
            };

            if value.0 < 0 || value.0 >= lines.len() as i32 || value.1 < 0 || value.1 >= lines[value.0 as usize].len() as i32 {
                Err("Out of bounds")
            } else {
                Ok(value)
            }
        }
    }

    fn xmas_count(lines: &[Vec<char>], ldx: i32, cdx: i32) -> i32 {
        if lines[ldx as usize][cdx as usize] != XMAS[0] {
            0
        } else {
            let mut cur_count: i32 = 0;

            for dir in Directions::iter() {
                let mut matching = true;
                let mut cur_row = ldx;
                let mut cur_col = cdx;
                for i in 1..XMAS.len() {
                    match dir.get_next_coord(lines, cur_row, cur_col) {
                        Ok((row, col)) => {
                            if lines[row as usize][col as usize] != XMAS[i] {
                                matching = false;
                                break;
                            }
                            cur_row = row;
                            cur_col = col;
                        }
                        Err(_) => {
                            matching = false;
                            break;
                        }
                    }
                }
                if matching {
                    cur_count += 1;
                }
            }

            cur_count
        }
    }

    fn is_cross_mas(lines: &[Vec<char>], ldx: i32, cdx: i32) -> bool {
        if lines[ldx as usize][cdx as usize] != 'A' ||
            ldx + 2 > lines.len() as i32 ||
            cdx + 2 > lines[ldx as usize].len() as i32 ||
            ldx - 1 < 0 ||
            cdx - 1 < 0 {
            return false;
        }

        let mut criss: String = String::new();
        let mut cross: String = String::new();
        for i in -1..=1 {
            criss.push(lines[(ldx + i) as usize][(cdx + i) as usize]);
            cross.push(lines[(ldx + i) as usize][(cdx - i) as usize]);
        }

        (criss == "MAS" || criss == "SAM") && (cross == "MAS" || cross == "SAM")

    }

        #[test]
    fn day4_part1() {
        //let lines: Vec<Vec<char>> = read_file_to_string_array("src/day4_test.data")
        let lines: Vec<Vec<char>> = read_file_to_string_array("src/day4_part1.data")
            .unwrap()
            .iter()
            .map(|line| line.chars().collect())
            .collect();

        let mut number_xmases:i32 = 0;
        for ldx in 0..lines.len() {
            for cdx in 0..lines[ldx].len() {
                number_xmases += xmas_count(&lines, ldx as i32, cdx as i32);
            }
        }

        println!("The number of xmases is: {}", number_xmases);
    }

    #[test]
    fn day4_part2() {
        let lines: Vec<Vec<char>> = read_file_to_string_array("src/day4_part1.data")
            .unwrap()
            .iter()
            .map(|line| line.chars().collect())
            .collect();

        let mut number_xmases:i32 = 0;
        for ldx in 0..lines.len() {
            for cdx in 0..lines[ldx].len() {
                if is_cross_mas(&lines, ldx as i32, cdx as i32) {
                    number_xmases += 1;
                }
            }
        }

        println!("The number of cross MAS es is: {}", number_xmases);
    }
}