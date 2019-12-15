use std::fs;

fn main() {
    println!("{:?}", parse_parameter_modes(1002));
    println!("{:?}", parse_parameter_modes(1102));
//    let program: Vec<usize> = fs::read_to_string("input.txt").unwrap()
//        .split(",")
//        .map(|slice| slice.parse::<usize>().unwrap())
//        .collect();
//    println!("The result of part one is {}.", part_one(program.clone()));
//    println!("The result of part two is {}.", part_two(program));
}

fn part_one(input: Vec<usize>) -> usize {
    run_program(input)
}

fn run_program(mut input: Vec<usize>) -> usize {
    let mut i = 0;
    loop {
        let opcode = input[i];
        println!("{:?}", parse_parameter_modes(opcode));
        return 1;
        match opcode {
            1 | 2 => {
                let target_pos = input[i + 3];
                let input_a = input[input[i + 1]];
                let input_b = input[input[i + 2]];
                if opcode == 1 {
                    input[target_pos] = input_a + input_b;
                } else {
                    input[target_pos] = input_a * input_b;
                }
            }
            3 => {
                let target_pos = input[i + 1];
//                TODO input??
                input[target_pos] = 0;
            },
            4 => {
                let target_pos = input[i + 1];
                println!("{}", input[target_pos]);
            },
            99 => {
                break;
            }
            _ => panic!("Encountering an unknown opcode means something went wrong.")
        }
        i += 4;
    }
    input[0]
}

fn parse_parameter_modes(opcode: usize) -> Vec<usize> {
    let mut numbers = number_to_vec(opcode);
    loop {
        if numbers.len() >= 5 {
            break;
        }
        numbers.insert(0, 0);
    }
    numbers.iter()
        .take(4)
        .collect()
}

fn number_to_vec(n: usize) -> Vec<usize> {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}