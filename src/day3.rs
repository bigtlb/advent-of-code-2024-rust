#[cfg(test)]
mod tests {
    use std::sync::LazyLock;
    use crate::util::read_file_to_string_array;
    use regex::Regex;

    static REGMATCH_MULT: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap());
    static REGMATCH_MULT_DO_DONT: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\))").unwrap());

    #[test]
    fn day3_part1() {
        let lines = read_file_to_string_array("src/day3_part1.data").unwrap();

        let mut total: i64 = 0;
        for line in lines {
            for cap in REGMATCH_MULT.captures_iter(&line) {
                let first: i64 = cap[1].parse().unwrap();
                let second: i64 = cap[2].parse().unwrap();
                total += first * second;
            }
        }

        println!("The total is: {}", total);
    }

    #[test]
    fn day3_part2() {
        let lines = read_file_to_string_array("src/day3_part1.data").unwrap();

        let mut total: i64 = 0;
        let mut donext: bool = true;
        for line in lines {
            for cap in REGMATCH_MULT_DO_DONT.captures_iter(&line) {
                if cap[0] == *"do()" {
                    donext = true
                } else if cap[0] == *"don't()" {
                    donext = false
                } else {
                    if donext {
                        let first: i64 = cap[2].parse().unwrap();
                        let second: i64 = cap[3].parse().unwrap();
                        total += first * second;
                    }
                }
            }
        }
        println!("The total is: {}", total);
    }
}
