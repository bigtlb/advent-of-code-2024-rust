#[cfg(test)]
mod test {
    use std::cmp::Ordering;
    use crate::util::read_file_to_string_array;

    fn is_ordered(page_order: &[(i32, i32)], print_run: &[i32]) -> bool {
        let mut idx = 0;
        while idx < print_run.len() {
            let cur_page = print_run[idx];
            let before: Vec<i32> = page_order.iter().filter(|x| x.1 == cur_page).map(|x| x.0).collect();
            let after: Vec<i32> = page_order.iter().filter(|x| x.0 == cur_page).map(|x| x.1).collect();

            // If the befores end up after, or the afters end up before, then it's not ordered
            if before.iter().any(|x| print_run[idx + 1..].contains(x)) || after.iter().any(|x| print_run[0..idx].contains(x)) {
                return false;
            }
            idx += 1;
        }
        true
    }

    fn fix_order(page_order: &[(i32, i32)], print_run: &[i32]) -> Vec<i32> {

        let mut sorted_run = print_run.to_vec();
        sorted_run.sort_by(|a, b| {
            for (a_page, b_page) in page_order {
                if a == a_page && b == b_page {
                    return Ordering::Less;
                } else if a == b_page && b == a_page {
                    return Ordering::Greater;
                }
            }
            Ordering::Equal
        });
        sorted_run
    }

    fn load_data(path: &str) -> (Vec<(i32,i32)>, Vec<Vec<i32>>) {
        let lines = read_file_to_string_array(path).unwrap();

        let mut tuples: Vec<(i32, i32)> = Vec::new();
        let mut ldx = 0;
        while lines[ldx].trim() != "" {
            let parts: Vec<i32> = lines[ldx].split("|").map(|x| x.parse::<i32>().unwrap()).collect();
            let tuple = (parts[0], parts[1]);
            tuples.push(tuple);
            ldx += 1;
        }
        ldx += 1;

        let mut print_lists: Vec<Vec<i32>> = Vec::new();
        while ldx < lines.len() {
            print_lists.push(lines[ldx].split(",").map(|x| x.parse::<i32>().unwrap()).collect());
            ldx += 1;
        }

        (tuples, print_lists)
    }
    #[test]
    fn day5_part1() {
        let (page_order, print_runs) = load_data("src/day5_part1.data");

        let mut middle_sum = 0;
        for run in print_runs {
            if is_ordered(&page_order, &run) {
                middle_sum += run[(run.len()-1)/2];
            }
        }

        println!("The middle sum is: {}", middle_sum);
    }


    #[test]
    fn day5_part2() {
        let (page_order, print_runs) = load_data("src/day5_part1.data");

        let mut middle_sum = 0;
        for run in print_runs {
            if !is_ordered(&page_order, &run) {
                let fixed_run = fix_order(&page_order, &run);
                middle_sum += fixed_run[(fixed_run.len()-1)/2];
            }
        }

        println!("The middle sum is: {}", middle_sum);
    }
}