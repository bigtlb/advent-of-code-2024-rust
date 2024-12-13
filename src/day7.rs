#[cfg(test)]
mod test {
    use crate::util::read_file_to_string_array;

    fn load_data(path: &str) -> Vec<Vec<i64>> {
        read_file_to_string_array(path).unwrap().iter()
            .map(|line| line.as_str().split(|c| c == ' ' || c == ':')
                .filter_map(|num| num.trim().parse::<i64>().ok())
                .collect())
            .collect()
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    enum Operator {
        Add,
        Multiply,
        Concatenate,
    }

    fn is_calibrated(test_value: &i64, numbers: &[i64], operators: &Vec<Operator>, use_concatenate: bool) -> bool {
        if operators.len() == numbers.len()-1 {
            let mut total = numbers[0];
            for i in 0..operators.len() {
                match operators[i] {
                    Operator::Add => total += numbers[i+1],
                    Operator::Multiply => total *= numbers[i+1],
                    Operator::Concatenate => total = format!("{}{}", total, numbers[i+1]).parse::<i64>().unwrap(),
                }
            }
            return total == *test_value
        } else {
            let mut add_operators = operators.clone();
            add_operators.push(Operator::Add);
            let mut multiply_operators = operators.clone();
            multiply_operators.push(Operator::Multiply);
            let mut concatenate_operators = operators.clone();
            concatenate_operators.push(Operator::Concatenate);

            is_calibrated(test_value, numbers, &add_operators, use_concatenate)
                || is_calibrated(test_value, numbers, &multiply_operators, use_concatenate)
                || (use_concatenate && is_calibrated(test_value, numbers, &concatenate_operators, true))
        }
    }

    #[test]
    fn day7_part1() {
        let numbers: Vec<Vec<i64>> = load_data("src/day7_part1.data");
        //let numbers: Vec<Vec<i64>> = load_data("src/day7_test.data");
        let mut total: i64 = 0;
        for number in numbers {
            if is_calibrated(&number[0], &number[1..], &vec![], false) {
                total += number[0];
            }
        }
        println!("The total is: {}", total);
    }

    #[test]
    fn day7_part2() {
        let numbers: Vec<Vec<i64>> = load_data("src/day7_part1.data");
        //let numbers: Vec<Vec<i64>> = load_data("src/day7_test.data");
        let mut total: i64 = 0;
        for number in numbers {
            if is_calibrated(&number[0], &number[1..], &vec![], true) {
                total += number[0];
            }
        }
        println!("The total is: {}", total);
    }
}