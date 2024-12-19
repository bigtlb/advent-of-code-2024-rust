#[cfg(test)]
mod test {
    use std::collections::{HashMap};
    use crate::util::read_file_to_string_array;

    fn make_combo(v: &usize, a: usize, b: usize, c: usize) -> usize {
        match v {
            0..=3 => *v,
            4 => a,
            5 => b,
            6 => c,
            _ => panic!("Invalid value: {}", v),
        }
    }

    fn execute_instruction(opcode: usize, operand: &mut usize, a: &mut usize, b: &mut usize, c: &mut usize, ip: &mut usize, output: &mut Vec<usize>, debug: bool) {
        let mut inc_ip = true;
        match opcode {
            0 => {
                let numerator = *a;
                let denominator = 2_usize.pow(make_combo(operand, *a, *b, *c) as u32);
                *a = numerator / denominator;
                if debug {
                    println!("({})ADV - numerator: {}, operand: {}, denominator: {}, result: {}", ip, numerator, operand, denominator, numerator / denominator);
                }
            }
            1 => {
                let result = *b ^ *operand % 8;
                *b = result;
                if debug {
                    println!("({})BXL - operand: {}, result: {}", ip, operand, result);
                }
            }
            2 => {
                let result = make_combo(operand, *a, *b, *c) % 8;
                *b = result;
                if debug {
                    println!("({})BST - operand: {}, result: {}", ip, operand, result);
                }
            }
            3 => {
                if *a != 0 {
                    let org_ip = *ip;
                    *ip = *operand;
                    inc_ip = false;
                    if debug {
                        println!("({})JNZ - A: {}, operand: {}, ip: {}", org_ip, a, operand, ip);
                    }
                }
            }
            4 => {
                let result = *b ^ *c;
                *b = result;
                if debug {
                    println!("({})BXC - B: {} C: {} result: {}", ip, b, c, result);
                }
            }
            5 => {
                let value = make_combo(operand, *a, *b, *c) % 8;
                output.push(value);
                if debug {
                    println!("({})OUT - operand: {}, value: {}", ip, operand, value);
                }
            }
            6 => {
                let numerator = *a;
                let denominator = 2_usize.pow(make_combo(operand, *a, *b, *c) as u32);
                *b = numerator / denominator;
                if debug {
                    println!("({})BDV - numerator: {}, operand: {}, denominator: {}, result: {}", ip, numerator, operand, denominator, numerator / denominator);
                }
            }
            7 => {
                let numerator = *a;
                let denominator = 2_usize.pow(make_combo(operand, *a, *b, *c) as u32);
                *c = numerator / denominator;
                if debug {
                    println!("({})CDV - numerator: {}, operand: {}, denominator: {}, result: {}", ip, numerator, operand, denominator, numerator / denominator);
                }
            }
            _ => {
                println!("({})Unknown opcode: {}", ip, opcode);
            }
        }
        if inc_ip {
            *ip += 2;
        }
    }

    fn execute_program(a: usize, b: usize, c: usize, program: &Vec<usize>, debug: bool) -> (usize, usize, usize, Vec<usize>) {
        let mut a = a;
        let mut b = b;
        let mut c = c;
        let mut output: Vec<usize> = Vec::new();
        let mut ip: usize = 0;
        loop {
            let opcode = program[ip];
            let mut operand = program[ip + 1];
            execute_instruction(opcode, &mut operand, &mut a, &mut b, &mut c, &mut ip, &mut output, debug);
            if ip >= program.len() {
                break;
            }
        }
        (a, b, c, output)
    }

    // The op codes in the program constrain things to mod 8. So You would only need a valu between 0 and 7 to produce an output digit
    // Because make_combo returns 0, 1, 2 and 3, and it is used with a 2.pow(n) denominator,
    // the result will always be 0, 1, 2, 3, 4, 5, 6, 7 (or values in registers that are also mod 8)
    fn try_all_octal_values(a: usize, b: usize, c: usize, program: &Vec<usize>, pos: isize, digits: &mut Vec<usize>)-> bool{
        if pos < 0 {
            return true;
        }
        let checkdigit = program[pos as usize];
        for i in 0..8 {
            let output = execute_program(a + i, b, c, program, false).3;
            let checkoutput = output[0];
            if checkoutput == checkdigit {
                digits.push(i);
                let new_a = a + i << 3;
                if try_all_octal_values(new_a, b, c, program, pos-1, digits) {
                    return true;
                }
                digits.pop();
            }
        }
        false
    }

    fn load_state(path: &str) -> (HashMap<char, usize>, Vec<usize>) {
        let mut registers: HashMap<char, usize> = HashMap::new();
        let mut program: Vec<usize> = Vec::new();
        read_file_to_string_array(path).unwrap().iter().for_each(|line| {
            if line.starts_with("Register") {
                let parts: Vec<&str> = line.split(" ").collect();
                let register = parts[1].chars().next().unwrap();
                let value = parts[2].parse::<usize>().unwrap();
                registers.insert(register, value);
            } else if line.starts_with("Program") {
                let parts: Vec<&str> = line.split(|c| c == ' ' || c == ',').collect();
                parts.iter().skip(1).for_each(|part| {
                    program.push(part.parse::<usize>().unwrap());
                });
            }
        });
        (registers, program)
    }
    #[test]
    fn day17_part1() {
        let (registers, program) = load_state("src/day17_part1.data");
        println!("Registers: {:?}, program: {:?}", registers, program);
        let (a, b, c, output) = execute_program(registers[&'A'], registers[&'B'], registers[&'C'], &program, true);
        println!("New Registers: {}, {}, {}, output: {:?}", a, b, c, output.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(","));
    }


    #[test]
    fn day17_part2() {
        let (mut registers, program) = load_state("src/day17_part1.data");
        println!("Registers: {:?}, program: {:?}", registers, program);
        registers.insert('A',0);
        let mut digits: Vec<usize> = Vec::new();
        if try_all_octal_values(0, registers[&'B'], registers[&'C'], &program, (program.len()-1) as isize, &mut digits) {
            println!("Digits: {}", usize::from_str_radix(&digits.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(""),8).expect("InvalidOctal number"));
        } else {
            println!("No solution found");
        }

    }
}