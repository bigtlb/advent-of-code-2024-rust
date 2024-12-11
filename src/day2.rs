use crate::util::read_file_to_string_array;

#[cfg(test)]
mod tests {
    use super::*;
    #[derive(PartialEq)]
    enum Direction {
        Up,
        Down,
    }

    fn is_safe(report: &[i32]) -> (bool, i32) {
        let mut report_direction: Option<Direction> = None;

        for i in 1..report.len() {
            let level = report[i];
            let previous_level = report[i - 1];
            let level_difference = level - previous_level;
            if level_difference > 3 || level_difference < -3 || level_difference == 0 {
                return (false, i as i32);
            }
            let current_direction = if level_difference > 0 {
                Direction::Up
            } else {
                Direction::Down
            };

            if report_direction.is_none() {
                report_direction = Some(current_direction);
            }
            if level_difference > 0 && report_direction == Some(Direction::Up) {
                continue;
            }
            if level_difference < 0 && report_direction == Some(Direction::Down) {
                continue;
            }
            return (false, i as i32);
        }
        (true, -1)
    }

    #[test]
    fn day12_part1() {
        let lines = read_file_to_string_array("src/day2_part1.data").unwrap();

        let mut reports: Vec<Vec<i32>> = Vec::new();
        for line in lines {
            let levels = line.split_whitespace().map(|s| s.parse::<i32>()).collect::<Result<Vec<i32>, _>>().unwrap();
            reports.push(levels);
        }

        let mut safe_reports: i32 = 0;
        for report in reports {
            let (safe, _) = is_safe(&report);
            if safe {
                safe_reports += 1;
            }
        }

        println!("The number of safe reports is: {}", safe_reports);
    }


    #[test]
    fn day2_part2() {
        let lines = read_file_to_string_array("src/day2_part1.data").unwrap();

        let mut reports: Vec<Vec<i32>> = Vec::new();
        for line in lines {
            let levels = line.split_whitespace().map(|s| s.parse::<i32>()).collect::<Result<Vec<i32>, _>>().unwrap();
            reports.push(levels);
        }

        let mut safe_reports: i32 = 0;
        for report in reports {
            let (safe, idx) = is_safe(&report);
            if safe {
                safe_reports += 1;
            } else {
                for ndx in 0..=idx {
                    let mut skip_report = report.clone();
                    skip_report.remove(ndx as usize);
                    let (safe, _) = is_safe(&skip_report);
                    if safe {
                        safe_reports += 1;
                        break;
                    }
                }
            }
        }

        println!("The number of safe reports is: {}", safe_reports);
    }
}
