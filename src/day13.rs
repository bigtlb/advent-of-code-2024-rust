#[cfg(test)]
mod test {
    use std::collections::HashSet;
    use crate::util::read_file_to_string_array;


    #[derive(Eq, PartialEq, Debug, Clone)]
    struct State {
        cost: usize,
        x: isize,
        y: isize,
    }

    impl Ord for State {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            other.cost.cmp(&self.cost)
        }
    }

    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    #[derive(Debug, Clone)]
    struct Game {
        button_a: State,
        button_b: State,
        prize: (isize, isize),
    }


    fn find_combinations(target: isize, button_a: isize, button_b: isize) -> Vec<(usize, usize)> {
        let mut combinations = Vec::new();
        let max_a_presses = target / button_a;

        for a_presses in 1..=max_a_presses {
            if (target - a_presses * button_a) % button_b == 0 {
                let b_presses = (target - a_presses * button_a) / button_b;
                combinations.push((a_presses as usize, b_presses as usize));
            }
        }
        combinations
    }

    fn minimum_cost_to_win(game: Game) -> Option<usize> {
        let x_combinations = find_combinations(game.prize.0, game.button_a.x, game.button_b.x);
        let y_combinations = find_combinations(game.prize.1, game.button_a.y, game.button_b.y);

        let mut min_cost = None;
        for (a_x, b_x) in x_combinations {
            for (a_y, b_y) in &y_combinations {
                if a_x == *a_y && b_x == *b_y {
                    let total_cost = a_x * game.button_a.cost + b_x * game.button_b.cost;
                    min_cost = Some(min_cost.map_or(total_cost, |c:usize| c.min(total_cost)));
                }
            }
        }
        min_cost
    }

    fn load_games(path: &str) -> Vec<Game> {
        let lines = read_file_to_string_array(path).unwrap();
        let games: Vec<Game> = lines.chunks(4).map(|chunk| {
            let button_a = chunk[0].split_whitespace().skip(2).collect::<Vec<&str>>();
            let button_b = chunk[1].split_whitespace().skip(2).collect::<Vec<&str>>();
            let prize = chunk[2].split_whitespace().skip(1).collect::<Vec<&str>>();

            let parse_digits = |s: &str| -> isize {
                s.chars().filter(|c| c.is_digit(10)).collect::<String>().parse().unwrap()
            };
            Game {
                button_a: State {
                    cost: 3,
                    x: parse_digits(button_a[0]),
                    y: parse_digits(button_a[1]),
                },
                button_b: State {
                    cost: 1,
                    x: parse_digits(button_b[0]),
                    y: parse_digits(button_b[1]),
                },
                prize: (
                    parse_digits(prize[0]),
                    parse_digits(prize[1]),
                ),
            }
        }).collect();
        games
    }

    fn compute_cost_to_win(game: Game) -> usize {
        // Figure out simultaneous equations, substitute and solve for A button presses
        let top = game.prize.0 * game.button_b.y - game.prize.1 * game.button_b.x;
        let botton = game.button_a.x * game.button_b.y - game.button_a.y * game.button_b.x;

        // If top is evenly divisible by bottom then we have a solution, get the i64 value of top/bottom, the is button_a presses. Then compute button b presses and the cost, otherwise return 0;
        if top % botton == 0 {
            let a_presses = top / botton;
            let b_presses = (game.prize.0 - a_presses * game.button_a.x) / game.button_b.x;
            (a_presses as usize * game.button_a.cost + b_presses as usize * game.button_b.cost) as usize
        } else {
            0
        }
    }

    #[test]
    fn day13_part1(){
        let games = load_games("src/day13_part1.data");
        let mut result: usize = 0;
        let mut result2: usize = 0;
        for game in games {
            result2 += compute_cost_to_win(game.clone());
            if let Some(cost) = minimum_cost_to_win(game) {
                result += cost;
            }
        }
        println!("Result 1, {:?} - Result 2, {:?}", result, result2);
    }

    #[test]
    fn day13_part2(){
        let games = load_games("src/day13_part1.data");
        let mut result: usize = 0;
        for game in games {
            let mut updated_game = game.clone();
            updated_game.prize = (updated_game.prize.0 + 10000000000000, updated_game.prize.1 + 10000000000000);

            result += compute_cost_to_win(updated_game);
        }
        println!("{:?}", result);

    }
}