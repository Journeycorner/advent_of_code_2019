fn main() {
    let input = String::from("125730-579381");
    let mut input = input.split('-');
    let min: u32 = input.next().unwrap().parse().unwrap();
    let max: u32 = input.next().unwrap().parse().unwrap();

    println!("The result of part one is {}.", part_one(min, max));
    println!("The result of part two is {}.", part_two(min, max));
}

fn part_one(min: u32, max: u32) -> i32 {
    let mut possible_results = 0;
    for number in min..=max {
        if check_constraints_one(number) {
            possible_results += 1;
        }
    }
    possible_results
}

fn part_two(min: u32, max: u32) -> i32 {
    let mut possible_results = 0;
    for number in min..=max {
        if check_constraints_two(number) {
            possible_results += 1;
        }
    }
    possible_results
}

fn check_constraints_one(password: u32) -> bool {
    let number_vec = number_to_vec(password);
    let mut last_digit: Option<u32> = Option::None;
    let mut double = false;
    for i in 0..number_vec.len() {
        let current_digit = number_vec[i];
        if let Some(x) = last_digit {
            if current_digit == x {
                double = true;
            } else if current_digit < x {
                // digits must not decrease, when read from left to right
                return false;
            }
        }
        last_digit = Option::from(number_vec[i]);
    }
    // min okay, max okay, ascending number okay: so double or not matters
    double
}

fn check_constraints_two(password: u32) -> bool {
    let number_vec = number_to_vec(password);
    let mut last_digit: Option<u32> = Option::None;
    for i in 0..number_vec.len() {
        let current_digit = number_vec[i];
        if let Some(x) = last_digit {
            if current_digit < x {
                // digits must not decrease, when read from left to right
                return false;
            }
        }
        last_digit = Option::from(number_vec[i]);
    }
    // min okay, max okay, ascending number okay: so double or not matters
    has_double_adjacent(&number_vec)
}

fn number_to_vec(n: u32) -> Vec<u32> {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

fn has_double_adjacent(numbers: &Vec<u32>) -> bool {
    let mut count = 1;
    let mut i = 0;
    loop {
        if i >= numbers.len() {
            return count == 2;
        }
        let current_digit = numbers[i];
        if let Some(next) = numbers.get(i + 1) {
            if current_digit == *next {
                count += 1;
            } else if count == 2 {
                return true;
            } else {
                count = 1;
            }
        }
        i += 1;
    }
}