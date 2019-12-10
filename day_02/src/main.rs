use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    println!("The result of part one is {}.", part_one(&input));
}

fn part_one(input: &str) -> i32 {
    // parse input to numbers
    let mut input: Vec<i32> = input.split(",")
        .map(|slice| slice.parse::<i32>().unwrap())
        .collect();
    // sanitize input
    input[1] = 12;
    input[2] = 2;
    // process data
    let mut i = 0;
    loop {
        let opcode = input[i];
        match opcode {
            1 | 2 => {
                let target_pos = input[i + 3] as usize;
                let input_a = input[input[i + 1] as usize];
                let input_b = input[input[i + 2] as usize];
                if opcode == 1 {
                    input[target_pos] = input_a + input_b;
                } else {
                    input[target_pos] = input_a * input_b;
                }
            },
            99 => {
                break;
            },
            _ => panic!("Encountering an unknown opcode means something went wrong.")
        }
        i += 4;
    }
    input[0]
}
