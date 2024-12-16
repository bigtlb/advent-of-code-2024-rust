#[cfg(test)]
mod test {
    use crate::util::read_file_to_string_array;
    use std::collections::HashMap;

    fn blink(stones: &Vec<i64>) -> Vec<i64> {
        let mut result: Vec<i64> = Vec::new();
        for i in 0..stones.len() {
            match stones[i] {
                0 => result.push(1),
                n if n.to_string().len() % 2 == 0 => {
                    let digits = n.to_string();
                    let first_half = digits[..digits.len() / 2].parse::<i64>().unwrap();
                    let second_half = digits[digits.len() / 2..].parse::<i64>().unwrap();
                    result.push(first_half);
                    result.push(second_half);
                }
                n => result.push(n * 2024),
            }
        }
        result
    }

    fn blinks_per_stone_results(
        stone: i64,
        blinks: i64,
        memo: &mut HashMap<(i64, i64), i64>,
    ) -> i64 {
        if blinks == 0 {
            return 1;
        }

        if let Some(&result) = memo.get(&(stone, blinks)) {
            return result;
        }

        let result = match stone {
            0 => blinks_per_stone_results(1, blinks - 1, memo),
            n if n.to_string().len() % 2 == 0 => {
                let digits = n.to_string();
                let first_half = digits[..digits.len() / 2].parse::<i64>().unwrap();
                let second_half = digits[digits.len() / 2..].parse::<i64>().unwrap();
                blinks_per_stone_results(first_half, blinks - 1, memo) + blinks_per_stone_results(second_half, blinks - 1, memo)
            },
            n => blinks_per_stone_results(n * 2024, blinks - 1, memo),
        };

        memo.insert((stone, blinks), result);
        result
    }


    #[test]
    fn day11_part1() {
        let stones: Vec<i64> = read_file_to_string_array("src/day11_part1.data").unwrap()[0]
            .split(" ")
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        let mut result: Vec<i64> = stones;
        for _ in 0..25 {
            result = blink(&result);
        }
        println!("The result is: {}", result.len());
    }

    #[test]
    fn day11_part2() {

        let stones: Vec<i64> = read_file_to_string_array("src/day11_part1.data").unwrap()[0]
            .split(" ")
            .map(|x| x.parse::<i64>().unwrap())
            .collect();

        let mut result: i64 = 0;
        let mut memo: HashMap<(i64, i64), i64> = HashMap::new();
        for &stone in stones.iter() {
            result += blinks_per_stone_results(stone, 75, &mut memo);
        }
        println!("The result is: {:?}", result);

    }
}
