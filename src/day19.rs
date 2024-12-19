#[cfg(test)]
mod test {
    use crate::util::read_file_to_string_array;
    use std::collections::HashMap;

    fn design_search(
        current_sequence: String,
        towel_patterns: &Vec<String>,
        design: &str,
        count: &mut usize,
        memo: &mut HashMap<String, usize>,
    ) {
        if let Some(&cached_count) = memo.get(&current_sequence) {
            *count += cached_count;
            return;
        }

        if current_sequence == design {
            *count += 1;
            memo.insert(current_sequence, 1);
            return;
        }

        let mut local_count = 0;
        for pattern in towel_patterns.iter() {
            let new_sequence = format!("{}{}", current_sequence, pattern);
            if design.starts_with(&new_sequence) {
                design_search(
                    new_sequence.clone(),
                    towel_patterns,
                    design,
                    &mut local_count,
                    memo,
                );
            }
        }
        memo.insert(current_sequence, local_count);
        *count += local_count;
    }

    fn count_all_possible_designs(towel_patterns: &Vec<String>, design: &str) -> usize {
        let mut count = 0;
        let mut memo: HashMap<String, usize> = HashMap::new();
        design_search(String::new(), towel_patterns, design, &mut count, &mut memo);
        count
    }

    fn load_data(path: &str) -> (Vec<String>, Vec<String>) {
        let mut data = read_file_to_string_array(path).unwrap();
        let avail_patterns: Vec<String> = data.remove(0).split(", ").map(String::from).collect(); // Need the map to change the slick into a String
        let mut designs: Vec<String> = Vec::new();
        for line in data.iter() {
            if line.is_empty() {
                continue;
            }
            designs.push(line.to_string());
        }
        (avail_patterns, designs)
    }
    #[test]
    fn day19_part1() {
        // let (avail_patterns, designs) = load_data("src/day19_test.data");
        let (avail_patterns, designs) = load_data("src/day19_part1.data");
        let mut possible = 0;

        for design in designs.iter() {
            if count_all_possible_designs(&avail_patterns, design) > 0 {
                possible += 1;
                println!("Design: {} is possible", design);
            } else {
                println!("Design: {} is not possible", design);
            }
        }

        println!("\n\n Total possible: {}", possible);
    }

    #[test]
    fn day19_part2() {
        // let (avail_patterns, designs) = load_data("src/day19_test.data");
        // let (avail_patterns, designs) = load_data("src/day19_test2.data");
        let (avail_patterns, designs) = load_data("src/day19_part1.data");
        let mut possible = 0;

        for design in designs.iter() {
            let count = count_all_possible_designs(&avail_patterns, design);
            if count > 0 {
                possible += count;
                println!("Design: {} has {} possible designs", design, count);
            } else {
                println!("Design: {} is not possible", design);
            }
        }

        println!("\n\n Total possible: {}", possible);
    }
}
