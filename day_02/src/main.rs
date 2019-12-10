use std::fs;

fn main() {
    let input: Vec<usize> = fs::read_to_string("input.txt").unwrap()
        .split(",")
        .map(|slice| slice.parse::<usize>().unwrap())
        .collect();
    println!("The result of part one is {}.", part_one(input.clone()));
    println!("The result of part two is {}.", part_two(input));
}

fn part_one(input: Vec<usize>) -> usize {
    run_program(input, 12, 2)
}

fn part_two(input: Vec<usize>) -> usize {
    for noun in 1..=99 {
        for verb in 1..=99 {
            let output = run_program(input.clone(), noun, verb);
            if output == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    panic!("Could not find the correct solution.");
}

fn run_program(mut input: Vec<usize>, noun: usize, verb: usize) -> usize {
    input[1] = noun;
    input[2] = verb;
    let mut i = 0;
    loop {
        let opcode = input[i];
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
            99 => {
                break;
            }
            _ => panic!("Encountering an unknown opcode means something went wrong.")
        }
        i += 4;
    }
    input[0]
}
