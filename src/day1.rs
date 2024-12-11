use crate::util::read_file_to_string_array;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1_part1() {


        let lines = read_file_to_string_array("src/day1_part1.data").unwrap();

        let mut array1: Vec<i64> = Vec::new();
        let mut array2: Vec<i64> = Vec::new();
        for line in lines {
            let split_line: Vec<&str> = line.split_whitespace().collect();
            array1.push(split_line[0].parse::<i64>().unwrap());
            array2.push(split_line[1].parse::<i64>().unwrap());
        }

        array1.sort();
        array2.sort();

        let mut sum = 0;
        for i in 0..array1.len() {
            sum += (array1[i] - array2[i]).abs();
        }

        println!("The sum of all the differences is: {}", sum);

    }

    #[test]
    fn day1_part2() {

        let lines = read_file_to_string_array("src/day1_part1.data").unwrap();

        let mut array1: Vec<i64> = Vec::new();
        let mut array2: Vec<i64> = Vec::new();
        for line in lines {
            let split_line: Vec<&str> = line.split_whitespace().collect();
            array1.push(split_line[0].parse::<i64>().unwrap());
            array2.push(split_line[1].parse::<i64>().unwrap());
        }

        let mut dict: std::collections::HashMap<i64, i64> = std::collections::HashMap::new();
        for i in 0..array1.len() {
            let key = array1[i];
            if !dict.contains_key(&key) {
                let value = array2.iter().filter(|&n| *n == key).count() as i64;
                dict.insert(key, value);
            }
        }

        let mut similarity_score = 0;
        for &key in &array1 {
            if let Some(&count) = dict.get(&key) {
                similarity_score += key * count;
            }
        }

        println!("The similarity score is: {}", similarity_score);

    }
}